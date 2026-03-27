use async_trait::async_trait;
use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry};

use crate::config::LdapConfig;
use crate::errors::AuthError;

use super::{AuthProvider, AuthenticatedUser, ProviderInfo};

#[derive(Clone)]
pub struct LdapProvider {
    config: LdapConfig,
}

impl LdapProvider {
    pub fn new(config: LdapConfig) -> Self {
        Self { config }
    }

    pub fn enabled(&self) -> bool {
        self.config.enabled
    }

    /// Authenticate a user via LDAP simple bind.
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthenticatedUser, AuthError> {
        if !self.config.enabled {
            return Err(AuthError::ProviderNotConfigured);
        }

        let bind_dn = self
            .config
            .bind_dn_template
            .replace("{username}", username);

        let settings = LdapConnSettings::new();
        let (conn, mut ldap) = LdapConnAsync::with_settings(settings, &self.config.url)
            .await
            .map_err(|e| AuthError::Ldap(format!("LDAP connection failed: {e}")))?;

        // Drive the connection in the background
        ldap3::drive!(conn);

        // Attempt bind with user credentials
        let result = ldap
            .simple_bind(&bind_dn, password)
            .await
            .map_err(|e| AuthError::Ldap(format!("LDAP bind failed: {e}")))?;

        if result.rc != 0 {
            let _ = ldap.unbind().await;
            return Err(AuthError::InvalidCredentials);
        }

        // Search for user attributes
        let filter = self
            .config
            .user_filter
            .replace("{username}", username);

        let attrs = vec![
            self.config.attr_email.as_str(),
            self.config.attr_name.as_str(),
            self.config.attr_username.as_str(),
        ];

        let (results, _) = ldap
            .search(&self.config.search_base, Scope::Subtree, &filter, attrs)
            .await
            .map_err(|e| AuthError::Ldap(format!("LDAP search failed: {e}")))?
            .success()
            .map_err(|e| AuthError::Ldap(format!("LDAP search error: {e}")))?;

        let _ = ldap.unbind().await;

        let entry = results
            .into_iter()
            .next()
            .map(SearchEntry::construct)
            .ok_or_else(|| AuthError::Ldap("User not found in LDAP".to_string()))?;

        let email = entry
            .attrs
            .get(&self.config.attr_email)
            .and_then(|v| v.first())
            .cloned()
            .unwrap_or_default();

        let display_name = entry
            .attrs
            .get(&self.config.attr_name)
            .and_then(|v| v.first())
            .cloned();

        let ldap_username = entry
            .attrs
            .get(&self.config.attr_username)
            .and_then(|v| v.first())
            .cloned();

        if email.is_empty() {
            return Err(AuthError::Ldap("Email attribute not found in LDAP entry".to_string()));
        }

        Ok(AuthenticatedUser {
            email,
            username: ldap_username,
            display_name,
            external_id: Some(entry.dn),
            avatar_url: None,
            provider: "ldap".to_string(),
        })
    }
}

#[async_trait]
impl AuthProvider for LdapProvider {
    fn provider_type(&self) -> &str {
        "ldap"
    }

    fn name(&self) -> &str {
        "LDAP"
    }

    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "LDAP".to_string(),
            provider_type: "ldap".to_string(),
            enabled: self.config.enabled,
        }
    }
}

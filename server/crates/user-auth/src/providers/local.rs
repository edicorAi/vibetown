use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::config::LocalAuthConfig;
use crate::errors::AuthError;
use crate::models;
use crate::password;

use super::{AuthProvider, ProviderInfo};

#[derive(Clone)]
pub struct LocalProvider {
    config: LocalAuthConfig,
}

impl LocalProvider {
    pub fn new(config: LocalAuthConfig) -> Self {
        Self { config }
    }

    pub fn enabled(&self) -> bool {
        self.config.enabled
    }

    /// Authenticate a user with email and password.
    pub async fn authenticate(
        &self,
        pool: &SqlitePool,
        email: &str,
        password_input: &str,
    ) -> Result<models::User, AuthError> {
        if !self.config.enabled {
            return Err(AuthError::ProviderNotConfigured);
        }

        let user_row = models::get_user_by_email(pool, email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        if user_row.is_active == 0 {
            return Err(AuthError::AccountDisabled);
        }

        let hash = user_row
            .password_hash
            .as_deref()
            .ok_or(AuthError::InvalidCredentials)?;

        if !password::verify_password(password_input, hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(models::User::from(user_row))
    }
}

#[async_trait]
impl AuthProvider for LocalProvider {
    fn provider_type(&self) -> &str {
        "local"
    }

    fn name(&self) -> &str {
        "Local"
    }

    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "Local".to_string(),
            provider_type: "local".to_string(),
            enabled: self.config.enabled,
        }
    }
}

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::config::OidcProviderConfig;
use crate::errors::AuthError;

use super::{AuthProvider, AuthenticatedUser, ProviderInfo};

/// OIDC Discovery document.
#[derive(Debug, Clone, Deserialize)]
struct OidcDiscovery {
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: Option<String>,
}

/// Token response from the OIDC token endpoint.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
    token_type: String,
}

/// UserInfo response.
#[derive(Debug, Deserialize)]
struct UserInfo {
    sub: String,
    email: Option<String>,
    name: Option<String>,
    preferred_username: Option<String>,
    picture: Option<String>,
}

/// Pending OIDC auth state between redirect and callback.
#[derive(Debug)]
struct PendingOidcAuth {
    code_verifier: String,
}

#[derive(Clone)]
pub struct OidcProvider {
    config: OidcProviderConfig,
    discovery: OidcDiscovery,
    redirect_url: String,
    http: reqwest::Client,
    pending: Arc<RwLock<HashMap<String, PendingOidcAuth>>>,
}

impl OidcProvider {
    pub async fn new(
        config: OidcProviderConfig,
        redirect_url: &str,
    ) -> Result<Self, AuthError> {
        let http = reqwest::Client::new();

        // Discover endpoints
        let discovery_url = format!(
            "{}/.well-known/openid-configuration",
            config.issuer_url.trim_end_matches('/')
        );
        let discovery: OidcDiscovery = http
            .get(&discovery_url)
            .send()
            .await
            .map_err(|e| AuthError::Oidc(format!("Discovery request failed: {e}")))?
            .json()
            .await
            .map_err(|e| AuthError::Oidc(format!("Discovery parse failed: {e}")))?;

        Ok(Self {
            config,
            discovery,
            redirect_url: redirect_url.to_string(),
            http,
            pending: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Generate the authorization URL for the OIDC flow.
    pub async fn authorization_url(&self) -> Result<(String, String), AuthError> {
        use sha2::{Digest, Sha256};

        // Generate PKCE code verifier + challenge
        let mut verifier_bytes = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut verifier_bytes);
        let code_verifier = base64url_encode(&verifier_bytes);

        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let code_challenge = base64url_encode(&hasher.finalize());

        // Generate state (CSRF token)
        let mut state_bytes = [0u8; 16];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut state_bytes);
        let state = base64url_encode(&state_bytes);

        // Build scopes
        let scopes = self.config.scopes.join(" ");

        let auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            self.discovery.authorization_endpoint,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.redirect_url),
            urlencoding::encode(&scopes),
            urlencoding::encode(&state),
            urlencoding::encode(&code_challenge),
        );

        self.pending.write().await.insert(
            state.clone(),
            PendingOidcAuth { code_verifier },
        );

        Ok((auth_url, state))
    }

    /// Exchange the authorization code for tokens and extract user info.
    pub async fn exchange_code(
        &self,
        code: &str,
        state: &str,
    ) -> Result<AuthenticatedUser, AuthError> {
        let pending = self
            .pending
            .write()
            .await
            .remove(state)
            .ok_or_else(|| AuthError::Oidc("Invalid or expired OIDC state".to_string()))?;

        // Exchange code for tokens (application/x-www-form-urlencoded)
        let body = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}&code_verifier={}",
            urlencoding::encode(code),
            urlencoding::encode(&self.redirect_url),
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.client_secret),
            urlencoding::encode(&pending.code_verifier),
        );
        let token_response: TokenResponse = self
            .http
            .post(&self.discovery.token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .map_err(|e| AuthError::Oidc(format!("Token exchange failed: {e}")))?
            .json()
            .await
            .map_err(|e| AuthError::Oidc(format!("Token parse failed: {e}")))?;

        // Get user info from userinfo endpoint
        let userinfo_url = self
            .discovery
            .userinfo_endpoint
            .as_deref()
            .ok_or_else(|| AuthError::Oidc("No userinfo endpoint".to_string()))?;

        let user_info: UserInfo = self
            .http
            .get(userinfo_url)
            .bearer_auth(&token_response.access_token)
            .send()
            .await
            .map_err(|e| AuthError::Oidc(format!("UserInfo request failed: {e}")))?
            .json()
            .await
            .map_err(|e| AuthError::Oidc(format!("UserInfo parse failed: {e}")))?;

        let email = user_info
            .email
            .ok_or_else(|| AuthError::Oidc("Email not provided by OIDC provider".to_string()))?;

        Ok(AuthenticatedUser {
            email,
            username: user_info.preferred_username,
            display_name: user_info.name,
            external_id: Some(user_info.sub),
            avatar_url: user_info.picture,
            provider: "oidc".to_string(),
        })
    }

    pub fn provider_name(&self) -> &str {
        &self.config.name
    }
}

#[async_trait]
impl AuthProvider for OidcProvider {
    fn provider_type(&self) -> &str {
        "oidc"
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: self.config.name.clone(),
            provider_type: "oidc".to_string(),
            enabled: true,
        }
    }
}

/// Base64url encode without padding.
fn base64url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub mod ldap;
pub mod local;
pub mod oidc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Credentials submitted by the user for authentication.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AuthCredentials {
    Password {
        email: String,
        password: String,
    },
    LdapBind {
        username: String,
        password: String,
    },
}

/// Information about a successfully authenticated user from a provider.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub external_id: Option<String>,
    pub avatar_url: Option<String>,
    pub provider: String,
}

/// Info about an auth provider exposed to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct ProviderInfo {
    pub name: String,
    pub provider_type: String,
    pub enabled: bool,
}

/// Trait implemented by each authentication provider.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    fn provider_type(&self) -> &str;
    fn name(&self) -> &str;
    fn info(&self) -> ProviderInfo;
}

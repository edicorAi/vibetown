use serde::{Deserialize, Serialize};

/// Authentication configuration parsed from environment variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub mode: AuthMode,
    pub session_ttl: i64,
    pub local: LocalAuthConfig,
    pub oidc_providers: Vec<OidcProviderConfig>,
    pub ldap: LdapConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthMode {
    None,
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAuthConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcProviderConfig {
    pub name: String,
    pub issuer_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapConfig {
    pub enabled: bool,
    pub url: String,
    pub bind_dn_template: String,
    pub search_base: String,
    pub user_filter: String,
    pub use_tls: bool,
    pub use_starttls: bool,
    pub attr_email: String,
    pub attr_name: String,
    pub attr_username: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            mode: AuthMode::None,
            session_ttl: 604800, // 7 days
            local: LocalAuthConfig { enabled: true },
            oidc_providers: Vec::new(),
            ldap: LdapConfig::default(),
        }
    }
}

impl Default for LdapConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: String::new(),
            bind_dn_template: String::new(),
            search_base: String::new(),
            user_filter: "(uid={username})".to_string(),
            use_tls: false,
            use_starttls: false,
            attr_email: "mail".to_string(),
            attr_name: "cn".to_string(),
            attr_username: "uid".to_string(),
        }
    }
}

impl AuthConfig {
    /// Parse auth configuration from environment variables.
    pub fn from_env() -> Self {
        let mode = match std::env::var("VT_AUTH_MODE").as_deref() {
            Ok("required") => AuthMode::Required,
            _ => AuthMode::None,
        };

        let session_ttl = std::env::var("VT_SESSION_TTL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(604800);

        let local = LocalAuthConfig {
            enabled: std::env::var("VT_AUTH_LOCAL_ENABLED")
                .map(|v| v != "false" && v != "0")
                .unwrap_or(true),
        };

        let oidc_providers = parse_oidc_providers();
        let ldap = parse_ldap_config();

        Self {
            mode,
            session_ttl,
            local,
            oidc_providers,
            ldap,
        }
    }

    pub fn auth_required(&self) -> bool {
        self.mode == AuthMode::Required
    }
}

fn parse_oidc_providers() -> Vec<OidcProviderConfig> {
    let mut providers = Vec::new();
    for i in 1..=10 {
        let prefix = format!("VT_AUTH_OIDC_{i}_");
        let name = match std::env::var(format!("{prefix}NAME")) {
            Ok(v) => v,
            Err(_) => break,
        };
        let issuer_url = match std::env::var(format!("{prefix}ISSUER_URL")) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let client_id = match std::env::var(format!("{prefix}CLIENT_ID")) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let client_secret = std::env::var(format!("{prefix}CLIENT_SECRET")).unwrap_or_default();
        let scopes = std::env::var(format!("{prefix}SCOPES"))
            .unwrap_or_else(|_| "openid,email,profile".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        providers.push(OidcProviderConfig {
            name,
            issuer_url,
            client_id,
            client_secret,
            scopes,
        });
    }
    providers
}

fn parse_ldap_config() -> LdapConfig {
    let enabled = std::env::var("VT_AUTH_LDAP_ENABLED")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    if !enabled {
        return LdapConfig {
            enabled: false,
            ..Default::default()
        };
    }

    LdapConfig {
        enabled: true,
        url: std::env::var("VT_AUTH_LDAP_URL").unwrap_or_default(),
        bind_dn_template: std::env::var("VT_AUTH_LDAP_BIND_DN_TEMPLATE").unwrap_or_default(),
        search_base: std::env::var("VT_AUTH_LDAP_SEARCH_BASE").unwrap_or_default(),
        user_filter: std::env::var("VT_AUTH_LDAP_USER_FILTER")
            .unwrap_or_else(|_| "(uid={username})".to_string()),
        use_tls: std::env::var("VT_AUTH_LDAP_TLS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false),
        use_starttls: std::env::var("VT_AUTH_LDAP_STARTTLS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false),
        attr_email: std::env::var("VT_AUTH_LDAP_ATTR_EMAIL")
            .unwrap_or_else(|_| "mail".to_string()),
        attr_name: std::env::var("VT_AUTH_LDAP_ATTR_NAME")
            .unwrap_or_else(|_| "cn".to_string()),
        attr_username: std::env::var("VT_AUTH_LDAP_ATTR_USERNAME")
            .unwrap_or_else(|_| "uid".to_string()),
    }
}

pub mod config;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod password;
pub mod providers;
pub mod session;

use sqlx::SqlitePool;

use config::AuthConfig;
use errors::AuthError;
use models::{CreateUser, User};
use providers::ldap::LdapProvider;
use providers::local::LocalProvider;
use providers::oidc::OidcProvider;
use providers::{AuthProvider, ProviderInfo};

/// Central authentication service that coordinates providers and sessions.
#[derive(Clone)]
pub struct UserAuthService {
    config: AuthConfig,
    pool: SqlitePool,
    local_provider: LocalProvider,
    oidc_providers: Vec<OidcProvider>,
    ldap_provider: LdapProvider,
}

impl UserAuthService {
    /// Create a new UserAuthService from config and database pool.
    /// OIDC providers are initialized lazily since discovery is async.
    pub fn new(config: AuthConfig, pool: SqlitePool) -> Self {
        let local_provider = LocalProvider::new(config.local.clone());
        let ldap_provider = LdapProvider::new(config.ldap.clone());

        Self {
            config,
            pool,
            local_provider,
            oidc_providers: Vec::new(),
            ldap_provider,
        }
    }

    /// Initialize OIDC providers (requires async for discovery).
    pub async fn init_oidc_providers(&mut self, base_url: &str) -> Result<(), AuthError> {
        for oidc_config in &self.config.oidc_providers {
            let redirect_url = format!(
                "{}/api/auth/oidc/{}/callback",
                base_url.trim_end_matches('/'),
                urlencoding::encode(&oidc_config.name)
            );
            match OidcProvider::new(oidc_config.clone(), &redirect_url).await {
                Ok(provider) => {
                    tracing::info!("OIDC provider '{}' initialized", oidc_config.name);
                    self.oidc_providers.push(provider);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to initialize OIDC provider '{}': {}",
                        oidc_config.name,
                        e
                    );
                }
            }
        }
        Ok(())
    }

    pub fn auth_required(&self) -> bool {
        self.config.auth_required()
    }

    pub fn config(&self) -> &AuthConfig {
        &self.config
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    // ── Authentication ──

    /// Authenticate with email and password (local provider).
    pub async fn login_local(
        &self,
        email: &str,
        password: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, String), AuthError> {
        let user = self.local_provider.authenticate(&self.pool, email, password).await?;
        models::update_last_login(&self.pool, &user.id).await?;
        let (token, _session) = session::create_session(
            &self.pool,
            &user.id,
            self.config.session_ttl,
            ip_address,
            user_agent,
        )
        .await?;
        Ok((user, token))
    }

    /// Authenticate via LDAP.
    pub async fn login_ldap(
        &self,
        username: &str,
        password: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, String), AuthError> {
        let auth_user = self.ldap_provider.authenticate(username, password).await?;
        let user = self.find_or_create_external_user(&auth_user).await?;
        models::update_last_login(&self.pool, &user.id).await?;
        let (token, _session) = session::create_session(
            &self.pool,
            &user.id,
            self.config.session_ttl,
            ip_address,
            user_agent,
        )
        .await?;
        Ok((user, token))
    }

    /// Get OIDC authorization URL for a provider.
    pub async fn oidc_authorization_url(
        &self,
        provider_name: &str,
    ) -> Result<(String, String), AuthError> {
        let provider = self
            .oidc_providers
            .iter()
            .find(|p| p.provider_name() == provider_name)
            .ok_or(AuthError::ProviderNotConfigured)?;
        provider.authorization_url().await
    }

    /// Complete OIDC flow by exchanging code for user info.
    pub async fn oidc_callback(
        &self,
        provider_name: &str,
        code: &str,
        state: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, String), AuthError> {
        let provider = self
            .oidc_providers
            .iter()
            .find(|p| p.provider_name() == provider_name)
            .ok_or(AuthError::ProviderNotConfigured)?;

        let auth_user = provider.exchange_code(code, state).await?;
        let user = self.find_or_create_external_user(&auth_user).await?;
        models::update_last_login(&self.pool, &user.id).await?;
        let (token, _session) = session::create_session(
            &self.pool,
            &user.id,
            self.config.session_ttl,
            ip_address,
            user_agent,
        )
        .await?;
        Ok((user, token))
    }

    // ── Session ──

    /// Validate a session token and return the associated user.
    pub async fn validate_session(&self, token: &str) -> Result<User, AuthError> {
        let sess = session::validate_session(&self.pool, token).await?;
        let user = models::get_user_by_id(&self.pool, &sess.user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        if !user.is_active {
            return Err(AuthError::AccountDisabled);
        }
        Ok(user)
    }

    /// Logout by revoking the session.
    pub async fn logout(&self, token: &str) -> Result<(), AuthError> {
        session::revoke_session_by_token(&self.pool, token).await
    }

    // ── User Management (admin) ──

    /// Create a new local user (admin action).
    pub async fn create_user(&self, data: CreateUser) -> Result<User, AuthError> {
        models::create_user(&self.pool, &data).await
    }

    /// List all users (admin action).
    pub async fn list_users(&self) -> Result<Vec<User>, AuthError> {
        models::list_users(&self.pool).await
    }

    /// Get a user by ID.
    pub async fn get_user(&self, id: &str) -> Result<Option<User>, AuthError> {
        models::get_user_by_id(&self.pool, id).await
    }

    /// Deactivate a user and revoke all their sessions.
    pub async fn deactivate_user(&self, user_id: &str) -> Result<(), AuthError> {
        models::update_user_active(&self.pool, user_id, false).await?;
        session::revoke_all_user_sessions(&self.pool, user_id).await
    }

    /// Activate a user.
    pub async fn activate_user(&self, user_id: &str) -> Result<(), AuthError> {
        models::update_user_active(&self.pool, user_id, true).await
    }

    /// Update admin status.
    pub async fn set_user_admin(&self, user_id: &str, is_admin: bool) -> Result<(), AuthError> {
        models::update_user_admin(&self.pool, user_id, is_admin).await
    }

    /// Reset a user's password (admin action).
    pub async fn reset_user_password(
        &self,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        models::update_user_password(&self.pool, user_id, new_password).await
    }

    /// Check if any users exist (for first-run setup).
    pub async fn has_users(&self) -> Result<bool, AuthError> {
        Ok(models::user_count(&self.pool).await? > 0)
    }

    // ── Providers ──

    /// List all configured auth providers.
    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        let mut providers = Vec::new();
        if self.local_provider.enabled() {
            providers.push(self.local_provider.info());
        }
        for p in &self.oidc_providers {
            providers.push(p.info());
        }
        if self.ldap_provider.enabled() {
            providers.push(self.ldap_provider.info());
        }
        providers
    }

    // ── Internal ──

    /// Find an existing user by external ID or email, or create a new one (JIT provisioning).
    async fn find_or_create_external_user(
        &self,
        auth_user: &providers::AuthenticatedUser,
    ) -> Result<User, AuthError> {
        // Try by external ID first
        if let Some(ext_id) = &auth_user.external_id {
            if let Some(row) =
                models::get_user_by_external_id(&self.pool, &auth_user.provider, ext_id).await?
            {
                let user = User::from(row);
                if !user.is_active {
                    return Err(AuthError::AccountDisabled);
                }
                return Ok(user);
            }
        }

        // Try by email
        if let Some(row) = models::get_user_by_email(&self.pool, &auth_user.email).await? {
            let user = User::from(row);
            if !user.is_active {
                return Err(AuthError::AccountDisabled);
            }
            return Ok(user);
        }

        // JIT provisioning: create new user
        let data = CreateUser {
            email: auth_user.email.clone(),
            username: auth_user.username.clone(),
            display_name: auth_user.display_name.clone(),
            password: None,
            auth_provider: auth_user.provider.clone(),
            external_id: auth_user.external_id.clone(),
            avatar_url: auth_user.avatar_url.clone(),
            is_admin: false,
        };
        models::create_user(&self.pool, &data).await
    }
}

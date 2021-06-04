use crate::models::auth::HashAndSalt;
use argon2::{Config, ThreadMode};
use color_eyre::{Report, Result};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {
    pub fn new(secret_key: String) -> Self {
        CryptoService { key: Arc::new(secret_key) }
    }

    #[instrument(skip(self, password))]
    pub fn hash_password(&self, password: &str, maybe_salt: Option<String>) -> Result<HashAndSalt> {
        let mut cfg = Config::default();
        cfg.secret = self.key.as_bytes();
        cfg.thread_mode = ThreadMode::Parallel;

        let salt: String = maybe_salt.unwrap_or(rand::thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect());

        let maybe_hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &cfg);

        maybe_hash
            .map(|password_hash| HashAndSalt { password_hash, salt })
            .map_err(|e| Report::new(e))
    }
}

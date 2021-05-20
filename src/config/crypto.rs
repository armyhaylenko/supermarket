use argonautica::Hasher;
use color_eyre::Result;
use futures::compat::Future01CompatExt;
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
    pub async fn hash_password(&self, password: &str) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|e| eyre::Report::msg(format!("Hashing error: {:?}", e)))
    }
}

use crate::config::crypto::CryptoService;
use crate::models::{NewUser, User};
use color_eyre::Result;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: NewUser, crypto_svc: &CryptoService) -> Result<User> {
        let pwd_hash = crypto_svc.hash_password(&user.password[..]).await?;

        let add_user_sql = include_str!("../../sql/add_user.sql");

        let user_in_db: User = sqlx::query_as::<_, User>(add_user_sql)
            .bind(&user.username)
            .bind(&user.email)
            .bind(pwd_hash)
            .bind(&user.user_role)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user_in_db)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let get_user_sql = include_str!("../../sql/get_user.sql");

        let maybe_user: Option<User> = sqlx::query_as(get_user_sql).bind(&username).fetch_optional(&*self.pool).await?;

        Ok(maybe_user)
    }
}

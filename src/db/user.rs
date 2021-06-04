use crate::config::crypto::CryptoService;
use crate::models::auth::{AuthenticatedUser, HashAndSalt, LoginUser, NewUser, User};
use color_eyre::{Report, Result};
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
        let hash_and_salt = crypto_svc.hash_password(&user.password[..], None)?;

        let add_user_sql = include_str!("../../sql/add_user.sql");

        let user_in_db: User = sqlx::query_as::<_, User>(add_user_sql)
            .bind(&user.username)
            .bind(&user.email)
            .bind(hash_and_salt.password_hash)
            .bind(&user.user_role)
            .bind(hash_and_salt.salt)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user_in_db)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let get_user_sql = include_str!("../../sql/get_user.sql");

        let maybe_user: Option<User> = sqlx::query_as(get_user_sql).bind(&username).fetch_optional(&*self.pool).await?;

        Ok(maybe_user)
    }

    pub async fn login(&self, user: &LoginUser, crypto_svc: &CryptoService) -> Result<AuthenticatedUser> {
        let sql = include_str!("../../sql/get_user.sql");
        let user_from_db = sqlx::query_as::<_, User>(sql)
            .bind(&user.username)
            .fetch_optional(&*self.pool)
            .await?
            .ok_or(Report::msg(format!("User {} could not be found in the database.", &user.username)))?;
        let hash_and_salt_from_frontend: HashAndSalt = crypto_svc.hash_password(&user.password.to_owned(), Some(user_from_db.salt))?;
        if hash_and_salt_from_frontend.password_hash.eq(&user_from_db.password_hash.trim()) {
            Ok(AuthenticatedUser {
                username: user_from_db.username,
                user_role: user_from_db.user_role,
            })
        } else {
            Err(Report::msg("Incorrect password."))
        }
    }
}

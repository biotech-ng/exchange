use crate::errors::errors::DbError;
use database::users::{User, UserInput};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUserDb {
    pool: PgPool,
}

impl PgUserDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub type OwnedUser = UserInput<
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
>;

#[async_trait::async_trait]
pub trait UserDb: Clone + Send + Sync + 'static {
    async fn get_user_by_email(
        &self,
        email: impl AsRef<str> + std::fmt::Debug + Send,
    ) -> Result<User, DbError>;

    async fn insert_user(&self, user_input: &OwnedUser) -> Result<Uuid, DbError>;

    async fn get_user(&self, id: &Uuid) -> Result<User, DbError>;

    async fn get_access_token(&self, id: &Uuid) -> Result<String, DbError>;

    async fn update_user_token(
        &self,
        id: &Uuid,
        token: impl AsRef<str> + std::fmt::Debug + Send,
        refresh_token: impl AsRef<str> + std::fmt::Debug + Send,
    ) -> Result<u64, DbError>;
}

#[async_trait::async_trait]
impl UserDb for PgUserDb {
    #[tracing::instrument(skip(self))]
    async fn get_user_by_email(
        &self,
        email: impl AsRef<str> + std::fmt::Debug + Send,
    ) -> Result<User, DbError> {
        database::users::get_user_by_email(&self.pool, email)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn insert_user(&self, user_input: &OwnedUser) -> Result<Uuid, DbError> {
        database::users::insert_user(&self.pool, user_input)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn get_user(&self, id: &Uuid) -> Result<User, DbError> {
        database::users::get_user(&self.pool, id)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn get_access_token(&self, id: &Uuid) -> Result<String, DbError> {
        database::users::get_access_token(&self.pool, id)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn update_user_token(
        &self,
        id: &Uuid,
        token: impl AsRef<str> + std::fmt::Debug + Send,
        refresh_token: impl AsRef<str> + std::fmt::Debug + Send,
    ) -> Result<u64, DbError> {
        database::users::update_user_token(&self.pool, id, token, refresh_token)
            .await
            .map_err(Into::into)
    }
}

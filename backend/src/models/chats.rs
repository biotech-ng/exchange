use crate::models::errors::DbError;
use database::chats::{Chat, ChatType};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgChatDb {
    pool: PgPool,
}

impl PgChatDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
pub trait ChatDb: Clone + Send + Sync + 'static {
    async fn get_chat(&self, id: Uuid) -> Result<Chat, DbError>;

    async fn insert_chat(
        &self,
        r#type: ChatType,
        title: impl AsRef<str> + std::fmt::Debug + Send,
        description: impl AsRef<str> + std::fmt::Debug + Send,
        avatar: impl AsRef<str> + std::fmt::Debug + Send,
    ) -> Result<Uuid, DbError>;
}

#[async_trait::async_trait]
impl ChatDb for PgChatDb {
    #[tracing::instrument(skip(self))]
    async fn get_chat(&self, id: Uuid) -> Result<Chat, DbError> {
        database::chats::get_chat(&self.pool, id)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn insert_chat(
        &self,
        r#type: ChatType,
        title: impl AsRef<str> + std::fmt::Debug + Send,
        description: impl AsRef<str> + std::fmt::Debug + Send,
        avatar: impl AsRef<str> + std::fmt::Debug + Send,
    ) -> Result<Uuid, DbError> {
        database::chats::insert_chat(&self.pool, r#type, title, description, avatar)
            .await
            .map_err(Into::into)
    }
}

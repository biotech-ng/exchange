use crate::models::errors::DbError;
use database::chats::ChatMessage;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgChatMessageDb {
    pool: PgPool,
}

impl PgChatMessageDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
pub trait ChatMessageDb: Clone + Send + Sync + 'static {
    async fn get_chat_message(&self, id: Uuid) -> Result<ChatMessage, DbError>;

    async fn insert_chat_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        message: impl AsRef<str> + std::fmt::Debug + Send,
        parent_id: Option<Uuid>,
    ) -> Result<Uuid, DbError>;
}

#[async_trait::async_trait]
impl ChatMessageDb for PgChatMessageDb {
    #[tracing::instrument(skip(self))]
    async fn get_chat_message(&self, id: Uuid) -> Result<ChatMessage, DbError> {
        database::chats::get_chat_message(&self.pool, id)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn insert_chat_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        message: impl AsRef<str> + std::fmt::Debug + Send,
        parent_id: Option<Uuid>,
    ) -> Result<Uuid, DbError> {
        database::chats::insert_chat_message(&self.pool, chat_id, sender_id, message, parent_id)
            .await
            .map_err(Into::into)
    }
}

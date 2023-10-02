use crate::models::errors::DbError;
use database::chats::{Chat, ChatType,CreateChat};
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
        chat: &CreateChat,
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
        chat: &CreateChat,
    ) -> Result<Uuid, DbError> {
        database::chats::insert_chat(&self.pool, chat)
            .await
            .map_err(Into::into)
    }
}

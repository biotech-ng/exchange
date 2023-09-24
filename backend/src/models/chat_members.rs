use crate::models::errors::DbError;
use database::chats::{ChatMember, ChatMemberRole};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgChatMemberDb {
    pool: PgPool,
}

impl PgChatMemberDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
pub trait ChatMemberDb: Clone + Send + Sync + 'static {
    async fn get_chat_member(&self, id: &Uuid) -> Result<ChatMember, DbError>;

    async fn insert_chat_member(
        &self,
        chat_id: Uuid,
        member: impl AsRef<str> + std::fmt::Debug + Send,
        role: ChatMemberRole,
    ) -> Result<Uuid, DbError>;
}

#[async_trait::async_trait]
impl ChatMemberDb for PgChatMemberDb {
    #[tracing::instrument(skip(self))]
    async fn get_chat_member(&self, id: Uuid) -> Result<ChatMember, DbError> {
        database::chats::get_chat_member(&self.pool, id)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn insert_chat_member(
        &self,
        chat_id: Uuid,
        member: impl AsRef<str> + std::fmt::Debug + Send,
        role: ChatMemberRole,
    ) -> Result<Uuid, DbError> {
        database::chats::insert_chat_member(&self.pool, chat_id, member, role)
            .await
            .map_err(Into::into)
    }
}

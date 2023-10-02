use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Copy, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Channel,
}

#[derive(sqlx::FromRow)]
pub struct Chat {
    pub id: Uuid,
    pub r#type: ChatType,
    pub title: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug)]
pub struct CreateChat {
    pub r#type: ChatType,
    pub title: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

pub async fn insert_chat(
    pool: &PgPool,
    chat: &CreateChat,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO chats ( id, type, title, description, avatar, created_at, updated_at )
                SELECT $1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
        Uuid::new_v4(),
        chat.r#type as ChatType,
        chat.title.as_ref(),
        chat.description.as_ref(),
        chat.avatar.as_ref(),
    )
    .fetch_one(pool)
    .await
    .map(|x| x.id)
}

pub async fn get_chat(pool: &PgPool, id: Uuid) -> Result<Chat, sqlx::Error> {
    sqlx::query_as!(
            Chat,
            r#"
                SELECT id, type as "type: _", title, description, avatar, created_at, updated_at FROM chats
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum ChatMemberRole {
    Creator,
    Admin,
    Member,
    Left,
    Banned,
}

#[derive(sqlx::FromRow)]
pub struct ChatMember {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub member: String,
    pub role: ChatMemberRole,
    pub last_read_message_id: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub async fn insert_chat_member(
    pool: &PgPool,
    chat_id: Uuid,
    member: impl AsRef<str>,
    role: ChatMemberRole,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO chat_member ( id, chat_id, member, role, created_at, updated_at )
                SELECT $1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
        Uuid::new_v4(),
        chat_id,
        member.as_ref(),
        role as ChatMemberRole,
    )
    .fetch_one(pool)
    .await
    .map(|x| x.id)
}

pub async fn get_chat_member(pool: &PgPool, id: Uuid) -> Result<ChatMember, sqlx::Error> {
    sqlx::query_as!(
            ChatMember,
            r#"
                SELECT id, chat_id, member, role as "role: _", last_read_message_id, created_at, updated_at FROM chat_member
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[derive(sqlx::FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub message: String,
    pub parent_id: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

pub async fn insert_chat_message(
    pool: &PgPool,
    chat_id: Uuid,
    sender_id: Uuid,
    message: impl AsRef<str>,
    parent_id: Option<Uuid>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO chat_messages ( id, chat_id, sender_id, message, parent_id, created_at, updated_at )
                SELECT $1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            chat_id,
            sender_id,
            message.as_ref(),
            parent_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_chat_message(pool: &PgPool, id: Uuid) -> Result<ChatMessage, sqlx::Error> {
    sqlx::query_as!(
            ChatMessage,
            r#"
                SELECT id, chat_id, sender_id, message, parent_id, created_at, updated_at, deleted_at FROM chat_messages
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::pg_pool;
    use crate::users::tests::create_random_user_inputs;
    use crate::users::{get_user, insert_user, User};

    #[tokio::test]
    async fn test_create_chat() {
        let r#type = ChatType::Channel;
        let title = "chat title";
        let description = "chat description";
        let avatar = "https://some_avatar.png";

        let pool = pg_pool().await.expect("pool is expected");

        let id = insert_chat(&pool, r#type, &title, &description, &avatar)
            .await
            .expect("chat is created");

        let chat = get_chat(&pool, id)
            .await
            .expect("user for given id is expected");

        assert_eq!(r#type, chat.r#type);
        assert_eq!(title, chat.title);
        assert_eq!(description, chat.description.expect("last name"));
        assert_eq!(avatar, chat.avatar.expect("last name"));
    }

    async fn create_chat(pool: &PgPool) -> Chat {
        let r#type = ChatType::Channel;
        let title = "chat title";
        let description = "chat description";
        let avatar = "https://some_avatar.png";

        let id = insert_chat(pool, r#type, &title, &description, &avatar)
            .await
            .expect("chat is created");

        get_chat(pool, id)
            .await
            .expect("user for given id is expected")
    }

    pub async fn create_user(pool: &PgPool) -> User {
        let user_input = create_random_user_inputs();

        let id = insert_user(pool, &user_input)
            .await
            .expect("user is created");

        get_user(pool, &id)
            .await
            .expect("user for given id is expected")
    }

    #[tokio::test]
    async fn test_create_chat_participant() {
        let pool = pg_pool().await.expect("pool is expected");

        let user = create_user(&pool).await;
        let chat = create_chat(&pool).await;

        let role = ChatMemberRole::Member;

        let chat_participant_id =
            insert_chat_member(&pool, chat.id, user.alias.expect("alias is expected"), role)
                .await
                .expect("chat member created");

        let chat_participant = get_chat_member(&pool, chat_participant_id)
            .await
            .expect("chat member");

        assert_eq!(chat_participant.role, role);
    }

    #[tokio::test]
    async fn test_create_chat_message() {
        let pool = pg_pool().await.expect("pool is expected");

        let chat = create_chat(&pool).await;

        let parent_user = create_user(&pool).await;
        let parent_message_text = "test message";

        let chat_parent_message_id =
            insert_chat_message(&pool, chat.id, parent_user.id, parent_message_text, None)
                .await
                .expect("parent message is created");

        let parent_message = get_chat_message(&pool, chat_parent_message_id)
            .await
            .expect("parent message");

        assert_eq!(parent_message.id, chat_parent_message_id);
        assert_eq!(parent_message.message, parent_message_text);
        assert_eq!(parent_message.sender_id, parent_user.id);
        assert_eq!(parent_message.chat_id, chat.id);
        assert_eq!(parent_message.parent_id, None);
        assert_eq!(parent_message.deleted_at, None);

        let sender_user = create_user(&pool).await;
        let message_text = "test message 2";

        let chat_message_id = insert_chat_message(
            &pool,
            chat.id,
            sender_user.id,
            message_text,
            Some(parent_message.id),
        )
        .await
        .expect("parent message is created");

        let message = get_chat_message(&pool, chat_message_id)
            .await
            .expect("parent message");

        assert_eq!(message.id, chat_message_id);
        assert_eq!(message.message, message_text);
        assert_eq!(message.sender_id, sender_user.id);
        assert_eq!(message.chat_id, chat.id);
        assert_eq!(message.parent_id, Some(chat_parent_message_id));
        assert_eq!(message.deleted_at, None);
    }
}

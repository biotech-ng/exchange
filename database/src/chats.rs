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

pub async fn insert_chat<T1: AsRef<str>, T2: AsRef<str>, T3: AsRef<str>>(
    pool: &PgPool,
    r#type: ChatType,
    title: T1,
    description: T2,
    avatar: T3,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO chats ( id, type, title, description, avatar, created_at, updated_at )
                SELECT $1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
        Uuid::new_v4(),
        r#type as ChatType,
        title.as_ref(),
        description.as_ref(),
        avatar.as_ref(),
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
pub enum ChatParticipantRole {
    Admin,
    Reader,
    Writer,
    Banned,
}

#[derive(sqlx::FromRow)]
pub struct ChatParticipant {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub participant: String,
    pub role: ChatParticipantRole,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

// TODO test
pub async fn insert_chat_participant<T: AsRef<str>>(
    pool: &PgPool,
    chat_id: Uuid,
    participant: T,
    role: ChatParticipantRole,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO chat_participant ( id, chat_id, participant, role, created_at, updated_at )
                SELECT $1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            chat_id,
            participant.as_ref(),
            role as ChatParticipantRole,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

// TODO test
pub async fn get_chat_participant(pool: &PgPool, id: Uuid) -> Result<ChatParticipant, sqlx::Error> {
    sqlx::query_as!(
            ChatParticipant,
            r#"
                SELECT id, chat_id, participant, role as "role: _", created_at, updated_at FROM chat_participant
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
    pub sender: String,
    pub message: String,
    pub parent_id: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

// TODO test
pub async fn insert_chat_message<T1: AsRef<str>, T2: AsRef<str>>(
    pool: &PgPool,
    chat_id: Uuid,
    sender: T1,
    message: T2,
    parent_id: Option<Uuid>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO chat_messages ( id, chat_id, sender, message, parent_id, created_at, updated_at )
                SELECT $1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            chat_id,
            sender.as_ref(),
            message.as_ref(),
            parent_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

// TODO test
pub async fn get_chat_message(pool: &PgPool, id: Uuid) -> Result<ChatMessage, sqlx::Error> {
    sqlx::query_as!(
            ChatMessage,
            r#"
                SELECT id, chat_id, sender, message, parent_id, created_at, updated_at, deleted_at FROM chat_messages
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pg_pool;
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

        let id = insert_chat(&pool, r#type, &title, &description, &avatar)
            .await
            .expect("chat is created");

        get_chat(&pool, id)
            .await
            .expect("user for given id is expected")
    }

    async fn create_user(pool: &PgPool) -> User {
        let alias = format!("vova:{}", Uuid::new_v4());
        let first_name = "volodymyr";
        let last_name = "gorbenko";
        let phone_number = format!("pn:{}", Uuid::new_v4());
        let language_code = "ru-ru";
        let avatar = "https://some_image.png";
        let country_code = "SW";

        let id = insert_user(
            &pool,
            &alias,
            &first_name,
            &last_name,
            &phone_number,
            &language_code,
            &avatar,
            &country_code,
        )
        .await
        .expect("user is created");

        get_user(&pool, id)
            .await
            .expect("user for given id is expected")
    }

    #[tokio::test]
    async fn test_create_chat_participant() {
        let pool = pg_pool().await.expect("pool is expected");

        let user = create_user(&pool).await;
        let chat = create_chat(&pool).await;

        let role = ChatParticipantRole::Writer;

        let chat_participant_id = insert_chat_participant(&pool, chat.id, user.alias, role)
            .await
            .expect("chat participant created");

        let chat_participant = get_chat_participant(&pool, chat_participant_id)
            .await
            .expect("chat participant");

        assert_eq!(chat_participant.role, role);
    }
}

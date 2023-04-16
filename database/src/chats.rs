use sqlx::PgPool;
use sqlx::types::time::PrimitiveDateTime;
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

pub async fn insert<T1: AsRef<str>, T2: AsRef<str>, T3: AsRef<str>>(
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


pub async fn get(pool: &PgPool, id: Uuid) -> Result<Chat, sqlx::Error> {
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

#[cfg(test)]
mod tests {
    use crate::pg_pool;
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let r#type = ChatType::Channel;
        let title = "chat title";
        let description = "chat description";
        let avatar = "https://some_avatar.png";

        let pool = pg_pool().await.expect("pool is expected");

        let id = insert(
            &pool,
            r#type,
            &title,
            &description,
            &avatar
        ).await.expect("chat is created");

        let chat = get(&pool, id).await.expect("user for given id is expected");

        assert_eq!(r#type, chat.r#type);
        assert_eq!(title, chat.title);
        assert_eq!(description, chat.description.expect("last name"));
        assert_eq!(avatar, chat.avatar.expect("last name"));
    }
}
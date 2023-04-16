use sqlx::PgPool;
use sqlx::types::time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub alias: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: String,
    pub language_code: String,
    pub avatar: Option<String>,
    pub country_code: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub accessed_at: PrimitiveDateTime,
}

pub async fn insert(
    pool: &PgPool,
    alias: &str,
    first_name: &str,
    last_name: &str,
    phone_number: &str,
    language_code: &str,
    avatar: &str,
    country_code: &str,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO users ( id, alias, first_name, last_name, phone_number, language_code, avatar, country_code, created_at, updated_at, accessed_at )
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            alias,
            first_name,
            last_name,
            phone_number,
            language_code,
            avatar,
            country_code
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}


pub async fn get(pool: &PgPool,id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
            User,
            r#"
                SELECT id, alias, first_name, last_name, phone_number, language_code, avatar, country_code, created_at, updated_at, accessed_at FROM users
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
        let alias = format!("vova:{}", Uuid::new_v4());
        let first_name = "volodymyr";
        let last_name = "gorbenko";
        let phone_number = format!("pn:{}", Uuid::new_v4());
        let language_code = "ru-ru";
        let avatar = "https://some_image.png";
        let country_code = "SW";

        let pool = pg_pool().await.expect("pool is expected");

        let id = insert(
            &pool,
            alias.as_str(),
            first_name,
            last_name,
            phone_number.as_str(),
            language_code,
            avatar,
            country_code,
        ).await.expect("user is created");

        let user = get(&pool, id).await.expect("user for given id is expected");

        assert_eq!(alias, user.alias);
    }
}
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

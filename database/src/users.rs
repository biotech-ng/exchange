use sqlx::PgPool;
use sqlx::types::time::PrimitiveDateTime;
use uuid::Uuid;

// avatar        text,
// country_code  character varying(2), -- ISO 3166-1 alpha-2
// created_at    timestamp(0) without time zone NOT NULL,
// updated_at    timestamp(0) without time zone NOT NULL,
// accessed_at   timestamp(0) without time zone NOT NULL

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub alias: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub language_code: String,
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
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO users ( id, alias, first_name, last_name, phone_number, language_code, created_at, updated_at, accessed_at )
                SELECT $1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            alias,
            first_name,
            last_name,
            phone_number,
            language_code
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

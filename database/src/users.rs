use sqlx::PgPool;
use uuid::Uuid;

// phone_number  character varying(255) NOT NULL,
// language_code character varying(5) NOT NULL, -- ISO 639-1 standard language codes
// avatar        text,
// country_code  character varying(2), -- ISO 3166-1 alpha-2
// created_at    timestamp(0) without time zone NOT NULL,
// updated_at    timestamp(0) without time zone NOT NULL,
// accessed_at   timestamp(0) without time zone NOT NULL

pub async fn insert(
    pool: &PgPool,
    alias: &str,
    first_name: &str,
    last_name: &str,
    phone_number: &str,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO users ( id, alias, first_name, last_name, phone_number, created_at, updated_at, accessed_at )
                SELECT $1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            alias,
            first_name,
            last_name,
            phone_number,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

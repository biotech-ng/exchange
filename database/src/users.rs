use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
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

pub async fn insert_user<
    T1: AsRef<str>,
    T2: AsRef<str>,
    T3: AsRef<str>,
    T4: AsRef<str>,
    T5: AsRef<str>,
    T6: AsRef<str>,
    T7: AsRef<str>,
>(
    pool: &PgPool,
    alias: T1,
    first_name: T2,
    last_name: T3,
    phone_number: T4,
    language_code: T5,
    avatar: T6,
    country_code: T7,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO users ( id, alias, first_name, last_name, phone_number, language_code, avatar, country_code, created_at, updated_at, accessed_at )
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            alias.as_ref(),
            first_name.as_ref(),
            last_name.as_ref(),
            phone_number.as_ref(),
            language_code.as_ref(),
            avatar.as_ref(),
            country_code.as_ref()
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
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
    use super::*;
    use crate::pg_pool;

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

        let user = get_user(&pool, id)
            .await
            .expect("user for given id is expected");

        assert_eq!(alias, user.alias);
        assert_eq!(first_name, user.first_name.expect("first name"));
        assert_eq!(last_name, user.last_name.expect("last name"));
        assert_eq!(phone_number, user.phone_number);
        assert_eq!(language_code, user.language_code);
        assert_eq!(avatar, user.avatar.expect("avatar"));
        assert_eq!(country_code, user.country_code.expect("country code"));
    }
}

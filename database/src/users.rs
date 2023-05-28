use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub alias: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password_salt: String,
    pub password_sha512: String,
    pub phone_number: Option<String>,
    pub language_code: String,
    pub avatar: Option<String>,
    pub country_code: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub accessed_at: PrimitiveDateTime,
}

pub struct UserInput<
    T1: AsRef<str>,
    T2: AsRef<str>,
    T3: AsRef<str>,
    T4: AsRef<str>,
    T5: AsRef<str>,
    T6: AsRef<str>,
    T7: AsRef<str>,
    T8: AsRef<str>,
    T9: AsRef<str>,
    T10: AsRef<str>,
> {
    pub alias: T1,
    pub first_name: T2,
    pub last_name: T3,
    pub email: T4,
    pub password_salt: T5,
    pub password_sha512: T6,
    pub phone_number: Option<T7>,
    pub language_code: T8,
    pub avatar: T9,
    pub country_code: T10,
}

pub async fn insert_user<
    T1: AsRef<str>,
    T2: AsRef<str>,
    T3: AsRef<str>,
    T4: AsRef<str>,
    T5: AsRef<str>,
    T6: AsRef<str>,
    T7: AsRef<str>,
    T8: AsRef<str>,
    T9: AsRef<str>,
    T10: AsRef<str>,
>(
    pool: &PgPool,
    user_input: UserInput<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO users ( id, alias, first_name, last_name, email, password_salt, password_sha512, phone_number, language_code, avatar, country_code, created_at, updated_at, accessed_at )
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            user_input.alias.as_ref(),
            user_input.first_name.as_ref(),
            user_input.last_name.as_ref(),
            user_input.email.as_ref(),
            user_input.password_salt.as_ref(),
            user_input.password_sha512.as_ref(),
            user_input.phone_number.as_ref().map(|x| x.as_ref()),
            user_input.language_code.as_ref(),
            user_input.avatar.as_ref(),
            user_input.country_code.as_ref()
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
            User,
            r#"
                SELECT id, alias, first_name, last_name, email, password_salt, password_sha512, phone_number, language_code, avatar, country_code, created_at, updated_at, accessed_at FROM users
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
        let email = format!("em:{}", Uuid::new_v4());
        let password_salt = format!("ps:{}", Uuid::new_v4());
        let password_sha512 = format!("ph:{}", Uuid::new_v4());
        let phone_number = Some(format!("pn:{}", Uuid::new_v4()));
        let language_code = "ru-ru";
        let avatar = "https://some_image.png";
        let country_code = "SW";

        let pool = pg_pool().await.expect("pool is expected");

        let user_input = UserInput {
            alias: &alias,
            first_name,
            last_name,
            email: &email,
            password_salt: &password_salt,
            password_sha512: &password_sha512,
            phone_number: phone_number.as_ref(),
            language_code,
            avatar,
            country_code,
        };

        let id = insert_user(&pool, user_input)
            .await
            .expect("user is created");

        let user = get_user(&pool, id)
            .await
            .expect("user for given id is expected");

        assert_eq!(alias, user.alias);
        assert_eq!(first_name, user.first_name.expect("first name"));
        assert_eq!(last_name, user.last_name.expect("last name"));
        assert_eq!(email, user.email);
        assert_eq!(password_salt, user.password_salt);
        assert_eq!(password_sha512, user.password_sha512);
        assert_eq!(phone_number, user.phone_number);
        assert_eq!(language_code, user.language_code);
        assert_eq!(avatar, user.avatar.expect("avatar"));
        assert_eq!(country_code, user.country_code.expect("country code"));
    }
}

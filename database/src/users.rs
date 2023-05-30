use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub alias: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password_salt: String,
    pub password_sha512: String,
    pub access_token: String,
    pub phone_number: Option<String>,
    pub language_code: String,
    pub avatar: Option<String>,
    pub country_code: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub accessed_at: PrimitiveDateTime,
}

#[derive(Debug)]
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
    T11: AsRef<str>,
> {
    pub alias: Option<T1>,
    pub first_name: Option<T2>,
    pub last_name: Option<T3>,
    pub email: T4,
    pub password_salt: T5,
    pub password_sha512: T6,
    pub access_token: T7,
    pub phone_number: Option<T8>,
    pub language_code: T9,
    pub avatar: Option<T10>,
    pub country_code: Option<T11>,
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
    T11: AsRef<str>,
>(
    pool: &PgPool,
    user_input: &UserInput<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO users ( id, alias, first_name, last_name, email, password_salt, password_sha512, access_token, phone_number, language_code, avatar, country_code, created_at, updated_at, accessed_at )
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            user_input.alias.as_ref().map(|x| x.as_ref()),
            user_input.first_name.as_ref().map(|x| x.as_ref()),
            user_input.last_name.as_ref().map(|x| x.as_ref()),
            user_input.email.as_ref(),
            user_input.password_salt.as_ref(),
            user_input.password_sha512.as_ref(),
            user_input.access_token.as_ref(),
            user_input.phone_number.as_ref().map(|x| x.as_ref()),
            user_input.language_code.as_ref(),
            user_input.avatar.as_ref().map(|x| x.as_ref()),
            user_input.country_code.as_ref().map(|x| x.as_ref())
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn update_user_token<T: AsRef<str>>(pool: &PgPool, id: &Uuid, token: T) -> Result<(), sqlx::Error> {
    sqlx::query!(
            r#"
                UPDATE users
                SET access_token = $1
                WHERE id = $2
            "#,
            token.as_ref(),
            id
        )
        .execute(pool)
        .await
        .map(|_| ())
}

pub async fn get_user(pool: &PgPool, id: &Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
            User,
            r#"
                SELECT id, alias, first_name, last_name, email, password_salt, password_sha512, phone_number, access_token, language_code, avatar, country_code, created_at, updated_at, accessed_at FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

pub async fn get_user_by_email<T: AsRef<str> + std::fmt::Debug + Send>(
    pool: &PgPool,
    email: T,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
            User,
            r#"
                SELECT id, alias, first_name, last_name, email, password_salt, password_sha512, phone_number, access_token, language_code, avatar, country_code, created_at, updated_at, accessed_at FROM users
                WHERE email = $1
            "#,
            email.as_ref()
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::pg_pool;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use crate::utils::random_samples::RandomSample;

    type TestUserInputs = UserInput<
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        &'static str,
        &'static str,
        &'static str,
    >;

    // TODO generate normal password
    pub fn create_random_user_inputs() -> TestUserInputs {
        let alias = Some(format!("vova:{}", Uuid::new_v4()));
        let first_name = Some("volodymyr".to_owned());
        let last_name = Some("gorbenko".to_owned());
        let email = format!("em:{}", Uuid::new_v4());
        let password_salt = String::new_random(22);
        let password_sha512 = format!("ph:{}", Uuid::new_v4());
        let access_token = String::new_random(1025);
        let phone_number = Some(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(15)
                .map(char::from)
                .collect(),
        );
        let language_code = "ru-ru";
        let avatar = Some("https://some_image.png");
        let country_code = Some("SW");

        UserInput {
            alias,
            first_name,
            last_name,
            email,
            password_salt,
            password_sha512,
            access_token,
            phone_number,
            language_code,
            avatar,
            country_code,
        }
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = pg_pool().await.expect("pool is expected");

        let user_input = create_random_user_inputs();

        let id = insert_user(&pool, &user_input)
            .await
            .expect("user is created");

        let user = get_user(&pool, &id)
            .await
            .expect("user for given id is expected");

        assert_eq!(id, user.id);
        assert_eq!(user_input.first_name, user.first_name);
        assert_eq!(user_input.last_name, user.last_name);
        assert_eq!(user_input.email, user.email);
        assert_eq!(user_input.password_salt, user.password_salt);
        assert_eq!(user_input.password_sha512, user.password_sha512);
        assert_eq!(user_input.access_token, user.access_token);
        assert_eq!(user_input.phone_number, user.phone_number);
        assert_eq!(user_input.language_code, user.language_code);
        assert_eq!(user_input.avatar, user.avatar.as_ref().map(|x| x.as_ref()));
        assert_eq!(
            user_input.country_code,
            user.country_code.as_ref().map(|x| x.as_ref())
        );
    }

    #[tokio::test]
    async fn test_get_user_by_email() {
        let pool = pg_pool().await.expect("pool is expected");

        let user_input = create_random_user_inputs();

        let id = insert_user(&pool, &user_input)
            .await
            .expect("user is created");

        let user = get_user_by_email(&pool, &user_input.email)
            .await
            .expect("user for given id is expected");

        assert_eq!(id, user.id);
        assert_eq!(user_input.alias, user.alias);
        assert_eq!(user_input.first_name, user.first_name);
        assert_eq!(user_input.last_name, user.last_name);
        assert_eq!(user_input.email, user.email);
        assert_eq!(user_input.password_salt, user.password_salt);
        assert_eq!(user_input.password_sha512, user.password_sha512);
        assert_eq!(user_input.access_token, user.access_token);
        assert_eq!(user_input.phone_number, user.phone_number);
        assert_eq!(user_input.language_code, user.language_code);
        assert_eq!(user_input.avatar, user.avatar.as_ref().map(|x| x.as_ref()));
        assert_eq!(
            user_input.country_code,
            user.country_code.as_ref().map(|x| x.as_ref())
        );
    }

    #[tokio::test]
    async fn test_update_token() {
        let pool = pg_pool().await.expect("pool is expected");

        let user_input = create_random_user_inputs();

        let id = insert_user(&pool, &user_input)
            .await
            .expect("user is created");

        let user = get_user_by_email(&pool, &user_input.email)
            .await
            .expect("user for given id is expected");

        assert_eq!(user_input.access_token, user.access_token);

        let access_token = String::new_random(1025);

        _ = update_user_token(&pool, &id, &access_token).await;

        let user = get_user_by_email(&pool, &user_input.email)
            .await
            .expect("user for given id is expected");
        assert_eq!(access_token, user.access_token);
    }
}

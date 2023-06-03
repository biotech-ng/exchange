use crate::models::user::UserDb;
use crate::utils::tokens::{AccessToken, AccessTokenResponse, DigestAccessToken, UserInfo};
use time::{OffsetDateTime, PrimitiveDateTime};

// TODO fix race condition
// TODO: must be a layer
pub async fn authenticate_with_token<UDB: UserDb>(
    token: impl AsRef<str>,
    user_db: &UDB,
) -> (AccessTokenResponse, UserInfo) {
    let access_token = AccessToken::from_token(token.as_ref()).expect("TODO");

    let user = user_db
        .get_user(&access_token.get_user().user_id)
        .await
        .expect("TODO");
    let user_info = access_token.get_user().clone();

    if user.access_token != token.as_ref() {
        panic!("TODO: wrong token");
    }

    let now = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now.date(), now.time());

    let access_token_response = if access_token.get_expires_at() <= &now {
        if access_token.get_expires_at() <= &now {
            panic!("TODO: wrong token");
        }

        let access_token = AccessToken::new_with_user(access_token.get_user().clone());

        let token_response: AccessTokenResponse = DigestAccessToken::try_from(access_token)
            .expect("TODO 1")
            .try_into()
            .expect("TODO 2");

        user_db
            .update_user_token(&user.id, &token_response.token)
            .await
            .expect("TODO");

        token_response
    } else {
        DigestAccessToken::try_from(access_token)
            .expect("TODO 1")
            .try_into()
            .expect("TODO 2")
    };

    (access_token_response, user_info)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use database::utils::random_samples::RandomSample;
    use crate::models::user::{OwnedUser, PgUserDb, UserDb};
    use crate::utils::salted_hashes::generate_hash_and_salt_for_text;
    use crate::utils::tokens::tests::{create_token, make_expired_token};
    use crate::utils::tokens::{AccessTokenResponse, UserInfo};

    async fn create_user(user_db: &impl UserDb, user: UserInfo, token_response: AccessTokenResponse) -> Uuid {
        let email = Uuid::new_v4().to_string();
        let password = std::format!("password:{:?}", String::new_random(124));
        let first_name = user.first_name;
        let last_name = user.last_name;
        let language_code = "ru-ru".to_owned();

        let (password_sha512, password_salt) =
            generate_hash_and_salt_for_text(&password);

        let user_input = OwnedUser {
            user_id: user.user_id,
            alias: None,
            first_name,
            last_name,
            email,
            password_salt,
            password_sha512,
            access_token: token_response.token,
            phone_number: None,
            language_code,
            avatar: None,
            country_code: None,
        };

        user_db.insert_user(&user_input).await.expect("user created")
    }

    #[tokio::test]
    async fn two_parallel_requests_receives_the_same_refreshed_tokens() {
        let pool = crate::pg_pool()
            .await
            .expect("failed to create postgres pool");
        let user_db = PgUserDb::new(pool);

        let (user, token_response) = create_token();

        let user_id = create_user(&user_db, user.clone(), token_response).await;

        let expired_token = make_expired_token(user);
        user_db.update_user_token(&user_id, expired_token.token).await.expect("token updated");


    }
}
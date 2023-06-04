use crate::models::user::UserDb;
use crate::utils::tokens::{AccessToken, AccessTokenResponse, DigestAccessToken, UserInfo};
use std::ops::Add;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

// TODO fix race condition
// TODO: must be a layer
pub async fn authenticate_with_token(
    token: impl AsRef<str>,
    user_db: &impl UserDb,
) -> (AccessTokenResponse, UserInfo) {
    let access_token = AccessToken::from_token(token.as_ref()).expect("TODO");
    let user_info = access_token.get_user().clone();

    let now = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now.date(), now.time());

    // When token is fresh
    if access_token.get_expires_at() > &now {
        let access_token_response = AccessTokenResponse {
            token: String::from(token.as_ref()),
            expires_at: *access_token.get_expires_at(),
            refresh_at: *access_token.get_refresh_at(),
        };
        return (access_token_response, user_info);
    }

    let user = user_db
        .get_user(&access_token.get_user().user_id)
        .await
        .expect("TODO");

    if user.access_token != token.as_ref() {
        // In case of token refresh race condition, check a previous token
        if let Some(previous_access_token) = user.previous_access_token {
            let two_minutes_before_now = now.add(-Duration::seconds(30));
            let two_minutes_before_now = PrimitiveDateTime::new(
                two_minutes_before_now.date(),
                two_minutes_before_now.time(),
            );

            if previous_access_token == token.as_ref()
                && access_token.get_expires_at() > &two_minutes_before_now
            {
                let access_token = AccessToken::from_token(&user.access_token).expect("TODO critical error");
                let access_token_response = AccessTokenResponse {
                    token: user.access_token,
                    expires_at: *access_token.get_expires_at(),
                    refresh_at: *access_token.get_refresh_at(),
                };
                return (access_token_response, user_info);
            }
        }

        panic!("TODO: wrong token 1, return un-authorised");
    }

    if access_token.get_refresh_at() <= &now {
        panic!("TODO: wrong token 2, return un-authorised");
    }

    let access_token: DigestAccessToken =
        AccessToken::new_with_user(access_token.get_user().clone())
            .try_into()
            .expect("TODO 1");

    let token_response: AccessTokenResponse = access_token.try_into().expect("TODO 2");

    let update_result = user_db
        .update_user_token(&user.id, &token_response.token, &user.access_token)
        .await
        .expect("TODO");

    let access_token_response = if update_result == 0 {
        // In case of token refresh race condition, return token from database
        let token = user_db
            .get_user(&user_info.user_id)
            .await
            .expect("TODO")
            .access_token;
        let access_token = AccessToken::from_token(token).expect("TODO");
        DigestAccessToken::try_from(access_token)
            .expect("TODO 1")
            .try_into()
            .expect("TODO 2")
    } else {
        token_response
    };

    (access_token_response, user_info)
}

#[cfg(test)]
mod tests {
    use crate::models::user::{OwnedUser, PgUserDb, UserDb};
    use crate::utils::salted_hashes::generate_hash_and_salt_for_text;
    use crate::utils::tokens::tests::{create_token, make_expired_token};
    use crate::utils::tokens::{AccessTokenResponse, UserInfo};
    use crate::web::authentication::authenticate_with_token;
    use database::utils::random_samples::RandomSample;
    use time::Duration;
    use uuid::Uuid;

    async fn create_user(
        user_db: &impl UserDb,
        user: UserInfo,
        token_response: AccessTokenResponse,
    ) -> Uuid {
        let email = Uuid::new_v4().to_string();
        let password = std::format!("password:{:?}", String::new_random(124));
        let first_name = user.first_name;
        let last_name = user.last_name;
        let language_code = "ru-ru".to_owned();

        let (password_sha512, password_salt) = generate_hash_and_salt_for_text(&password);

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

        user_db
            .insert_user(&user_input)
            .await
            .expect("user created")
    }

    #[tokio::test]
    async fn two_parallel_requests_receives_the_same_refreshed_tokens() {
        let pool = crate::pg_pool()
            .await
            .expect("failed to create postgres pool");
        let user_db = PgUserDb::new(pool);

        let (user, token_response) = create_token();
        let old_token = token_response.token.clone();

        let user_id = create_user(&user_db, user.clone(), token_response).await;

        let expired_token = make_expired_token(user, Duration::microseconds(1));
        let updated = user_db
            .update_user_token(&user_id, &expired_token.token, &old_token)
            .await
            .expect("token updated");
        assert!(updated > 0);

        let refresh_a = authenticate_with_token(&expired_token.token, &user_db);
        let refresh_b = authenticate_with_token(&expired_token.token, &user_db);
        let refresh_c = authenticate_with_token(&expired_token.token, &user_db);
        let refresh_d = authenticate_with_token(&expired_token.token, &user_db);
        let refresh_e = authenticate_with_token(&expired_token.token, &user_db);

        let (result_a, result_b, result_c, result_d, result_e) =
            tokio::join!(refresh_a, refresh_b, refresh_c, refresh_d, refresh_e);
        assert_ne!(result_a.0.token, expired_token.token);
        assert_ne!(result_b.0.token, expired_token.token);
        assert_ne!(result_c.0.token, expired_token.token);
        assert_ne!(result_d.0.token, expired_token.token);
        assert_ne!(result_e.0.token, expired_token.token);

        assert_eq!(result_a.0.token, result_b.0.token);
        assert_eq!(result_a.0.token, result_c.0.token);
        assert_eq!(result_a.0.token, result_d.0.token);
        assert_eq!(result_a.0.token, result_e.0.token);
    }
}

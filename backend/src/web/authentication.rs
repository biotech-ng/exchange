use crate::models::user::UserDb;
use crate::utils::tokens::{AccessToken, AccessTokenResponse, DigestAccessToken, UserInfo};
use time::{OffsetDateTime, PrimitiveDateTime};

// TODO fix race condition
// TODO: must be a layer
pub async fn authenticate_with_token<T: AsRef<str>, UDB: UserDb>(
    token: T,
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

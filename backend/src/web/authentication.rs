use time::{OffsetDateTime, PrimitiveDateTime};
use crate::models::user::UserDb;
use crate::utils::tokens::{AccessToken, DigestAccessToken, AccessTokenResponse};

pub async fn authenticate_with_token<T: AsRef<str>, UDB: UserDb>(token: T, user_db: &UDB) -> AccessTokenResponse {

    let access_token = AccessToken::from_token(token.as_ref()).expect("TODO");

    let user = user_db.get_user(&access_token.get_user().user_id).await.expect("TODO");

    if user.access_token != token.as_ref() {
        panic!("TODO: wrong token");
    }

    let now = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now.date(), now.time());

    let access_token: AccessTokenResponse = if access_token.get_expires_at() <= &now {
        if access_token.get_expires_at() <= &now {
            panic!("TODO: wrong token");
        }

        let result = AccessToken::new_with_user(access_token.get_user().clone());

        let token_response = DigestAccessToken::try_from(result).expect("TODO").into_token_response();

        user_db.update_user_token(&user.id, &token_response.token).await.expect("TODO");

        token_response
    } else {
        // String::from(token.as_ref())
        todo!()
    };

    //access_token
    todo!()
}

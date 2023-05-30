use std::borrow::Cow;

#[derive(Debug)]
pub enum DbError {
    NotFoundError,
    UnavailableTryAgain,
    UnexpectedError(String),
}

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFoundError,
            // pg error: serialization_failure
            sqlx::Error::Database(db_error) if db_error.code() == Some(Cow::Borrowed("40001")) => {
                DbError::UnavailableTryAgain
            }
            _ => Self::UnexpectedError(value.to_string()),
        }
    }
}
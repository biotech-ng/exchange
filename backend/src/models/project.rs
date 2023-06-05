use crate::models::errors::DbError;
use database::projects::{Project, ProjectInput};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgProjectDb {
    pool: PgPool,
}

impl PgProjectDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
pub trait ProjectDb: Clone + Send + Sync + 'static {
    async fn get_project_by_id(&self, id: &Uuid) -> Result<Project, DbError>;

    async fn insert_project(
        &self,
        user_input: &ProjectInput<String, String>,
    ) -> Result<Uuid, DbError>;
}

#[async_trait::async_trait]
impl ProjectDb for PgProjectDb {
    #[tracing::instrument(skip(self))]
    async fn get_project_by_id(&self, id: &Uuid) -> Result<Project, DbError> {
        database::projects::get_project(&self.pool, id)
            .await
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    async fn insert_project(
        &self,
        user_input: &ProjectInput<String, String>,
    ) -> Result<Uuid, DbError> {
        database::projects::insert_project(&self.pool, user_input)
            .await
            .map_err(Into::into)
    }
}

use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub async fn insert_project<T1: AsRef<str>, T2: AsRef<str>>(
    pool: &PgPool,
    name: T1,
    description: T2,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO projects ( id, name, description, created_at, updated_at )
                SELECT $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            name.as_ref(),
            description.as_ref(),
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_project(pool: &PgPool, id: Uuid) -> Result<Project, sqlx::Error> {
    sqlx::query_as!(
            Project,
            r#"
                SELECT id, name, description, created_at, updated_at FROM projects
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[derive(sqlx::FromRow)]
pub struct CompanyProject {
    pub id: Uuid,
    pub project_id: Uuid,
    pub company_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub async fn insert_company_project(
    pool: &PgPool,
    project_id: Uuid,
    company_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO company_projects ( id, project_id, company_id, created_at, updated_at )
                SELECT $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            project_id,
            company_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_company_project(pool: &PgPool, id: Uuid) -> Result<CompanyProject, sqlx::Error> {
    sqlx::query_as!(
            CompanyProject,
            r#"
                SELECT id, project_id, company_id, created_at, updated_at FROM company_projects
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
    use crate::companies::tests::create_company;
    use super::*;
    use crate::pg_pool;

    async fn create_project(pool: &PgPool) -> Project {
        let name = "project name";
        let description = "project description";

        let project_id = insert_project(&pool, name, description).await.expect("project created");
        get_project(&pool, project_id).await.expect("project returned")
    }

    #[tokio::test]
    async fn test_create_project() {
        let pool = pg_pool().await.expect("pool is expected");

        let name = "project name";
        let description = "project description";

        let project_id = insert_project(&pool, name, description).await.expect("project created");
        let project = get_project(&pool, project_id).await.expect("project returned");

        assert_eq!(project.name, name);
        assert_eq!(project.description, description);
    }

    #[tokio::test]
    async fn test_create_company_project() {
        let pool = pg_pool().await.expect("pool is expected");

        let project = create_project(&pool).await;
        let company = create_company(&pool).await;

        let company_project_id = insert_company_project(&pool, project.id, company.id)
            .await.expect("company project created");
        let company_project = get_company_project(&pool, company_project_id).await.expect("company project returned");

        assert_eq!(company_project.project_id, project.id);
        assert_eq!(company_project.company_id, company.id);
    }
}

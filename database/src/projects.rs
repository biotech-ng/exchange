use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub user_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug)]
pub struct ProjectInput<T1: AsRef<str>, T2: AsRef<str>> {
    pub name: T1,
    pub description: T2,
    pub user_id: Uuid,
}

pub async fn insert_project<T1: AsRef<str>, T2: AsRef<str>>(
    pool: &PgPool,
    project: &ProjectInput<T1, T2>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO projects ( id, name, description, user_id, created_at, updated_at )
                SELECT $1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
        Uuid::new_v4(),
        project.name.as_ref(),
        project.description.as_ref(),
        project.user_id
    )
    .fetch_one(pool)
    .await
    .map(|x| x.id)
}

pub async fn get_project(pool: &PgPool, id: &Uuid) -> Result<Project, sqlx::Error> {
    sqlx::query_as!(
        Project,
        r#"
                SELECT id, name, description, created_at, user_id, updated_at FROM projects
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
    company_id: Uuid,
    project_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO company_projects ( id, company_id, project_id, created_at, updated_at )
                SELECT $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
        Uuid::new_v4(),
        company_id,
        project_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| x.id)
}

pub async fn get_company_project(pool: &PgPool, id: Uuid) -> Result<CompanyProject, sqlx::Error> {
    sqlx::query_as!(
        CompanyProject,
        r#"
                SELECT id, company_id, project_id, created_at, updated_at FROM company_projects
                WHERE id = $1
            "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

#[derive(sqlx::FromRow)]
pub struct ProjectMember {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub async fn insert_project_member(
    pool: &PgPool,
    project_id: Uuid,
    member_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO project_members ( id, project_id, user_id, created_at, updated_at )
                SELECT $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
        Uuid::new_v4(),
        project_id,
        member_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| x.id)
}

pub async fn get_project_member(pool: &PgPool, id: Uuid) -> Result<ProjectMember, sqlx::Error> {
    sqlx::query_as!(
        ProjectMember,
        r#"
                SELECT id, project_id, user_id, created_at, updated_at FROM project_members
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
    use crate::chats::tests::create_user;
    use crate::companies::tests::create_company;
    use crate::pg_pool;

    async fn create_project(pool: &PgPool) -> Project {
        let user_id = create_user(pool).await.id;

        let project_input = ProjectInput {
            name: "project name",
            description: "project description",
            user_id
        };

        let project_id = insert_project(pool, &project_input)
            .await
            .expect("project created");
        get_project(&pool, &project_id)
            .await
            .expect("project returned")
    }

    #[tokio::test]
    async fn test_create_project() {
        let pool = pg_pool().await.expect("pool is expected");
        let user_id = create_user(&pool).await.id;

        let project_input = ProjectInput {
            name: "project name",
            description: "project description",
            user_id,
        };

        let project_id = insert_project(&pool, &project_input)
            .await
            .expect("project created");
        let project = get_project(&pool, &project_id)
            .await
            .expect("project returned");

        assert_eq!(project.name, project_input.name);
        assert_eq!(project.description, project_input.description);
        assert_eq!(project.user_id, project_input.user_id);
    }

    #[tokio::test]
    async fn test_create_company_project() {
        let pool = pg_pool().await.expect("pool is expected");

        let project = create_project(&pool).await;
        let company = create_company(&pool).await;

        let company_project_id = insert_company_project(&pool, company.id, project.id)
            .await
            .expect("company project created");
        let company_project = get_company_project(&pool, company_project_id)
            .await
            .expect("company project returned");

        assert_eq!(company_project.project_id, project.id);
        assert_eq!(company_project.company_id, company.id);
    }

    #[tokio::test]
    async fn test_create_project_member() {
        let pool = pg_pool().await.expect("pool is expected");

        let project = create_project(&pool).await;
        let user = create_user(&pool).await;

        let project_member_id = insert_project_member(&pool, project.id, user.id)
            .await
            .expect("company project created");
        let project_member = get_project_member(&pool, project_member_id)
            .await
            .expect("company project returned");

        assert_eq!(project_member.project_id, project.id);
        assert_eq!(project_member.user_id, user.id);
    }
}

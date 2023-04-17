use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub address_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub async fn insert_company<T: AsRef<str>>(
    pool: &PgPool,
    name: T,
    address_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO companies ( id, name, address_id, created_at, updated_at )
                SELECT $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            name.as_ref(),
            address_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_company(pool: &PgPool, id: Uuid) -> Result<Company, sqlx::Error> {
    sqlx::query_as!(
            Company,
            r#"
                SELECT id, name, address_id, created_at, updated_at FROM companies
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[derive(sqlx::FromRow)]
pub struct CompanyMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub async fn insert_company_member(
    pool: &PgPool,
    user_id: Uuid,
    company_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO company_members ( id, user_id, company_id, created_at, updated_at )
                SELECT $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            user_id,
            company_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_company_member(pool: &PgPool, id: Uuid) -> Result<CompanyMember, sqlx::Error> {
    sqlx::query_as!(
            CompanyMember,
            r#"
                SELECT id, user_id, company_id, created_at, updated_at FROM company_members
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
pub mod tests {
    use crate::addresses::{Address, get_addresses, insert_addresses};
    use crate::chats::tests::create_user;
    use super::*;
    use crate::pg_pool;

    pub async fn create_addresses(pool: &PgPool) -> Address {
        let zip_code = 76236;
        let country = "sw";
        let region = "region";
        let city = "city";
        let district = Some("district");
        let street = "street";
        let building = "building";
        let apartment = "apartment";

        let id = insert_addresses(
            pool, zip_code, country, region, city, district, street, building, apartment,
        )
            .await
            .expect("user is created");

        get_addresses(&pool, id)
            .await
            .expect("user for given id is expected")
    }

    pub async fn create_company(pool: &PgPool) -> Company {
        let address = create_addresses(&pool).await;

        let name = "company";

        let id = insert_company(
            &pool, name, address.id,
        )
            .await
            .expect("company is created");

        get_company(&pool, id)
            .await
            .expect("company for given id is expected")
    }

    #[tokio::test]
    async fn test_create_company() {
        let pool = pg_pool().await.expect("pool is expected");

        let address = create_addresses(&pool).await;

        let name = "company";

        let id = insert_company(
            &pool, name, address.id,
        )
            .await
            .expect("company is created");

        let company = get_company(&pool, id)
            .await
            .expect("company for given id is expected");

        assert_eq!(name, company.name);
        assert_eq!(address.id, company.address_id);
    }

    #[tokio::test]
    async fn test_create_company_member() {
        let pool = pg_pool().await.expect("pool is expected");

        let company = create_company(&pool).await;
        let user = create_user(&pool).await;

        let company_member_id = insert_company_member(
            &pool, user.id, company.id
        ).await.expect("company member");

        let company_member = get_company_member(
            &pool, company_member_id
        ).await.expect("company member");

        assert_eq!(company_member.company_id, company.id);
        assert_eq!(company_member.user_id, user.id);
    }
}

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
mod tests {
    use crate::addresses::{Address, get_addresses, insert_addresses};
    use super::*;
    use crate::pg_pool;

    async fn create_addresses(pool: &PgPool) -> Address {
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

    #[tokio::test]
    async fn test_create_company() {
        let pool = pg_pool().await.expect("pool is expected");

        let address = create_addresses(&pool).await;

        let name = "sw";

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

    //
    // #[tokio::test]
    // async fn test_create_company_member() {
    //     let zip_code = 76236;
    //     let country = "sw";
    //     let region = "region";
    //     let city = "city";
    //     let district = Some("district");
    //     let street = "street";
    //     let building = "building";
    //     let apartment = "apartment";
    //
    //     let pool = pg_pool().await.expect("pool is expected");
    //
    //     let id = insert_addresses(
    //         &pool, zip_code, country, region, city, district, street, building, apartment,
    //     )
    //         .await
    //         .expect("user is created");
    //
    //     let address = get_addresses(&pool, id)
    //         .await
    //         .expect("user for given id is expected");
    //
    //     assert_eq!(zip_code, address.zip_code);
    //     assert_eq!(country, address.country);
    //     assert_eq!(region, address.region);
    //     assert_eq!(city, address.city);
    //     assert_eq!(district, address.district.as_ref().map(|x| x.as_ref()));
    //     assert_eq!(street, address.street);
    //     assert_eq!(building, address.building);
    //     assert_eq!(apartment, address.apartment);
    // }
}

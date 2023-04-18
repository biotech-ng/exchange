use sqlx::types::time::PrimitiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Address {
    pub id: Uuid,
    pub zip_code: i32,
    pub country: String,
    pub region: String,
    pub city: String,
    pub district: Option<String>,
    pub street: String,
    pub building: String,
    pub apartment: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub struct AddressInput<
    T1: AsRef<str>,
    T2: AsRef<str>,
    T3: AsRef<str>,
    T4: AsRef<str>,
    T5: AsRef<str>,
    T6: AsRef<str>,
    T7: AsRef<str>,
> {
    pub zip_code: i32,
    pub country: T1,
    pub region: T2,
    pub city: T3,
    pub district: Option<T4>,
    pub street: T5,
    pub building: T6,
    pub apartment: T7,
}

pub async fn insert_addresses<
    T1: AsRef<str>,
    T2: AsRef<str>,
    T3: AsRef<str>,
    T4: AsRef<str>,
    T5: AsRef<str>,
    T6: AsRef<str>,
    T7: AsRef<str>,
>(
    pool: &PgPool,
    address_input: AddressInput<T1, T2, T3, T4, T5, T6, T7>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
            r#"
                INSERT INTO addresses ( id, zip_code, country, region, city, district, street, building, apartment, created_at, updated_at )
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                RETURNING id
            "#,
            Uuid::new_v4(),
            address_input.zip_code,
            address_input.country.as_ref(),
            address_input.region.as_ref(),
            address_input.city.as_ref(),
            address_input.district.as_ref().map(|x| x.as_ref()),
            address_input.street.as_ref(),
            address_input.building.as_ref(),
            address_input.apartment.as_ref(),
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
}

pub async fn get_addresses(pool: &PgPool, id: Uuid) -> Result<Address, sqlx::Error> {
    sqlx::query_as!(
            Address,
            r#"
                SELECT id, zip_code, country, region, city, district, street, building, apartment, created_at, updated_at FROM addresses
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
    use crate::pg_pool;

    #[tokio::test]
    async fn test_create_addresses() {
        let zip_code = 76236;
        let country = "sw";
        let region = "region";
        let city = "city";
        let district = Some("district");
        let street = "street";
        let building = "building";
        let apartment = "apartment";

        let pool = pg_pool().await.expect("pool is expected");

        let address_input = AddressInput {
            zip_code,
            country,
            region,
            city,
            district,
            street,
            building,
            apartment,
        };

        let id = insert_addresses(&pool, address_input)
            .await
            .expect("user is created");

        let address = get_addresses(&pool, id)
            .await
            .expect("user for given id is expected");

        assert_eq!(zip_code, address.zip_code);
        assert_eq!(country, address.country);
        assert_eq!(region, address.region);
        assert_eq!(city, address.city);
        assert_eq!(district, address.district.as_ref().map(|x| x.as_ref()));
        assert_eq!(street, address.street);
        assert_eq!(building, address.building);
        assert_eq!(apartment, address.apartment);
    }
}

use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Phone {
    pub id: i64,
    pub company_id: Option<i64>,
    pub contact_id: Option<i64>,
    pub phone: Option<i64>,
    pub fax: bool,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

impl Phone {
    pub fn new() -> Self {
        Default::default()
    }

    async fn insert(pool: &Pool, phone: Phone) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO phones
                    (
                        company_id,
                        contact_id,
                        phone,
                        fax,
                        created_at,
                        updated_at
                    )
                    VALUES
                    (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5,
                        $6
                    )
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &phone.company_id,
                    &phone.contact_id,
                    &phone.phone,
                    &phone.fax,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn update_contacts(pool: &Pool, id: i64, fax: bool, phones: Vec<i64>) -> Result<()> {
        Phone::delete_contacts(pool, id, fax).await?;
        for value in phones {
            let mut phone = Phone::new();
            phone.contact_id = Some(id);
            phone.phone = Some(value);
            phone.fax = fax;
            Phone::insert(pool, phone).await?;
        }
        Ok(())
    }

    pub async fn update_companies(pool: &Pool, id: i64, fax: bool, phones: Vec<i64>) -> Result<()> {
        Phone::delete_companies(pool, id, fax).await?;
        for value in phones {
            let mut phone = Phone::new();
            phone.company_id = Some(id);
            phone.phone = Some(value);
            phone.fax = fax;
            Phone::insert(pool, phone).await?;
        }
        Ok(())
    }

    pub async fn delete_contacts(pool: &Pool, id: i64, fax: bool) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        phones
                    WHERE
                        contact_id = $1
                    AND
                        fax = $2
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id, &fax]).await?)
    }

    pub async fn delete_companies(pool: &Pool, id: i64, fax: bool) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        phones
                    WHERE
                        company_id = $1
                    AND
                        fax = $2
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id, &fax]).await?)
    }
}

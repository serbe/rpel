use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::RpelError;

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

    async fn insert(client: &Client, phone: Phone) -> Result<bool, RpelError> {
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
            .await?
            > 0)
    }

    pub async fn update_contacts(
        client: &Client,
        id: i64,
        fax: bool,
        phones: Vec<i64>,
    ) -> Result<(), RpelError> {
        Phone::delete_contacts(client, id, fax).await?;
        for value in phones {
            let mut phone = Phone::new();
            phone.contact_id = Some(id);
            phone.phone = Some(value);
            phone.fax = fax;
            Phone::insert(client, phone).await?;
        }
        Ok(())
    }

    pub async fn update_companies(
        client: &Client,
        id: i64,
        fax: bool,
        phones: Vec<i64>,
    ) -> Result<(), RpelError> {
        Phone::delete_companies(client, id, fax).await?;
        for value in phones {
            let mut phone = Phone::new();
            phone.company_id = Some(id);
            phone.phone = Some(value);
            phone.fax = fax;
            Phone::insert(client, phone).await?;
        }
        Ok(())
    }

    pub async fn delete_contacts(client: &Client, id: i64, fax: bool) -> Result<bool, RpelError> {
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
        Ok(client.execute(&stmt, &[&id, &fax]).await? > 0)
    }

    pub async fn delete_companies(client: &Client, id: i64, fax: bool) -> Result<bool, RpelError> {
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
        Ok(client.execute(&stmt, &[&id, &fax]).await? > 0)
    }
}

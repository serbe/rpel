use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Email {
    pub id: i64,
    pub company_id: Option<i64>,
    pub contact_id: Option<i64>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

impl Email {
    pub fn new() -> Self {
        Default::default()
    }

    async fn insert(pool: &Pool, email: Email) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO emails
                    (
                        company_id,
                        contact_id,
                        email,
                        created_at,
                        updated_at
                    )
                    VALUES
                    (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5
                    )
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &email.company_id,
                    &email.contact_id,
                    &email.email,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn update_contacts(pool: &Pool, id: i64, emails: Vec<String>) -> Result<()> {
        Email::delete_contacts(pool, id).await?;
        for value in emails {
            let mut email = Email::new();
            email.contact_id = Some(id);
            email.email = Some(value);
            Email::insert(pool, email).await?;
        }
        Ok(())
    }

    pub async fn update_companies(pool: &Pool, id: i64, emails: Vec<String>) -> Result<()> {
        Email::delete_companies(pool, id).await?;
        for value in emails {
            let mut email = Email::new();
            email.company_id = Some(id);
            email.email = Some(value);
            Email::insert(pool, email).await?;
        }
        Ok(())
    }

    pub async fn delete_contacts(pool: &Pool, id: i64) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        emails
                    WHERE
                        contact_id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }

    pub async fn delete_companies(pool: &Pool, id: i64) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        emails
                    WHERE
                        company_id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

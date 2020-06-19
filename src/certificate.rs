use chrono::{Local, NaiveDate, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::RpelError;

#[derive(Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(default)]
    pub id: i64,
    pub num: Option<String>,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
    pub cert_date: Option<NaiveDate>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct CertificateList {
    pub id: i64,
    pub num: Option<String>,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub cert_date: Option<String>,
    pub note: Option<String>,
}

impl Certificate {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(client: &Client, id: i64) -> Result<Certificate, RpelError> {
        let mut certificate = Certificate::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        num,
                        contact_id,
                        company_id,
                        cert_date,
                        note,
                        created_at,
                        updated_at
                    FROM
                        certificates
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        certificate.id = id;
        certificate.num = row.get(0);
        certificate.contact_id = row.get(1);
        certificate.company_id = row.get(2);
        certificate.cert_date = row.get(3);
        certificate.note = row.get(4);
        certificate.created_at = row.get(5);
        certificate.updated_at = row.get(6);
        Ok(certificate)
    }

    pub async fn insert(
        client: &Client,
        certificate: Certificate,
    ) -> Result<Certificate, RpelError> {
        let mut certificate = certificate;
        let stmt = client
            .prepare(
                "
                    INSERT INTO certificates
                    (
                        num,
                        contact_id,
                        company_id,
                        cert_date,
                        note,
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
                        $6,
                        $7
                    )
                    RETURNING
                        id
                ",
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[
                    &certificate.num,
                    &certificate.contact_id,
                    &certificate.company_id,
                    &certificate.cert_date,
                    &certificate.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        certificate.id = row.get(0);
        Ok(certificate)
    }

    pub async fn update(client: &Client, certificate: Certificate) -> Result<u64, RpelError> {
        let stmt = client
            .prepare(
                "
                    UPDATE certificates SET
                        num = $2,
                        contact_id = $3,
                        company_id = $4,
                        cert_date = $5,
                        note = $6,
                        updated_at = $7
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &certificate.id,
                    &certificate.num,
                    &certificate.contact_id,
                    &certificate.company_id,
                    &certificate.cert_date,
                    &certificate.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<u64, RpelError> {
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        certificates
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl CertificateList {
    pub async fn get_all(client: &Client) -> Result<Vec<CertificateList>, RpelError> {
        let mut certificates = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.id,
                        c.num,
                        c.contact_id,
                        p.name AS contact_name,
                        c.company_id,
                        co.name AS company_name,
                        c.cert_date,
                        c.note
                    FROM
                        certificates AS c
                    LEFT JOIN
                        contacts AS p ON c.contact_id = p.id
                    LEFT JOIN
                        companies AS co ON c.company_id = co.id
                    GROUP BY
                        c.id,
                        p.name,
                        co.name
                    ORDER BY
                        num ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            let date: Option<NaiveDate> = row.get(6);
            certificates.push(CertificateList {
                id: row.get("id"),
                num: row.get("num"),
                contact_id: row.try_get(2)?,
                contact_name: row.try_get(3)?,
                company_id: row.try_get(4)?,
                company_name: row.try_get(5)?,
                cert_date: if let Some(d) = date {
                    Some(d.format("%Y-%m-%d").to_string())
                } else {
                    None
                },
                note: row.try_get(7)?,
            });
        }
        Ok(certificates)
    }
}

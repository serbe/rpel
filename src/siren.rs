use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::RpelError;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Siren {
    #[serde(default)]
    pub id: i64,
    pub num_id: Option<i64>,
    pub num_pass: Option<String>,
    pub siren_type_id: Option<i64>,
    pub address: Option<String>,
    pub radio: Option<String>,
    pub desk: Option<String>,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub stage: Option<i64>,
    pub own: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SirenList {
    pub id: i64,
    pub siren_type_name: Option<String>,
    pub address: Option<String>,
    pub contact_name: Option<String>,
    pub phones: Vec<i64>,
}

impl Siren {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(client: &Client, id: i64) -> Result<Siren, RpelError> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        num_id,
                        num_pass,
                        siren_type_id,
                        address,
                        radio,
                        desk,
                        contact_id,
                        company_id,
                        latitude,
                        longitude,
                        stage,
                        own,
                        note,
                        created_at,
                        updated_at
                    FROM
                        sirens
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let siren = Siren {
            id,
            num_id: row.try_get(0)?,
            num_pass: row.try_get(1)?,
            siren_type_id: row.try_get(2)?,
            address: row.try_get(3)?,
            radio: row.try_get(4)?,
            desk: row.try_get(5)?,
            contact_id: row.try_get(6)?,
            company_id: row.try_get(7)?,
            latitude: row.try_get(8)?,
            longitude: row.try_get(9)?,
            stage: row.try_get(10)?,
            own: row.try_get(11)?,
            note: row.try_get(12)?,
            created_at: row.try_get(13)?,
            updated_at: row.try_get(14)?,
        };
        Ok(siren)
    }

    pub async fn insert(client: &Client, siren: Siren) -> Result<Siren, RpelError> {
        let mut siren = siren;
        let stmt = client
            .prepare(
                "
                    INSERT INTO sirens
                    (
                        num_id,
                        num_pass,
                        siren_type_id,
                        address,
                        radio,
                        desk,
                        contact_id,
                        company_id,
                        latitude,
                        longitude,
                        stage,
                        own,
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
                        $7,
                        $8,
                        $9,
                        $10,
                        $11,
                        $12,
                        $13,
                        $14,
                        $15
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
                    &siren.num_id,
                    &siren.num_pass,
                    &siren.siren_type_id,
                    &siren.address,
                    &siren.radio,
                    &siren.desk,
                    &siren.contact_id,
                    &siren.company_id,
                    &siren.latitude,
                    &siren.longitude,
                    &siren.stage,
                    &siren.own,
                    &siren.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        siren.id = row.get(0);
        Ok(siren)
    }

    pub async fn update(client: &Client, siren: Siren) -> Result<u64, RpelError> {
        let stmt = client
            .prepare(
                "
                    UPDATE sirens SET
                        num_id = $2,
                        num_pass = $3,
                        siren_type_id = $4,
                        address = $5,
                        radio = $6,
                        desk = $7,
                        contact_id = $8,
                        company_id = $9,
                        latitude = $10,
                        longitude = $11,
                        stage = $12,
                        own = $13,
                        note = $14,
                        updated_at = $15
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &siren.id,
                    &siren.num_id,
                    &siren.num_pass,
                    &siren.siren_type_id,
                    &siren.address,
                    &siren.radio,
                    &siren.desk,
                    &siren.contact_id,
                    &siren.company_id,
                    &siren.latitude,
                    &siren.longitude,
                    &siren.stage,
                    &siren.own,
                    &siren.note,
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
                        sirens
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl SirenList {
    pub async fn get_all(client: &Client) -> Result<Vec<SirenList>, RpelError> {
        let mut sirens = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        s.id,
                        t.name AS siren_type_name,
                        s.address,
                        c.name AS contact_name,
                        array_remove(array_agg(ph.phone), NULL) AS phones
                    FROM
                        sirens AS s
                    LEFT JOIN
                        siren_types AS t ON s.siren_type_id = t.id
                    LEFT JOIN
                        contacts AS c ON s.contact_id = c.id
                    LEFT JOIN
                        phones AS ph ON s.contact_id = ph.contact_id AND ph.fax = false
                    GROUP BY
                        s.id,
                        t.id,
                        c.id
                    ORDER BY
                        t.name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            sirens.push(SirenList {
                id: row.try_get(0)?,
                siren_type_name: row.try_get(1)?,
                address: row.try_get(2)?,
                contact_name: row.try_get(3)?,
                phones: row.try_get(4)?,
            });
        }
        Ok(sirens)
    }
}

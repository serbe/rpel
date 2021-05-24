use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{error::RpelError, RpelPool};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SirenType {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub radius: Option<i64>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SirenTypeList {
    pub id: i64,
    pub name: Option<String>,
    pub radius: Option<i64>,
    pub note: Option<String>,
}

impl SirenType {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(pool: &RpelPool, id: i64) -> Result<SirenType, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        name,
                        radius,
                        note,
                        created_at,
                        updated_at
                    FROM
                        siren_types
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let siren_type = SirenType {
            id,
            name: row.try_get(0)?,
            radius: row.try_get(1)?,
            note: row.try_get(2)?,
            created_at: row.try_get(3)?,
            updated_at: row.try_get(4)?,
        };
        Ok(siren_type)
    }

    pub async fn insert(pool: &RpelPool, siren_type: SirenType) -> Result<SirenType, RpelError> {
        let mut siren_type = siren_type;
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO siren_types
                    (
                        name,
                        radius,
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
                        $5
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
                    &siren_type.name,
                    &siren_type.radius,
                    &siren_type.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        siren_type.id = row.get(0);
        Ok(siren_type)
    }

    pub async fn update(pool: &RpelPool, siren_type: SirenType) -> Result<u64, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    UPDATE siren_types SET
                        name = $2,
                        radius = $3,
                        note = $4,
                        updated_at = $5
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &siren_type.id,
                    &siren_type.name,
                    &siren_type.radius,
                    &siren_type.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(pool: &RpelPool, id: i64) -> Result<u64, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        siren_types
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl SirenTypeList {
    pub async fn get_all(pool: &RpelPool) -> Result<Vec<SirenTypeList>, RpelError> {
        let mut siren_types = Vec::new();
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        id,
                        name,
                        radius,
                        note
                    FROM
                        siren_types
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            siren_types.push(SirenTypeList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                radius: row.try_get(2)?,
                note: row.try_get(3)?,
            });
        }
        Ok(siren_types)
    }
}

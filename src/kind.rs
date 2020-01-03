use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Kind {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct KindList {
    pub id: i64,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub note: Option<String>,
}

impl Kind {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(pool: &Pool, id: i64) -> Result<Kind> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        name,
                        short_name,
                        note,
                        created_at,
                        updated_at
                    FROM
                        kinds
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let kind = Kind {
            id,
            name: row.get(0),
            short_name: row.get(1),
            note: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        };
        Ok(kind)
    }

    pub async fn insert(pool: &Pool, kind: Kind) -> Result<Kind> {
        let mut kind = kind;
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO kinds
                    (
                        name,
                        short_name,
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
                    &kind.name,
                    &kind.note,
                    &kind.short_name,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        kind.id = row.get(0);
        Ok(kind)
    }

    pub async fn update(pool: &Pool, kind: Kind) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    UPDATE kinds SET
                        name = $2,
                        short_name = $3,
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
                    &kind.id,
                    &kind.name,
                    &kind.short_name,
                    &kind.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(pool: &Pool, id: i64) -> Result<u64> {
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        kinds
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl KindList {
    pub async fn get_all(pool: &Pool) -> Result<Vec<KindList>> {
        let mut kinds = Vec::new();
        let mut client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        id,
                        name,
                        short_name,
                        note
                    FROM
                        kinds
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            kinds.push(KindList {
                id: row.get(0),
                name: row.get(1),
                short_name: row.get(2),
                note: row.get(3),
            });
        }
        Ok(kinds)
    }
}

use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

use crate::error::RpelError;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Scope {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScopeList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Scope {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(pool: &Pool<tokio_postgres::NoTls>, id: i64) -> Result<Scope, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        name,
                        note,
                        created_at,
                        updated_at
                    FROM
                        scopes
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let scope = Scope {
            id,
            name: row.try_get(0)?,
            note: row.try_get(1)?,
            created_at: row.try_get(2)?,
            updated_at: row.try_get(3)?,
        };
        Ok(scope)
    }

    pub async fn insert(
        pool: &Pool<tokio_postgres::NoTls>,
        scope: Scope,
    ) -> Result<Scope, RpelError> {
        let mut scope = scope;
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO scopes
                    (
                        name,
                        note,
                        created_at,
                        updated_at
                    )
                    VALUES
                    (
                        $1,
                        $2,
                        $3,
                        $4
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
                    &scope.name,
                    &scope.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        scope.id = row.get(0);
        Ok(scope)
    }

    pub async fn update(
        pool: &Pool<tokio_postgres::NoTls>,
        scope: Scope,
    ) -> Result<u64, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    UPDATE scopes SET
                        name = $2,
                        note = $3,
                        updated_at = $4
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &scope.id,
                    &scope.name,
                    &scope.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(pool: &Pool<tokio_postgres::NoTls>, id: i64) -> Result<u64, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        scopes
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl ScopeList {
    pub async fn get_all(pool: &Pool<tokio_postgres::NoTls>) -> Result<Vec<ScopeList>, RpelError> {
        let mut scopes = Vec::new();
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        id,
                        name,
                        note
                    FROM
                        scopes
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            scopes.push(ScopeList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                note: row.try_get(2)?,
            });
        }
        Ok(scopes)
    }
}

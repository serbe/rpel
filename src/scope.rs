use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct ScopeList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Scope {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(client: &Client, id: i64) -> Result<Scope> {
        // let client = client.get().await?;
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
            name: row.get(0),
            note: row.get(1),
            created_at: row.get(2),
            updated_at: row.get(3),
        };
        Ok(scope)
    }

    pub async fn insert(client: &Client, scope: Scope) -> Result<Scope> {
        let mut scope = scope;
        // let client = client.get().await?;
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

    pub async fn update(client: &Client, scope: Scope) -> Result<u64> {
        // let client = client.get().await?;
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

    pub async fn delete(client: &Client, id: i64) -> Result<u64> {
        // let client = client.get().await?;
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
    pub async fn get_all(client: &Client) -> Result<Vec<ScopeList>> {
        let mut scopes = Vec::new();
        // let client = client.get().await?;
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
                id: row.get(0),
                name: row.get(1),
                note: row.get(2),
            });
        }
        Ok(scopes)
    }
}

use serde::{Deserialize, Serialize};

use crate::{error::RpelError, RpelPool};

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectItem {
    pub id: i64,
    pub name: Option<String>,
}

async fn select_name(pool: &RpelPool, name: &str) -> Result<Vec<SelectItem>, RpelError> {
    let client = pool.get().await?;
    let stmt = client
        .prepare(
            format!(
                "
                SELECT
                    id,
                    name
                FROM
                    {name}
                ORDER BY
                    name ASC
            "
            )
            .as_str(),
        )
        .await?;
    let mut select_list = Vec::new();
    for row in client.query(&stmt, &[]).await? {
        select_list.push(SelectItem {
            id: row.try_get(0)?,
            name: row.try_get(1)?,
        })
    }
    Ok(select_list)
}

impl SelectItem {
    pub async fn company_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "companies").await
    }

    pub async fn contact_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "contacts").await
    }

    pub async fn department_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "departments").await
    }

    pub async fn kind_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "kinds").await
    }

    pub async fn post_all(pool: &RpelPool, go: bool) -> Result<Vec<SelectItem>, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
        SELECT
            id,
            name
        FROM
            posts
        WHERE
            go = $1
        ORDER BY
            name ASC
    ",
            )
            .await?;
        let mut posts = Vec::new();
        for row in client.query(&stmt, &[&go]).await? {
            posts.push(SelectItem {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
            });
        }
        Ok(posts)
    }

    pub async fn rank_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "ranks").await
    }

    pub async fn scope_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "scopes").await
    }

    pub async fn siren_type_all(pool: &RpelPool) -> Result<Vec<SelectItem>, RpelError> {
        select_name(pool, "siren_types").await
    }
}

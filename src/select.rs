use anyhow::Result;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SelectItem {
    pub id: i64,
    pub name: Option<String>,
}

async fn select_name(client: &Client, name: &str) -> Result<Vec<SelectItem>> {
    let stmt = client
        .prepare(
            format!(
                "
                SELECT
                    id,
                    name
                FROM
                    {}
                ORDER BY
                    name ASC
            ",
                name
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
    pub async fn company_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "companies").await?)
    }

    pub async fn contact_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "contacts").await?)
    }

    pub async fn department_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "departments").await?)
    }

    pub async fn kind_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "kinds").await?)
    }

    pub async fn post_all(client: &Client, go: bool) -> Result<Vec<SelectItem>> {
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

    pub async fn rank_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "ranks").await?)
    }

    pub async fn scope_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "scopes").await?)
    }

    pub async fn siren_type_all(client: &Client) -> Result<Vec<SelectItem>> {
        Ok(select_name(client, "siren_types").await?)
    }
}

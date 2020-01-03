use anyhow::Result;
use deadpool_postgres::Pool;
use serde::Serialize;

#[derive(Serialize)]
pub struct SelectItem {
    pub id: i64,
    pub name: Option<String>,
}

async fn select_name(pool: &Pool, name: &str) -> Result<Vec<SelectItem>> {
    let mut client = pool.get().await?;
    let stmt = client
        .prepare(
            "
    SELECT
        id,
        name
    FROM
        $1
    ORDER BY
        name ASC
",
        )
        .await?;
    let mut select_list = Vec::new();
    for row in client.query(&stmt, &[&(name.to_string())]).await? {
        select_list.push(SelectItem {
            id: row.get(0),
            name: row.get(1),
        })
    }
    Ok(select_list)
}

impl SelectItem {
    pub async fn company_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "companies").await?)
    }

    pub async fn contact_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "contacts").await?)
    }

    pub async fn department_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "departments").await?)
    }

    pub async fn kind_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "kinds").await?)
    }

    pub async fn post_all(pool: &Pool, go: bool) -> Result<Vec<SelectItem>> {
        let mut client = pool.get().await?;
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
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(posts)
    }

    pub async fn rank_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "ranks").await?)
    }

    pub async fn scope_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "scopes").await?)
    }

    pub async fn siren_type_all(pool: &Pool) -> Result<Vec<SelectItem>> {
        Ok(select_name(pool, "siren_types").await?)
    }
}

use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::RpelError;

#[derive(Default, Deserialize, Serialize)]
pub struct Post {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub go: bool,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct PostList {
    pub id: i64,
    pub name: Option<String>,
    pub go: bool,
    pub note: Option<String>,
}

impl Post {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(client: &Client, id: i64) -> Result<Post, RpelError> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        name,
                        go,
                        note,
                        created_at,
                        updated_at
                    FROM
                        posts
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let post = Post {
            id,
            name: row.try_get(0)?,
            go: row.try_get(1)?,
            note: row.try_get(2)?,
            created_at: row.try_get(3)?,
            updated_at: row.try_get(4)?,
        };
        Ok(post)
    }

    pub async fn insert(client: &Client, post: Post) -> Result<Post, RpelError> {
        let mut post = post;
        let stmt = client
            .prepare(
                "
                    INSERT INTO posts
                    (
                        name,
                        go,
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
                    &post.name,
                    &post.go,
                    &post.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        post.id = row.get(0);
        Ok(post)
    }

    pub async fn update(client: &Client, post: Post) -> Result<u64, RpelError> {
        let stmt = client
            .prepare(
                "
                    UPDATE posts SET
                        name = $2,
                        go = $3,
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
                    &post.id,
                    &post.name,
                    &post.go,
                    &post.note,
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
                        posts
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl PostList {
    pub async fn get_all(client: &Client) -> Result<Vec<PostList>, RpelError> {
        let mut posts = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        id,
                        name,
                        go,
                        note
                    FROM
                        posts
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            posts.push(PostList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                go: row.try_get(2)?,
                note: row.try_get(3)?,
            });
        }
        Ok(posts)
    }
}

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{error::RpelError, RpelPool};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(default)]
    pub id: i64,
    pub name: String,
    pub key: String,
    pub role: i64,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserList {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub role: i64,
}

impl User {
    pub async fn get(pool: &RpelPool, id: i64) -> Result<User, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        name,
                        key,
                        role,
                        created_at,
                        updated_at
                    FROM
                        users
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let user = User {
            id,
            name: row.try_get(0)?,
            key: row.try_get(1)?,
            role: row.try_get(2)?,
            created_at: row.try_get(3)?,
            updated_at: row.try_get(4)?,
        };
        Ok(user)
    }

    pub async fn insert(pool: &RpelPool, user: User) -> Result<User, RpelError> {
        let mut user = user;
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO users
                    (
                        name,
                        key,
                        role,
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
                    &user.name,
                    &user.key,
                    &user.role,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        user.id = row.get(0);
        Ok(user)
    }

    pub async fn update(pool: &RpelPool, user: User) -> Result<u64, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    UPDATE users SET
                        name = $2,
                        key = $3,
                        role = $4,
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
                    &user.id,
                    &user.name,
                    &user.key,
                    &user.role,
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
                        users
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl UserList {
    pub async fn get_all(pool: &RpelPool) -> Result<Vec<UserList>, RpelError> {
        let mut users = Vec::new();
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        id,
                        name,
                        key,
                        role
                    FROM
                        users
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            users.push(UserList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                key: row.try_get(2)?,
                role: row.try_get(3)?,
            });
        }
        Ok(users)
    }
}

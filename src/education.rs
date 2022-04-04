use chrono::{Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{error::RpelError, RpelPool};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Education {
    #[serde(default)]
    pub id: i64,
    pub contact_id: Option<i64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub post_id: Option<i64>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EducationList {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub start_str: Option<String>,
    pub end_str: Option<String>,
    pub post_id: Option<i64>,
    pub post_name: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EducationShort {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub start_date: Option<NaiveDate>,
}

impl Education {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(pool: &RpelPool, id: i64) -> Result<Education, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        contact_id,
                        start_date,
                        end_date,
                        post_id,
                        note,
                        created_at,
                        updated_at
                    FROM
                        educations
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let education = Education {
            id,
            contact_id: row.try_get(0)?,
            start_date: row.try_get(1)?,
            end_date: row.try_get(2)?,
            post_id: row.try_get(3)?,
            note: row.try_get(4)?,
            created_at: row.try_get(5)?,
            updated_at: row.try_get(6)?,
        };
        Ok(education)
    }

    pub async fn insert(pool: &RpelPool, education: Education) -> Result<Education, RpelError> {
        let mut education = education;
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    INSERT INTO educations
                    (
                        contact_id,
                        start_date,
                        end_date,
                        post_id,
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
                        $7
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
                    &education.contact_id,
                    &education.start_date,
                    &education.end_date,
                    &education.post_id,
                    &education.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        education.id = row.get(0);
        Ok(education)
    }

    pub async fn update(pool: &RpelPool, education: Education) -> Result<u64, RpelError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    UPDATE educations SET
                        contact_id = $2,
                        start_date = $3,
                        end_date = $4,
                        post_id = $5,
                        note = $6,
                        updated_at = $7
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &education.id,
                    &education.contact_id,
                    &education.start_date,
                    &education.end_date,
                    &education.post_id,
                    &education.note,
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
                        educations
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl EducationList {
    pub async fn get_all(pool: &RpelPool) -> Result<Vec<EducationList>, RpelError> {
        let mut educations = Vec::new();
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        e.id,
                        e.contact_id,
                        c.name AS contact_name,
                        e.start_date,
                        e.end_date,
                        e.post_id,
                        p.name AS post_name,
                        e.note
                    FROM
                        educations AS e
                    LEFT JOIN
                        contacts AS c ON c.id = e.contact_id
                    LEFT JOIN
                        posts AS p ON p.id = e.post_id
                    ORDER BY
                        start_date DESC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            let start_str: Option<NaiveDate> = row.get(3);
            let end_str: Option<NaiveDate> = row.get(4);
            educations.push(EducationList {
                id: row.try_get(0)?,
                contact_id: row.try_get(1)?,
                contact_name: row.try_get(2)?,
                start_date: row.try_get(3)?,
                end_date: row.try_get(4)?,
                start_str: start_str.map(|d| d.format("%Y-%m-%d").to_string()),
                end_str: end_str.map(|d| d.format("%Y-%m-%d").to_string()),
                post_id: row.try_get(5)?,
                post_name: row.try_get(6)?,
                note: row.try_get(7)?,
            });
        }
        Ok(educations)
    }
}

impl EducationShort {
    pub async fn get_near(pool: &RpelPool) -> Result<Vec<EducationShort>, RpelError> {
        let mut educations = Vec::new();
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "
                    SELECT
                        e.id,
                        e.contact_id,
                        p.name AS contact_name,
                        c.id AS company_id,
                        c.name AS company_name,
                        e.start_date
                    FROM
                        educations AS e
                    LEFT JOIN
                        contacts AS p ON p.id = e.contact_id
                    LEFT JOIN
                        companies AS c ON c.id = p.company_id
                    WHERE
                        e.start_date > TIMESTAMP 'now'::timestamp - '1 month'::interval
                    ORDER BY
                        start_date ASC
                    LIMIT 10
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            educations.push(EducationShort {
                id: row.try_get(0)?,
                contact_id: row.try_get(1)?,
                contact_name: row.try_get(2)?,
                company_id: row.try_get(3)?,
                company_name: row.try_get(4)?,
                start_date: row.try_get(5)?,
            });
        }
        Ok(educations)
    }
}

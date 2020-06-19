use chrono::{Local, NaiveDate, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::email::Email;
use crate::error::RpelError;
use crate::phone::Phone;

#[derive(Default, Deserialize, Serialize)]
pub struct Contact {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub company_id: Option<i64>,
    pub department_id: Option<i64>,
    pub post_id: Option<i64>,
    pub post_go_id: Option<i64>,
    pub rank_id: Option<i64>,
    pub birthday: Option<NaiveDate>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
    pub emails: Vec<String>,
    pub phones: Vec<i64>,
    pub faxes: Vec<i64>,
    #[serde(skip_deserializing)]
    pub educations: Vec<NaiveDate>,
}

#[derive(Deserialize, Serialize)]
pub struct ContactList {
    pub id: i64,
    pub name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub post_name: Option<String>,
    pub phones: Vec<i64>,
    pub faxes: Vec<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct ContactShort {
    pub id: i64,
    pub name: Option<String>,
    pub department_name: Option<String>,
    pub post_name: Option<String>,
    pub post_go_name: Option<String>,
}

impl Contact {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(client: &Client, id: i64) -> Result<Contact, RpelError> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.name,
                        c.company_id,
                        c.department_id,
                        c.post_id,
                        c.post_go_id,
                        c.rank_id,
                        c.birthday,
                        c.note,
                        c.created_at,
                        c.updated_at,
                        array_remove(array_agg(e.email), NULL) AS emails,
                        array_remove(array_agg(ph.phone), NULL) AS phones,
                        array_remove(array_agg(f.phone), NULL) AS faxes,
                        array_remove(array_agg(ed.start_date), NULL) AS educations
                    FROM
                        contacts AS c
                    LEFT JOIN
                        emails AS e ON c.id = e.contact_id
                    LEFT JOIN
                        phones AS ph ON c.id = ph.contact_id AND ph.fax = false
                    LEFT JOIN
                        phones AS f ON c.id = f.contact_id AND f.fax = true
                    LEFT JOIN
                        educations AS ed ON c.id = ed.contact_id
                    WHERE
                        c.id = $1
                    GROUP BY
                        c.id
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let contact = Contact {
            id,
            name: row.try_get("name")?,
            company_id: row.try_get("company_id")?,
            department_id: row.try_get("department_id")?,
            post_id: row.try_get("post_id")?,
            post_go_id: row.try_get("post_go_id")?,
            rank_id: row.try_get("rank_id")?,
            birthday: row.try_get("birthday")?,
            note: row.try_get("note")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            emails: row.try_get("emails")?,
            phones: row.try_get("phones")?,
            faxes: row.try_get("faxes")?,
            educations: row.try_get("educations")?,
        };
        Ok(contact)
    }

    pub async fn insert(client: &Client, contact: Contact) -> Result<Contact, RpelError> {
        let mut contact = contact;
        let stmt = client
            .prepare(
                "
                    INSERT INTO contacts
                    (
                        name,
                        company_id,
                        department_id,
                        post_id,
                        post_go_id,
                        rank_id,
                        birthday,
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
                        $7,
                        $8,
                        $9,
                        $10
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
                    &contact.name,
                    &contact.company_id,
                    &contact.department_id,
                    &contact.post_id,
                    &contact.post_go_id,
                    &contact.rank_id,
                    &contact.birthday,
                    &contact.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        contact.id = row.get(0);
        Email::update_contacts(client, contact.id, contact.emails.clone()).await?;
        Phone::update_contacts(client, contact.id, false, contact.phones.clone()).await?;
        Phone::update_contacts(client, contact.id, true, contact.faxes.clone()).await?;
        Ok(contact)
    }

    pub async fn update(client: &Client, contact: Contact) -> Result<u64, RpelError> {
        let stmt = client
            .prepare(
                "
                    UPDATE contacts SET
                        name = $2,
                        company_id = $3,
                        department_id = $4,
                        post_id = $5,
                        post_go_id = $6,
                        rank_id = $7,
                        birthday = $8,
                        note = $9,
                        updated_at = $10
                    WHERE
                        id = $1
                    ",
            )
            .await?;
        Email::update_contacts(client, contact.id, contact.emails).await?;
        Phone::update_contacts(client, contact.id, false, contact.phones).await?;
        Phone::update_contacts(client, contact.id, true, contact.faxes).await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &contact.id,
                    &contact.name,
                    &contact.company_id,
                    &contact.department_id,
                    &contact.post_id,
                    &contact.post_go_id,
                    &contact.rank_id,
                    &contact.birthday,
                    &contact.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<u64, RpelError> {
        Phone::delete_contacts(client, id, true).await?;
        Phone::delete_contacts(client, id, false).await?;
        Email::delete_contacts(client, id).await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        contacts
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl ContactList {
    pub async fn get_all(client: &Client) -> Result<Vec<ContactList>, RpelError> {
        let mut contacts = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.id,
                        c.name,
                        co.id AS company_id,
                        co.name AS company_name,
                        po.name AS post_name,
                        array_remove(array_agg(ph.phone), NULL) AS phones,
                        array_remove(array_agg(f.phone), NULL) AS faxes
                    FROM
                        contacts AS c
                    LEFT JOIN
                        companies AS co ON c.company_id = co.id
                    LEFT JOIN
                        posts AS po ON c.post_id = po.id
                    LEFT JOIN
                        phones AS ph ON c.id = ph.contact_id AND ph.fax = false
                    LEFT JOIN
                        phones AS f ON c.id = f.contact_id AND f.fax = true
                    GROUP BY
                        c.id,
                        co.id,
                        po.name
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            contacts.push(ContactList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                company_id: row.try_get(2)?,
                company_name: row.try_get(3)?,
                post_name: row.try_get(4)?,
                phones: row.try_get(5)?,
                faxes: row.try_get(6)?,
            });
        }
        Ok(contacts)
    }
}

impl ContactShort {
    pub async fn get_by_company(
        client: &Client,
        company_id: i64,
    ) -> Result<Vec<ContactShort>, RpelError> {
        let mut contacts = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.id,
                        c.name,
                        d.name AS department_name,
                        p.name AS post_name,
                        pg.name AS post_go_name
                    FROM
                        contacts AS c
                    LEFT JOIN
                        departments AS d ON c.department_id = d.id
                    LEFT JOIN
                        posts AS p ON c.post_id = p.id AND p.go = false
                    LEFT JOIN
                        posts AS pg ON c.post_go_id = p.id AND p.go = true
                    WHERE
                        c.company_id = $1
                ",
            )
            .await?;
        for row in client.query(&stmt, &[&company_id]).await? {
            contacts.push(ContactShort {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                department_name: row.try_get(2)?,
                post_name: row.try_get(3)?,
                post_go_name: row.try_get(4)?,
            });
        }
        Ok(contacts)
    }
}

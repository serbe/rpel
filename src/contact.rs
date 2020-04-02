use anyhow::{Context, Result};
use chrono::{Local, NaiveDate, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::email::Email;
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
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
    pub educations: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct ContactList {
    pub id: i64,
    pub name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub post_name: Option<String>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
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

    pub async fn get(client: &Client, id: i64) -> Result<Contact> {
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
                        array_agg(DISTINCT e.email) AS emails,
                        array_agg(DISTINCT ph.phone) AS phones,
                        array_agg(DISTINCT f.phone) AS faxes,
                        array_agg(DISTINCT ed.start_date) AS educations
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
            .await.with_context(|| format!("Failed prepare get contact {}", &id))?;
        let row = client.query_one(&stmt, &[&id]).await.with_context(|| format!("Failed query one get contact {}", &id))?;
        println!("{} {:?}", row.len(), row.columns());
        let contact = Contact {
            id,
            name: row.get(0),
            company_id: row.get(1),
            department_id: row.get(2),
            post_id: row.get(3),
            post_go_id: row.get(4),
            rank_id: row.get(5),
            birthday: row.get(6),
            note: row.get(7),
            created_at: row.get(8),
            updated_at: row.get(9),
            emails: row.get(10),
            phones: row.get(11),
            faxes: row.get(12),
            educations: row.get(13),
        };
        Ok(contact)
    }

    pub async fn insert(client: &Client, contact: Contact) -> Result<Contact> {
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
        if let Some(emails) = contact.emails.clone() {
            Email::update_contacts(client, contact.id, emails).await?;
        }
        if let Some(phones) = contact.phones.clone() {
            Phone::update_contacts(client, contact.id, false, phones).await?;
        }
        if let Some(faxes) = contact.faxes.clone() {
            Phone::update_contacts(client, contact.id, true, faxes).await?;
        }
        Ok(contact)
    }

    pub async fn update(client: &Client, contact: Contact) -> Result<u64> {
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
        if let Some(emails) = contact.emails.clone() {
            Email::update_contacts(client, contact.id, emails).await?;
        }
        if let Some(phones) = contact.phones.clone() {
            Phone::update_contacts(client, contact.id, false, phones).await?;
        }
        if let Some(faxes) = contact.faxes.clone() {
            Phone::update_contacts(client, contact.id, true, faxes).await?;
        }
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

    pub async fn delete(client: &Client, id: i64) -> Result<u64> {
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
    pub async fn get_all(client: &Client) -> Result<Vec<ContactList>> {
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
                        array_agg(DISTINCT ph.phone) AS phones,
                        array_agg(DISTINCT f.phone) AS faxes
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
                id: row.get(0),
                name: row.get(1),
                company_id: row.get(2),
                company_name: row.get(3),
                post_name: row.get(4),
                phones: row.get(5),
                faxes: row.get(6),
            });
        }
        Ok(contacts)
    }
}

impl ContactShort {
    pub async fn get_by_company(client: &Client, company_id: i64) -> Result<Vec<ContactShort>> {
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
            .await.with_context(|| format!("Failed prepare get_by_company {}", &company_id))?;
        for row in client.query(&stmt, &[&company_id]).await? {
            contacts.push(ContactShort {
                id: row.get(0),
                name: row.get(1),
                department_name: row.get(2),
                post_name: row.get(3),
                post_go_name: row.get(4),
            });
        }
        Ok(contacts)
    }
}

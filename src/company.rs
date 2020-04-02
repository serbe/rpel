use anyhow::Result;
use chrono::{Local, NaiveDate, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::contact::ContactShort;
use crate::email::Email;
use crate::phone::Phone;
use crate::practice::PracticeList;

#[derive(Default, Deserialize, Serialize)]
pub struct Company {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub scope_id: Option<i64>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
    #[serde(skip_deserializing)]
    pub practices: Option<Vec<PracticeList>>,
    #[serde(skip_deserializing)]
    pub contacts: Option<Vec<ContactShort>>,
}

#[derive(Deserialize, Serialize)]
pub struct CompanyList {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub scope_name: Option<String>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
    pub practices: Option<Vec<NaiveDate>>,
}

impl Company {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get(client: &Client, id: i64) -> Result<Company> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.name,
                        c.address,
                        c.scope_id,
                        c.note,
                        c.created_at,
                        c.updated_at,
                        array_agg(DISTINCT e.email) AS emails,
                        array_agg(DISTINCT ph.phone) AS phones,
                        array_agg(DISTINCT f.phone) AS faxes
                    FROM
                        companies AS c
                    LEFT JOIN
                        emails AS e ON c.id = e.company_id
                    LEFT JOIN
                        phones AS ph ON c.id = ph.company_id AND ph.fax = false
                    LEFT JOIN
                        phones AS f ON c.id = f.company_id AND f.fax = true
                    WHERE
                        c.id = $1
                    GROUP BY
                        c.id
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let practices = PracticeList::get_by_company(client, id).await.ok();
        let contacts = ContactShort::get_by_company(client, id).await.ok();
        let company = Company {
            id,
            name: row.get(0),
            address: row.get(1),
            scope_id: row.get(2),
            note: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
            emails: row.get(6),
            phones: row.get(7),
            faxes: row.get(8),
            practices,
            contacts,
        };
        Ok(company)
    }

    pub async fn insert(client: &Client, company: Company) -> Result<Company> {
        let mut company = company;
        let stmt = client
            .prepare(
                "
                    INSERT INTO companies
                    (
                        name,
                        address,
                        scope_id,
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
                        $6
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
                    &company.name,
                    &company.address,
                    &company.scope_id,
                    &company.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        company.id = row.get(0);
        if let Some(emails) = company.emails.clone() {
            Email::update_companies(client, company.id, emails).await?;
        }
        if let Some(phones) = company.phones.clone() {
            Phone::update_companies(client, company.id, false, phones).await?;
        }
        if let Some(faxes) = company.faxes.clone() {
            Phone::update_companies(client, company.id, true, faxes).await?;
        }
        Ok(company)
    }

    pub async fn update(client: &Client, company: Company) -> Result<u64> {
        let stmt = client
            .prepare(
                "
                    UPDATE companies SET
                        name = $2,
                        address = $3,
                        scope_id = $4,
                        note = $5,
                        updated_at = $6
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let result = client
            .execute(
                &stmt,
                &[
                    &company.id,
                    &company.name,
                    &company.address,
                    &company.scope_id,
                    &company.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        if let Some(emails) = company.emails.clone() {
            Email::update_companies(client, company.id, emails).await?;
        }
        if let Some(phones) = company.phones.clone() {
            Phone::update_companies(client, company.id, false, phones).await?;
        }
        if let Some(faxes) = company.faxes.clone() {
            Phone::update_companies(client, company.id, true, faxes).await?;
        }
        Ok(result)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<u64> {
        Phone::delete_companies(&client, id, true).await?;
        Phone::delete_companies(&client, id, false).await?;
        Email::delete_companies(&client, id).await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        companyes
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl CompanyList {
    pub async fn get_all(client: &Client) -> Result<Vec<CompanyList>> {
        let mut companies = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.id,
                        c.name,
                        c.address,
                        s.name AS scope_name,
                        array_agg(DISTINCT e.email) AS emails,
                        array_agg(DISTINCT p.phone) AS phones,
                        array_agg(DISTINCT f.phone) AS faxes,
                        array_agg(DISTINCT pr.date_of_practice) AS practices
                    FROM
                        companies AS c
                    LEFT JOIN
                        scopes AS s ON c.scope_id = s.id
                    LEFT JOIN
                        emails AS e ON c.id = e.company_id
                    LEFT JOIN
                        phones AS p ON c.id = p.company_id AND p.fax = false
                    LEFT JOIN
                        phones AS f ON c.id = f.company_id AND f.fax = true
                    LEFT JOIN
                        practices AS pr ON c.id = pr.company_id
                    GROUP BY
                        c.id,
                        s.name
                    ORDER BY
                        c.name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            companies.push(CompanyList {
                id: row.get(0),
                name: row.get(1),
                address: row.get(2),
                scope_name: row.get(3),
                emails: row.get(4),
                phones: row.get(5),
                faxes: row.get(6),
                practices: row.get(7),
            });
        }
        Ok(companies)
    }
}

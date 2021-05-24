// // use chrono::NaiveDateTime;

// // pub struct Tcc {
// // 	pub id: i64,
// // 	pub address: Option<String>,
// // 	pub contact_id: Option<i64>,
// // 	pub contact_name: Option<String>,
// // 	pub company_id: Option<i64>,
// // 	pub company_name: Option<String>,
// // 	pub note: Option<String>,
// //  #[serde(skip_serializing)]
// // 	pub created_at: Option<NaiveDateTime>,
// //  #[serde(skip_serializing)]
// // 	pub updated_at: Option<NaiveDateTime>,
// // }

// // // TccList - struct for tcc list
// // pub struct TccList {
// // 	pub id: i64,
// // 	pub address: Option<String>,
// // 	pub contact_id: Option<i64>,
// // 	pub contact_name: Option<String>,
// // 	pub note: Option<String>,
// // }

// // // GetTcc - get one tcc by id
// // pub async fn GetTcc(pool: &Pool<tokio_postgres::NoTls>, id: i64) -> Result<Tcc, RpelError> {
// // let client = client.get().await?;
// // 	let mut tcc = Tcc::new();
// // 	if id == 0 {
// // 		Ok(tcc)
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("id = ?", id).
// // 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// // 	if err != nil {
// // 		errmsg("GetTcc select", err)
// // 	}
// // 	Ok(tcc)
// // }

// // // GetTccList - get all tcc for list
// // pub async fn GetTccList(pool: &Pool<tokio_postgres::NoTls>, id: i64) -> Result<TccList, RpelError> {
// // let client = client.get().await?;
// // 	let mut tccs = TccList::new();
// // 	else { for row in &conn.query("
// // 		Column("id", "address", "contact_id", "note").
// // 		Where("id = ?", id).
// // 		Select(&tccs)
// // 	if err != nil {
// // 		errmsg("GetTccList select", err)
// // 	}
// // 	Ok(tccs)
// // }

// // // GetTccListAll - get all tcc for list
// // pub async fn GetTccListAll(pool: &Pool<tokio_postgres::NoTls>, id: i64) -> Result<Vec<TccList>> {
// // 	let mut tccs = Vec::new();
// // 	else { for row in &conn.query("
// // 		Column("id", "address", "contact_id", "note").
// // 		Order("name ASC").
// // 		Select(&tccs)
// // 	if err != nil {
// // 		errmsg("GetTccList select", err)
// // 	}
// // 	Ok(tccs)
// // }

// // // CreateTcc - create new tcc
// // pub async fn CreateTcc(tcc Tcc) (int64, error) {
// // 	err := e.db.Insert(&tcc)
// // 	if err != nil {
// // 		errmsg("CreateTcc insert", err)
// // 	}
// // 	return tcc.Id, err
// // }

// // // UpdateTcc - save tcc changes
// // pub async fn UpdateTcc(tcc Tcc) error {
// // 	err := e.db.Update(&tcc)
// // 	if err != nil {
// // 		errmsg("UpdateTcc update", err)
// // 	}
// // 	return err
// // }

// // // DeleteTcc - delete tcc by id
// // pub async fn DeleteTcc(id int64) error {
// // 	if id == 0 {
// // 		return nil
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("id = ?", id).
// // 		Delete()
// // 	if err != nil {
// // 		errmsg("DeleteTcc delete", err)
// // 	}
// // 	return err
// // }

// // // pub async fn tccCreateTable() error {
// // // 	str := `
// // // 		CREATE TABLE IF NOT EXISTS
// // // 			tccs (
// // // 				id         bigserial PRIMARY KEY,
// // // 				address    text,
// // // 				contact_id bigint,
// // // 				company_id bigint,
// // // 				note       text,
// // // 				created_at TIMESTAMP without time zone,
// // // 				updated_at TIMESTAMP without time zone default now(),
// // // 				UNIQUE(num_id, num_pass, type_id)
// // // 			)
// // // 	`
// // // 	_, err := e.db.Exec(str)
// // // 	if err != nil {
// // // 		errmsg("tccCreateTable exec", err)
// // // 	}
// // // 	return err
// // // }

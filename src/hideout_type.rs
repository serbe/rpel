// package edc

// // HideoutType - struct for hideoutType
// pub struct HideoutType {
// 	#[serde(default)]
// 	pub id: i64,
// 	pub name: Option<String>,
// 	pub note: Option<String>,
// 	#[serde(skip_serializing)]
// 	pub created_at: Option<NaiveDateTime>,
// 	#[serde(skip_serializing)]
// 	pub updated_at: Option<NaiveDateTime>,
// }

// // HideoutTypeList - struct for hideoutType list
// pub struct HideoutTypeList {
// 	pub id: i64,
// 	pub name: Option<String>,
// 	pub note: Option<String>,
// }

// // GetHideoutType - get one hideoutType by id
// pub async fn GetHideoutType(client: &Client, id: i64) -> Result<HideoutType> {
// // let client = client.get().await?;
// 	let mut hideoutType = HideoutType::new();
// 	if id == 0 {
// 		Ok(hideoutType)
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetHideoutType select", err)
// 	}
// 	Ok(hideoutType)
// }

// // GetHideoutTypeList - get hideoutType for list by id
// pub async fn GetHideoutTypeList(client: &Client, id: i64) -> Result<HideoutTypeList> {
// // let client = client.get().await?;
// 	let mut hideoutType = HideoutTypeList::new();
// 	else { for row in &conn.query("
// 		Column("id", "name", "note").
// 		Where("id = ?", id).
// 		Select(&hideoutType)
// 	if err != nil {
// 		errmsg("GetHideoutTypeList select", err)
// 	}
// 	Ok(hideoutType)
// }

// // GetHideoutTypeListAll - get all hideoutType for list
// pub async fn GetHideoutTypeListAll(client: &Client, id: i64) -> Result<Vec<HideoutTypeList>> {
// 	let mut hideoutTypes = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name", "note").
// 		Order("name ASC").
// 		Select(&hideoutTypes)
// 	if err != nil {
// 		errmsg("GetHideoutTypeList select", err)
// 	}
// 	Ok(hideoutTypes)
// }

// // GetHideoutTypeSelect - get hideoutType for select by id
// pub async fn GetHideoutTypeSelect(client: &Client, id: i64) -> Result<Vec<SelectItem>> {
// 	let mut hideoutTypes = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Where("id = ?", id).
// 		Select(&hideoutTypes)
// 	if err != nil {
// 		errmsg("GetHideoutTypeSelect Select", err)
// 	}
// 	Ok(hideoutTypes)
// }

// // GetHideoutTypeSelectAll - get all hideoutType for select
// pub async fn GetHideoutTypeSelectAll(client: &Client, id: i64) -> Result<Vec<SelectItem>> {
// 	let mut hideoutTypes = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Order("name ASC").
// 		Select(&hideoutTypes)
// 	if err != nil {
// 		errmsg("GetHideoutTypeSelect Select", err)
// 	}
// 	Ok(hideoutTypes)
// }

// // CreateHideoutType - create new hideoutType
// pub async fn CreateHideoutType(hideoutType HideoutType) (int64, error) {
// 	err := e.db.Insert(&hideoutType)
// 	if err != nil {
// 		errmsg("CreateHideoutType insert", err)
// 	}
// 	return hideoutType.ID, nil
// }

// // UpdateHideoutType - save hideoutType changes
// pub async fn UpdateHideoutType(hideoutType HideoutType) error {
// 	err := e.db.Update(&hideoutType)
// 	if err != nil {
// 		errmsg("UpdateHideoutType update", err)
// 	}
// 	return err
// }

// // DeleteHideoutType - delete hideoutType by id
// pub async fn DeleteHideoutType(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteHideoutTypedelete", err)
// 	}
// 	return err
// }

// pub async fn hideoutTypeCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			hideout_types (
// 				id         bigserial primary key,
// 				name       text,
// 				note       text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE(name)
// 			);`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("hideoutCreateTable exec", err)
// 	}
// 	return err
// }

// package edc

// // Hideout       - защитное сооружение
// // ID            - номер в базе данных
// // Num           - Номер убежища в реестре имущества
// // InvNum        - Инвентарный номер убежища
// // InvAdd        - дополнительный код инвентарного номера убежища
// // HideoutTypeID - номер типа защитного сооружения в базе данных
// // HideoutType   - тип защитного сооружения
// // Address       - Полный адрес места расположения убежища, с указанием строения, подъезда
// // OwnerID       - номер собственника в базе данных
// // Owner         - организация, собственник ЗС
// // DesignerID    - номер проектной организации в базе данных
// // Designer      - проектная организация
// // BuilderID     - номер строительной организации в базе данных
// // Builder       - строительная организация
// // Purpose       - назначение ЗС в мирное время
// // Commissioning - дата ввода в эксплуатацию
// // Readiness     - время приведения в готовность
// // Capacity      - вместимость
// // Area          - общая площадь
// // Size          - общий объем
// // Floors        - встроено в здание (этажность)
// // Separate      - здание отдельно стоящее
// // Excavation    - здание в горных выработках
// // Inputs        - количество входов
// // Coefficient   - коэффициент ослабления гамма излучения К
// // Stress        - расчетная нагрузка на действие ударной волны
// // Ventilation   - система вентиляции
// // Heating       - система отопления
// // Power         - система энергосбережения
// // Water         - система водоснабжения
// // Sewerage      - система канализации
// // Implements    - оборудование (инструмент, инвентарь)
// // ContactID     - номер контактного лица в базе данных
// // Contact       - контактное лицо
// // Condition     - готовность к приему укрываемых
// // Note          - заметки
// // CreatedAt     - время создания записи в базе данных
// // UpdatedAt     - время изменения записи в базе данных
// pub struct Hideout {
// 	pub id: i64,
// 	pub $1: Option<i64>,
// 	pub inv_num: Option<i64>,
// 	pub inv_add: Option<i64>,
// 	pub hideout_type_id: Option<i64>,
// 	HideoutType   HideoutType `sql:"-"               json:"hideout_type"    form:"hideout_type"    query:"hideout_type"`
// 	pub $1: Option<String>,
// 	pub owner_id: Option<i64>,
// 	Owner         Company     `sql:"-"               json:"owner"           form:"owner"           query:"owner"`
// 	pub designer_id: Option<i64>,
// 	Designer      Company     `sql:"-"               json:"designer"        form:"designer"        query:"designer"`
// 	pub builder_id: Option<i64>,
// 	Builder       Company     `sql:"-"               json:"builder"         form:"builder"         query:"builder"`
// 	pub purpose: Option<String>,
// 	pub commissioning: Option<String>,
// 	pub readiness: Option<i64>,
// 	pub capacity: Option<i64>,
// 	pub area: Option<i64>,
// 	pub size: Option<i64>,
// 	pub floors: Option<i64>,
// 	Separate      bool        `sql:"separate"        json:"separate"        form:"separate"        query:"separate"`
// 	Excavation    bool        `sql:"excavation"      json:"excavation"      form:"excavation"      query:"excavation"`
// 	pub inputs: Option<i64>,
// 	pub coefficient: Option<i64>,
// 	pub stress: Option<i64>,
// 	pub ventilation: Option<String>,
// 	pub heating: Option<String>,
// 	pub power: Option<String>,
// 	pub water: Option<String>,
// 	pub sewerage: Option<String>,
// 	pub implements: Option<String>,
// 	pub contact_id: Option<i64>,
// 	Contact       Contact     `sql:"-"               json:"contact"         form:"contact"         query:"contact"`
// 	pub condition: Option<String>,
// 	pub note: Option<String>,
// 	#[serde(skip_serializing)]
// 	pub created_at: Option<NaiveDateTime>,
// 	#[serde(skip_serializing)]
// 	pub updated_at: Option<NaiveDateTime>,
// }

// // HideoutList - struct for hideout list
// pub struct HideoutList {
// 	pub id: i64,
// 	pub hideout_type_name: Option<String>,
// 	pub address: Option<String>,
// 	pub contact_name: Option<String>,
// 	pub phones: Vec<i64>,
// }

// // GetHideout - get one hideout by id
// pub async fn GetHideout(client: &Client, id: i64) -> Result<Hideout, RpelError> {
// // let client = client.get().await?;
// 	let mut hideout = Hideout::new();
// 	if id == 0 {
// 		Ok(hideout)
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetHideout select", err)
// 	}
// 	Ok(hideout)
// }

// // GetHideoutList - get all hideout for list
// pub async fn GetHideoutList(client: &Client, id: i64) -> Result<Vec<HideoutList>> {
// 	let mut $1 = Vec::new();
// 	for row in &conn.query("
// 		SELECT
// 			s.id,
// 			s.address,
// 			t.name AS hideout_type_name,
// 			c.name AS contact_name,
// 			array_remove(array_agg(ph.phone), NULL) AS phones
//         FROM
// 			hideouts AS s
// 		LEFT JOIN
// 			hideout_types AS t ON s.type_id = t.id
// 		LEFT JOIN
// 			contacts AS c ON s.contact_id = c.id
// 		LEFT JOIN
// 			phones AS ph ON s.contact_id = ph.contact_id AND ph.fax = false
// 		GROUP BY
// 			s.id,
// 			t.id,
// 			c.id
// 		ORDER BY
// 			t.name ASC
// 	", &[]).map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetHideoutList Query", err)
// 	}
// 	Ok(hideouts)
// }

// // CreateHideout - create new hideout
// pub async fn CreateHideout(hideout Hideout) (int64, error) {
// 	err := e.db.Insert(&hideout)
// 	if err != nil {
// 		errmsg("CreateHideout insert", err)
// 	}
// 	return hideout.ID, err
// }

// // UpdateHideout - save hideout changes
// pub async fn UpdateHideout(hideout Hideout) error {
// 	err := e.db.Update(&hideout)
// 	if err != nil {
// 		errmsg("UpdateHideout update", err)
// 	}
// 	return err
// }

// // DeleteHideout - delete hideout by id
// pub async fn DeleteHideout(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteHideout delete", err)
// 	}
// 	return err
// }

// pub async fn hideoutCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			hideouts (
// 				id              bigserial PRIMARY KEY,
// 				num             bigint,
// 				inv_num         bigint,
// 				inv_add         bigint,
// 				hideout_type_id bigint,
// 				address         text,
// 				owner_id        bigint,
// 				designer_id     bigint,
// 				builder_id      bigint,
// 				purpose         text,
// 				commissioning   text,
// 				readiness       bigint,
// 				capacity        bigint,
// 				area            bigint,
// 				size            bigint,
// 				floors          bigint,
// 				separate        bool
// 				excavation      bool
// 				inputs          bigint,
// 				coefficient     bigint,
// 				stress          bigint,
// 				ventilation     text,
// 				heating         text,
// 				power           text,
// 				water           text,
// 				sewerage        text,
// 				implements      text,
// 				contact_id      bigint,
// 				condition       text,
// 				note            text,
// 				created_at      TIMESTAMP without time zone,
// 				updated_at      TIMESTAMP without time zone default now(),
// 				UNIQUE(num, inv_num, inv_add)
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("hideoutCreateTable exec", err)
// 	}
// 	return err
// }

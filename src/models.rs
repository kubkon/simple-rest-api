use schema::readings;

#[derive(Debug,Queryable)]
pub struct Reading {
  pub id: i32,
  pub timestamp: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Debug,Insertable,Serialize,Deserialize)]
#[table_name = "readings"]
pub struct NewReading {
  pub timestamp: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}


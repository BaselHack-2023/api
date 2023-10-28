use crate::schema::properties;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Property {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub address2: Option<String>,
    pub city: String,
    pub zip: String,
    pub owner: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = properties)]
pub struct NewProperty<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub address2: Option<&'a str>,
    pub city: &'a str,
    pub zip: &'a str,
    pub owner: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyPayload {
    pub name: String,
    pub address: String,
    pub address2: Option<String>,
    pub city: String,
    pub zip: String,
    pub owner: Uuid,
}

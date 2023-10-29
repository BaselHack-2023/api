use crate::schema::items;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub size: String,
    pub colors: String,
    pub owner: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub size: &'a str,
    pub colors: &'a str,
    pub owner: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPayload {
    pub name: String,
    pub size: String,
    pub colors: String,
    pub owner: Uuid,
}

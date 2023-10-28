use crate::schema::roles;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub name: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePayload {
    pub name: String,
}

use crate::schema::machines;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
    pub property: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = machines)]
pub struct NewMachine<'a> {
    pub name: &'a str,
    pub property: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachinePayload {
    pub name: String,
    pub property: Uuid,
}

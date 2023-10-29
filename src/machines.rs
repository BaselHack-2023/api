use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::helpers::{ErrorResponse, SuccessResponse};
use crate::models::machine::{Machine, MachinePayload, NewMachine};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/machines")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let machines = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: machines,
    }))
}

#[post("/machines")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<MachinePayload>,
) -> Result<HttpResponse, Error> {
    let machine = web::block(move || {
        let mut conn = pool.get()?;
        add(&payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(SuccessResponse {
        status: 201,
        message: "Created".to_string(),
        data: machine,
    }))
}

#[get("/machines/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let machine = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if machine.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            status: 404,
            message: "Machine not found".to_string(),
        }));
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: machine,
    }))
}

#[put("/machines/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<MachinePayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let machine = web::block(move || {
        let mut conn = pool.get()?;
        update_by_id(id.into_inner(), &payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: machine,
    }))
}

#[delete("/machines/{id}")]
async fn destroy(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|machine| {
        HttpResponse::Ok().json(SuccessResponse {
            status: 200,
            message: "Deleted".to_string(),
            data: machine,
        })
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add(payload: &MachinePayload, conn: &mut PgConnection) -> Result<Machine, DbError> {
    use crate::schema::machines::dsl::*;

    let new_machine = NewMachine {
        name: payload.name.as_str(),
        property: payload.property,
        status: payload.status.as_str(),
        eta: payload.eta,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(machines)
        .values(&new_machine)
        .returning(machines::all_columns())
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Machine>, DbError> {
    use crate::schema::machines::dsl::*;

    let items = machines.load::<Machine>(conn)?;
    Ok(items)
}

fn find_by_id(machine_id: Uuid, conn: &mut PgConnection) -> Result<Option<Machine>, DbError> {
    use crate::schema::machines::dsl::*;

    let machine = machines
        .filter(id.eq(machine_id))
        .first::<Machine>(conn)
        .optional()?;

    Ok(machine)
}

fn update_by_id(
    machine_id: Uuid,
    payload: &MachinePayload,
    conn: &mut PgConnection,
) -> Result<Machine, DbError> {
    use crate::schema::machines::dsl::*;

    let machine = diesel::update(machines.find(machine_id))
        .set((
            name.eq(payload.name.to_string()),
            property.eq(payload.property),
            status.eq(payload.status.to_string()),
            eta.eq(payload.eta),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Machine>(conn)?;
    Ok(machine)
}

fn delete(machine_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::machines::dsl::*;

    let count = diesel::delete(machines.find(machine_id)).execute(conn)?;
    Ok(count)
}

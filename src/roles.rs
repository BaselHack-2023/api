use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::helpers::{ErrorResponse, SuccessResponse};
use crate::models::role::{NewRole, Role, RolePayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/roles")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let roles = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: roles,
    }))
}

#[post("/roles")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<RolePayload>,
) -> Result<HttpResponse, Error> {
    let role = web::block(move || {
        let mut conn = pool.get()?;
        add(&payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(SuccessResponse {
        status: 201,
        message: "Created".to_string(),
        data: role,
    }))
}

#[get("/roles/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let role = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if role.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            status: 404,
            message: "Role not found".to_string(),
        }));
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: role,
    }))
}

#[put("/roles/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<RolePayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let role = web::block(move || {
        let mut conn = pool.get()?;
        update_by_id(id.into_inner(), &payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: role,
    }))
}

#[delete("/roles/{id}")]
async fn destroy(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|role| {
        HttpResponse::Ok().json(SuccessResponse {
            status: 200,
            message: "Deleted".to_string(),
            data: role,
        })
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add(payload: &RolePayload, conn: &mut PgConnection) -> Result<Role, DbError> {
    use crate::schema::roles::dsl::*;

    let new_role = NewRole {
        name: payload.name.as_str(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(roles)
        .values(&new_role)
        .returning(roles::all_columns())
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Role>, DbError> {
    use crate::schema::roles::dsl::*;

    let items = roles.load::<Role>(conn)?;
    Ok(items)
}

fn find_by_id(role_id: Uuid, conn: &mut PgConnection) -> Result<Option<Role>, DbError> {
    use crate::schema::roles::dsl::*;

    let role = roles
        .filter(id.eq(role_id))
        .first::<Role>(conn)
        .optional()?;

    Ok(role)
}

fn update_by_id(
    role_id: Uuid,
    payload: &RolePayload,
    conn: &mut PgConnection,
) -> Result<Role, DbError> {
    use crate::schema::roles::dsl::*;

    let role = diesel::update(roles.find(role_id))
        .set((
            name.eq(payload.name.to_string()),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Role>(conn)?;
    Ok(role)
}

fn delete(role_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::roles::dsl::*;

    let count = diesel::delete(roles.find(role_id)).execute(conn)?;
    Ok(count)
}

use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::helpers::{ErrorResponse, SuccessResponse};
use crate::models::user::{NewUser, User, UserPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/users")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: users,
    }))
}

#[post("/users")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<UserPayload>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        add(&payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(SuccessResponse {
        status: 201,
        message: "Created".to_string(),
        data: user,
    }))
}

#[get("/users/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if user.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            status: 404,
            message: "User not found".to_string(),
        }));
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: user,
    }))
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<UserPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        update_by_id(id.into_inner(), &payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: user,
    }))
}

#[delete("/users/{id}")]
async fn destroy(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|user| {
        HttpResponse::Ok().json(SuccessResponse {
            status: 200,
            message: "Deleted".to_string(),
            data: user,
        })
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add(payload: &UserPayload, conn: &mut PgConnection) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        name: payload.name.as_str(),
        role: payload.role,
        property: payload.property,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(users)
        .values(&new_user)
        .returning(users::all_columns())
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;

    let items = users.load::<User>(conn)?;
    Ok(items)
}

fn find_by_id(user_id: Uuid, conn: &mut PgConnection) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

fn update_by_id(
    user_id: Uuid,
    payload: &UserPayload,
    conn: &mut PgConnection,
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user = diesel::update(users.find(user_id))
        .set((
            name.eq(payload.name.to_string()),
            role.eq(payload.role),
            property.eq(payload.property),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<User>(conn)?;
    Ok(user)
}

fn delete(user_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let count = diesel::delete(users.find(user_id)).execute(conn)?;
    Ok(count)
}

use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::helpers::{ErrorResponse, SuccessResponse};
use crate::models::item::{Item, ItemPayload, NewItem};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/items")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let items = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: items,
    }))
}

#[post("/items")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<ItemPayload>,
) -> Result<HttpResponse, Error> {
    let item = web::block(move || {
        let mut conn = pool.get()?;
        add(&payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(SuccessResponse {
        status: 201,
        message: "Created".to_string(),
        data: item,
    }))
}

#[get("/items/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let item = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if item.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            status: 404,
            message: "Item not found".to_string(),
        }));
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: item,
    }))
}

#[put("/items/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<ItemPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let item = web::block(move || {
        let mut conn = pool.get()?;
        update_by_id(id.into_inner(), &payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: item,
    }))
}

#[delete("/items/{id}")]
async fn destroy(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|item| {
        HttpResponse::Ok().json(SuccessResponse {
            status: 200,
            message: "Deleted".to_string(),
            data: item,
        })
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add(payload: &ItemPayload, conn: &mut PgConnection) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let new_item = NewItem {
        name: payload.name.as_str(),
        size: payload.size.as_str(),
        colors: payload.colors.as_str(),
        owner: payload.owner,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(items)
        .values(&new_item)
        .returning(items::all_columns())
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Item>, DbError> {
    use crate::schema::items::dsl::*;

    let items_ = items.load::<Item>(conn)?;
    Ok(items_)
}

fn find_by_id(item_id: Uuid, conn: &mut PgConnection) -> Result<Option<Item>, DbError> {
    use crate::schema::items::dsl::*;

    let item = items
        .filter(id.eq(item_id))
        .first::<Item>(conn)
        .optional()?;

    Ok(item)
}

fn update_by_id(
    item_id: Uuid,
    payload: &ItemPayload,
    conn: &mut PgConnection,
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let item = diesel::update(items.find(item_id))
        .set((
            name.eq(payload.name.as_str()),
            size.eq(payload.size.as_str()),
            colors.eq(payload.colors.as_str()),
            owner.eq(payload.owner),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Item>(conn)?;
    Ok(item)
}

fn delete(item_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::items::dsl::*;

    let count = diesel::delete(items.find(item_id)).execute(conn)?;
    Ok(count)
}

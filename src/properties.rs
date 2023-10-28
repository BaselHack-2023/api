use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::helpers::{ErrorResponse, SuccessResponse};
use crate::models::property::{NewProperty, Property, PropertyPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/properties")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let properties = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: properties,
    }))
}

#[post("/properties")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<PropertyPayload>,
) -> Result<HttpResponse, Error> {
    let property = web::block(move || {
        let mut conn = pool.get()?;
        add(&payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(SuccessResponse {
        status: 201,
        message: "Created".to_string(),
        data: property,
    }))
}

#[get("/properties/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let property = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if property.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            status: 404,
            message: "Property not found".to_string(),
        }));
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: property,
    }))
}

#[put("/properties/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<PropertyPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let property = web::block(move || {
        let mut conn = pool.get()?;
        update_by_id(id.into_inner(), &payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: property,
    }))
}

#[delete("/properties/{id}")]
async fn destroy(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|property| {
        HttpResponse::Ok().json(SuccessResponse {
            status: 200,
            message: "Deleted".to_string(),
            data: property,
        })
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add(payload: &PropertyPayload, conn: &mut PgConnection) -> Result<Property, DbError> {
    use crate::schema::properties::dsl::*;

    let new_property = NewProperty {
        name: payload.name.as_str(),
        address: payload.address.as_str(),
        address2: payload.address2.as_deref(),
        city: payload.city.as_str(),
        zip: payload.zip.as_str(),
        owner: payload.owner,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(properties)
        .values(&new_property)
        .returning(properties::all_columns())
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Property>, DbError> {
    use crate::schema::properties::dsl::*;

    let items = properties.load::<Property>(conn)?;
    Ok(items)
}

fn find_by_id(property_id: Uuid, conn: &mut PgConnection) -> Result<Option<Property>, DbError> {
    use crate::schema::properties::dsl::*;

    let property = properties
        .filter(id.eq(property_id))
        .first::<Property>(conn)
        .optional()?;

    Ok(property)
}

fn update_by_id(
    property_id: Uuid,
    payload: &PropertyPayload,
    conn: &mut PgConnection,
) -> Result<Property, DbError> {
    use crate::schema::properties::dsl::*;

    let property = diesel::update(properties.find(property_id))
        .set((
            name.eq(payload.name.to_string()),
            address.eq(payload.address.to_string()),
            address2.eq(payload.address2.as_deref()),
            city.eq(payload.city.to_string()),
            zip.eq(payload.zip.to_string()),
            owner.eq(payload.owner),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Property>(conn)?;
    Ok(property)
}

fn delete(property_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::properties::dsl::*;

    let count = diesel::delete(properties.find(property_id)).execute(conn)?;
    Ok(count)
}

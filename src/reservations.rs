use super::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers::{ErrorResponse, SuccessResponse};
use crate::models::reservation::{NewReservation, Reservation, ReservationPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Deserialize, Serialize)]
struct QueryParams {
    date: Option<String>,
}

#[get("/reservations")]
async fn index(
    info: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let reservations = web::block(move || {
        let mut conn = pool.get()?;

        if info.date.is_none() {
            return find_all(&mut conn);
        }
        let date: chrono::NaiveDate = info.date.is_some().to_string().parse().unwrap();

        find_all_by_date(date, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: reservations,
    }))
}

#[post("/reservations")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<ReservationPayload>,
) -> Result<HttpResponse, Error> {
    let reservation = web::block(move || {
        let mut conn = pool.get()?;
        add(&payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(SuccessResponse {
        status: 201,
        message: "Created".to_string(),
        data: reservation,
    }))
}

#[get("/reservations/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let reservation = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if reservation.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            status: 404,
            message: "Reservation not found".to_string(),
        }));
    }

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: reservation,
    }))
}

#[put("/reservations/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<ReservationPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let reservation = web::block(move || {
        let mut conn = pool.get()?;
        update_by_id(id.into_inner(), &payload, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        status: 200,
        message: "OK".to_string(),
        data: reservation,
    }))
}

#[delete("/reservations/{id}")]
async fn destroy(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|reservation| {
        HttpResponse::Ok().json(SuccessResponse {
            status: 200,
            message: "Deleted".to_string(),
            data: reservation,
        })
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add(payload: &ReservationPayload, conn: &mut PgConnection) -> Result<Reservation, DbError> {
    use crate::schema::reservations::dsl::*;

    let new_reservation = NewReservation {
        owner: payload.owner,
        machine: payload.machine,
        start_time: payload.start_time,
        end_time: payload.end_time,
        shared: payload.shared,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(reservations)
        .values(&new_reservation)
        .returning(reservations::all_columns())
        .get_result(conn)?;

    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Reservation>, DbError> {
    use crate::schema::reservations::dsl::*;

    let items = reservations.load::<Reservation>(conn)?;
    Ok(items)
}

fn find_all_by_date(
    date: chrono::NaiveDate,
    conn: &mut PgConnection,
) -> Result<Vec<Reservation>, DbError> {
    use crate::schema::reservations::dsl::*;

    // Filter by date
    let items = reservations
        .filter(start_time.between(
            date.and_hms_opt(0, 0, 0).unwrap(),
            date.and_hms_opt(23, 59, 59).unwrap(),
        ))
        .order(start_time.asc())
        .limit(10)
        .load::<Reservation>(conn)?;

    Ok(items)
}

fn find_by_id(
    reservation_id: Uuid,
    conn: &mut PgConnection,
) -> Result<Option<Reservation>, DbError> {
    use crate::schema::reservations::dsl::*;

    let reservation = reservations
        .filter(id.eq(reservation_id))
        .first::<Reservation>(conn)
        .optional()?;

    Ok(reservation)
}

fn update_by_id(
    reservation_id: Uuid,
    payload: &ReservationPayload,
    conn: &mut PgConnection,
) -> Result<Reservation, DbError> {
    use crate::schema::reservations::dsl::*;

    let reservation = diesel::update(reservations.find(reservation_id))
        .set((
            owner.eq(payload.owner),
            machine.eq(payload.machine),
            start_time.eq(payload.start_time),
            end_time.eq(payload.end_time),
            shared.eq(payload.shared),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Reservation>(conn)?;
    Ok(reservation)
}

fn delete(reservation_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::reservations::dsl::*;

    let count = diesel::delete(reservations.find(reservation_id)).execute(conn)?;
    Ok(count)
}

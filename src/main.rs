#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod helpers;
mod machines;
mod models;
mod properties;
mod reservations;
mod roles;
mod schema;
mod users;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Loading .env into environment variable.
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "1 REST API" }))
            .service(users::index)
            .service(users::create)
            .service(users::show)
            .service(users::update)
            .service(users::destroy)
            .service(roles::index)
            .service(roles::create)
            .service(roles::show)
            .service(roles::update)
            .service(roles::destroy)
            .service(properties::index)
            .service(properties::create)
            .service(properties::show)
            .service(properties::update)
            .service(properties::destroy)
            .service(machines::index)
            .service(machines::create)
            .service(machines::show)
            .service(machines::update)
            .service(machines::destroy)
            .service(reservations::index)
            .service(reservations::create)
            .service(reservations::show)
            .service(reservations::update)
            .service(reservations::destroy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

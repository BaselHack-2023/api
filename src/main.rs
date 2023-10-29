#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod helpers;
mod items;
mod machines;
mod metrics;
mod models;
mod properties;
mod reservations;
mod roles;
mod schema;
mod tea;
mod users;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Loading .env into environment variable.
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // set up connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // run the migrations on server startup
    {
        println!("Running database migrations...");
        let conn = pool.get();
        conn.unwrap()
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run database migrations.");

        println!("Migrations are up to date!")
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://app.iperka.com")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route("/", web::get().to(|| async { "Beutler REST API" }))
            .service(tea::index)
            .service(metrics::index)
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
            .service(items::index)
            .service(items::create)
            .service(items::show)
            .service(items::update)
            .service(items::destroy)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

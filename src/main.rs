mod errors;
mod schema;
mod tools;
mod users;

use crate::errors::FouineApiError;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), FouineApiError> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url =
        env::var("DATABASE_URL").map_err(FouineApiError::VarError)?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool<_> = Pool::builder()
        .build(manager)
        .map_err(FouineApiError::R2d2Error)?;

    let port = env::var("SERVER_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);

    HttpServer::new(move || App::new().app_data(pool.clone()).service(users::module()))
        .bind(("127.0.0.1", port))?
        .run()
        .await
        .map_err(FouineApiError::IoError)
}

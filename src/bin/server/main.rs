use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use fouine_api::errors::FouineApiError;
use fouine_api::tools::api_data::ApiContext;
use fouine_api::users;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), FouineApiError> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug, info");
    env_logger::init();

    let port = env::var("SERVER_PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8080);

    let ctx = ApiContext::new().await?;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ctx.clone()))
            .service(users::module())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
    .map_err(FouineApiError::IoError)
}

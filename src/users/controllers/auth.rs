use crate::errors::FouineApiError;
use actix_web::HttpResponse;

pub struct AuthController;

impl AuthController {
    pub async fn _register() -> Result<HttpResponse, FouineApiError> {
        Ok(HttpResponse::Ok().body("Show users"))
    }
}

use crate::errors::FouineApiError;
use crate::tools::uuid::check_uuid;
use actix_web::{web, HttpResponse};
use crate::users::services::user::UserService;

pub struct UserController;

impl UserController {
    pub async fn get_users() -> Result<HttpResponse, FouineApiError> {
        log::debug!("get users");

        let _ = UserService {

        };

        Ok(HttpResponse::Ok().body("Show users"))
    }

    pub async fn get_user_by_uuid(path: web::Path<String>) -> Result<HttpResponse, FouineApiError> {
        let uuid = path.into_inner();
        check_uuid(&uuid)?;

        log::debug!("get user {}", uuid);
        Ok(HttpResponse::Ok().body(format!("get user {}", uuid)))
    }

    pub async fn post_user() -> Result<HttpResponse, FouineApiError> {
        log::debug!("create user");
        Ok(HttpResponse::Ok().body("create user"))
    }

    pub async fn delete_user_by_uuid(
        path: web::Path<String>,
    ) -> Result<HttpResponse, FouineApiError> {
        let uuid = path.into_inner();
        check_uuid(&uuid)?;

        log::debug!("delete user {}", uuid);
        Ok(HttpResponse::Ok().body(format!("delete user {}", uuid)))
    }
}

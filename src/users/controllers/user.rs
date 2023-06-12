use crate::errors::FouineApiError;
use crate::tools::api_data::ApiContext;
use crate::tools::transformer::{transform_collection, transform_one};
use crate::tools::uuid::check_uuid;
use crate::users::entities::user::User;
use crate::users::services::user::UserService;
use crate::users::transformers::user::{SafeUser, UserTransformer};
use actix_web::{web, HttpResponse, Responder};

pub struct UserController;

impl UserController {
    pub async fn get_users(data: web::Data<ApiContext>) -> Result<impl Responder, FouineApiError> {
        let user_service = UserService::new(data.pool.clone());
        let users = user_service.find_all();

        transform_collection::<User, SafeUser, UserTransformer>(users.await?)
    }

    pub async fn get_user_by_uuid(
        data: web::Data<ApiContext>,
        path: web::Path<String>,
    ) -> Result<impl Responder, FouineApiError> {
        let uuid = check_uuid(&path.into_inner())?;

        let user_service = UserService::new(data.pool.clone());
        let user = user_service.find_by_uuid(uuid);

        transform_one::<User, SafeUser, UserTransformer>(user.await?)
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

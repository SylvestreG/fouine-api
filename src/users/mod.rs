mod controllers;
mod repositories;
mod services;

use actix_web::web;
use crate::users::controllers::user::UserController;

pub fn module() -> actix_web::Scope {
    web::scope("/1/users")
        .route("", web::get().to(UserController::get_users))
        .route("/{uuid}", web::get().to(UserController::get_user_by_uuid))
        .route("", web::post().to(UserController::post_user))
        .route("/{uuid}", web::delete().to(UserController::delete_user_by_uuid))
}

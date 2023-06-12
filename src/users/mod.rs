mod controllers;
mod entities;
mod repositories;
mod services;
mod transformers;

use crate::users::controllers::user::UserController;
use actix_web::web;

pub fn module() -> actix_web::Scope {
    web::scope("/1/users")
        .route("", web::get().to(UserController::get_users))
        .route("/{uuid}", web::get().to(UserController::get_user_by_uuid))
        .route("", web::post().to(UserController::post_user))
        .route(
            "/{uuid}",
            web::delete().to(UserController::delete_user_by_uuid),
        )
}

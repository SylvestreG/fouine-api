use actix_web::http::{header::ContentType, StatusCode};
use actix_web::HttpResponse;
use std::env::VarError;
use std::fmt::Debug;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FouineApiError {
    #[error("i/o error")]
    IoError(#[from] io::Error),
    #[error("env error")]
    VarError(#[from] VarError),
    #[error("r2d2 error")]
    R2d2Error(#[from] r2d2::Error),
    #[error("Invalid uuid: {0}")]
    InvalidUuid(String),
}

impl actix_web::error::ResponseError for FouineApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            FouineApiError::IoError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            FouineApiError::VarError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            FouineApiError::R2d2Error(..) => StatusCode::INTERNAL_SERVER_ERROR,
            FouineApiError::InvalidUuid { .. } => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

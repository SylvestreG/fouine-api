use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::env::VarError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FouineApiError {
    #[error("i/o error")]
    IoError(#[from] io::Error),
    #[error("env error")]
    VarError(#[from] VarError),
    #[error("SQL error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Invalid uuid: {0}")]
    InvalidUuid(String),
}

impl actix_web::error::ResponseError for FouineApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            FouineApiError::IoError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            FouineApiError::VarError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            FouineApiError::SqlxError(sqlx) => match sqlx {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            FouineApiError::InvalidUuid { .. } => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let generic_error = json!({
            "message": self.to_string(),
            "code": self.status_code().to_string()
        });

        let payload = web::Json(match self {
            FouineApiError::SqlxError(sqlx) => match sqlx {
                sqlx::Error::RowNotFound => json!({
                    "message": "Entity Missing",
                    "code": self.status_code().to_string()
                }),
                _ => generic_error,
            },
            _ => generic_error,
        });
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json()).json(
            payload
        )
    }
}

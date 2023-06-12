use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_status")]
#[sqlx(rename_all = "snake_case")]
pub enum UserStatus {
    WaitingMailValidation,
    WaitingPhoneValidation,
    Validated,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub status: UserStatus,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub phone: String,
    pub email: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {}

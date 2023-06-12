use crate::errors::FouineApiError;
use crate::users::entities::user::User;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserRepository { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, FouineApiError> {
        sqlx::query_as!(
            User,
            r#"select
                    id,
                    uuid,
                    status as "status: _",
                    firstname,
                    lastname,
                    password,
                    phone,
                    email,
                    created_at,
                    updated_at
            from "user""#
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(FouineApiError::SqlxError)
    }

    pub async fn find_by_uuid(&self, uuid: Uuid) -> Result<User, FouineApiError> {
        sqlx::query_as!(
            User,
            r#"select
                    id,
                    uuid,
                    status as "status: _",
                    firstname,
                    lastname,
                    password,
                    phone,
                    email,
                    created_at,
                    updated_at
            from "user"
            where "user".uuid = $1"#,
            uuid
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(FouineApiError::SqlxError)
    }
}

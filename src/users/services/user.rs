use crate::users::entities::user::User;
use crate::users::repositories::user::UserRepository;
use crate::FouineApiError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserService {
            repository: UserRepository::new(pool.clone()),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, FouineApiError> {
        self.repository.find_all().await
    }

    pub async fn find_by_uuid(&self, uuid: Uuid) -> Result<User, FouineApiError> {
        self.repository.find_by_uuid(uuid).await
    }
}

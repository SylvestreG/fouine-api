use crate::errors::FouineApiError;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait GenericFixtures<Entity, PartialEntity> {
    fn data(&self) -> Vec<PartialEntity>;

    async fn insert(&self, pool: &PgPool, elem: PartialEntity) -> Result<Entity, FouineApiError>;
}

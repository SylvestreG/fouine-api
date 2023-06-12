use crate::errors::FouineApiError;
use std::str::FromStr;
use uuid::Uuid;

pub fn check_uuid(uuid: &str) -> Result<Uuid, FouineApiError> {
    Uuid::from_str(uuid).map_err(|_| FouineApiError::InvalidUuid(uuid.to_owned()))
}

use crate::errors::FouineApiError;
use std::str::FromStr;
use uuid::Uuid;


pub fn check_uuid(uuid: &str) -> Result<(), FouineApiError> {
    let validated_uuid = Uuid::from_str(uuid);
    if validated_uuid.is_err() {
        return Err(FouineApiError::InvalidUuid(uuid.to_owned()));
    }
    Ok(())
}

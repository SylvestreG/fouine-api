use crate::tools::transformer::GenericTransformer;
use crate::users::entities::user::{User, UserStatus};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

pub struct UserTransformer;

#[derive(Debug, Serialize)]
pub struct SafeUser {
    pub uuid: Uuid,
    pub status: UserStatus,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct AdminUser {
    pub user: SafeUser,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            uuid: user.uuid,
            status: user.status,
            firstname: user.firstname,
            lastname: user.lastname,
            phone: user.phone,
            email: user.email,
        }
    }
}

impl From<User> for AdminUser {
    fn from(user: User) -> Self {
        AdminUser {
            user: SafeUser {
                uuid: user.uuid,
                status: user.status,
                firstname: user.firstname,
                lastname: user.lastname,
                phone: user.phone,
                email: user.email,
            },
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl GenericTransformer<User, SafeUser> for UserTransformer {
    fn transform(user: User) -> SafeUser {
        user.into()
    }
}

impl GenericTransformer<User, AdminUser> for UserTransformer {
    fn transform(user: User) -> AdminUser {
        user.into()
    }
}

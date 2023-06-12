use std::str::FromStr;
use crate::errors::FouineApiError;
use crate::tools::fixtures::GenericFixtures;
use crate::users::entities::user::User;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserFixtures;

pub struct UserFixtureEntity {
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub phone: String,
    pub email: String,
}

#[async_trait::async_trait]
impl GenericFixtures<User, UserFixtureEntity> for UserFixtures {
    fn data(&self) -> Vec<UserFixtureEntity> {
        vec![
            UserFixtureEntity {
                uuid: Uuid::from_str("79f2f95e-3936-47ce-85c2-be1456fa3180").unwrap(),
                firstname: "Sylvestre".to_string(),
                lastname: "Gallon".to_string(),
                password: "toto".to_string(),
                phone: "+003342".to_string(),
                email: "syl@test.com".to_string(),
            },
            UserFixtureEntity {
                uuid: Uuid::from_str("79f2f95e-3936-47ce-85c2-be1456fa3181").unwrap(),
                firstname: "Yoann".to_string(),
                lastname: "Thomas".to_string(),
                password: "tutu".to_string(),
                phone: "+003343".to_string(),
                email: "yo@test.com".to_string(),
            },
            UserFixtureEntity {
                uuid: Uuid::from_str("79f2f95e-3936-47ce-85c2-be1456fa3182").unwrap(),
                firstname: "Homer".to_string(),
                lastname: "Simpson".to_string(),
                password: "toto".to_string(),
                phone: "+003345".to_string(),
                email: "homer@test.com".to_string(),
            },
            UserFixtureEntity {
                uuid: Uuid::from_str("79f2f95e-3936-47ce-85c2-be1456fa3183").unwrap(),
                firstname: "Marge".to_string(),
                lastname: "Simpson".to_string(),
                password: "tutu".to_string(),
                phone: "+003344".to_string(),
                email: "marge@test.com".to_string(),
            },
        ]
    }

    async fn insert(&self, pool: &PgPool, elem: UserFixtureEntity) -> Result<User, FouineApiError> {
        sqlx::query_as!(
            User,
            r#"
            insert into "user" (uuid, firstname, lastname, password, phone, email)
            values ($1, $2, $3, $4, $5, $6)
            returning
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
        "#,
            &elem.uuid,
            &elem.firstname,
            &elem.lastname,
            &elem.password,
            &elem.phone,
            &elem.email
        )
        .fetch_one(pool)
        .await
        .map_err(FouineApiError::SqlxError)
    }
}

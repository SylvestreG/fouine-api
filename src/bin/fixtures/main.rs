use dotenv::dotenv;
use fouine_api::errors::FouineApiError;
use fouine_api::errors::FouineApiError::SqlxError;
use fouine_api::tools::api_data::ApiContext;
use fouine_api::tools::fixtures::GenericFixtures;
use fouine_api::users::fixtures::user::UserFixtures;
use log::info;
use sqlx::PgPool;
use std::env;
use std::fmt::Debug;

#[derive(Debug)]
pub enum FixturesContainer {
    User(UserFixtures),
}

#[derive(Debug)]
pub enum FixturesAction {
    Setup,
}

pub async fn insert<Entity, PartialEntity>(
    pool: &PgPool,
    fixture: Box<dyn GenericFixtures<Entity, PartialEntity>>,
) -> Result<(), FouineApiError> {
    let entries = fixture.data();

    for entry in entries {
        fixture.insert(pool, entry).await?;
    }
    Ok(())
}

pub async fn match_and_call(
    pool: &PgPool,
    fixtures: FixturesContainer,
    action: FixturesAction,
) -> Result<(), FouineApiError> {
    info!("\t ⦿ {:#?} on {:?} ✅", action, fixtures);

    match fixtures {
        FixturesContainer::User(user) => match action {
            FixturesAction::Setup => insert(pool, Box::new(user)).await?,
        },
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), FouineApiError> {
    dotenv().ok();

    let tables_to_truncate = vec!["auth_client", "token_history", "token", "user"];

    let fixtures: Vec<FixturesContainer> = vec![FixturesContainer::User(UserFixtures {})];

    env::set_var("RUST_LOG", "actix_web=debug, sqlx=warn, info");
    env_logger::init();
    let ctx = ApiContext::new().await?;

    info!("Truncate Tables:");
    for table in tables_to_truncate {
        let query = format!("delete from \"{}\";", table);
        info!("\t ⦿ {} ✅", query);
        sqlx::query(&query)
            .execute(&*ctx.pool)
            .await
            .map_err(SqlxError)?;
    }

    info!("Setup Tables:");
    for fixture in fixtures {
        match_and_call(&ctx.pool, fixture, FixturesAction::Setup).await?;
    }

    Ok(())
}

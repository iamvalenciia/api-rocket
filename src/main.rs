mod api;
mod models;
mod repository;

use anyhow::anyhow;
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;
use shuttle_secrets::SecretStore;

#[macro_use]
extern crate rocket;

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    // get secret defined in `Secrets.toml` file.
    let secret = if let Some(secret) = secret_store.get("MONGOURI") {
        secret
    } else {
        return Err(anyhow!("Secret not found").into());
    };
    let db = MongoRepo::init(secret);
    let rocket = rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user]);
    Ok(rocket.into())
}

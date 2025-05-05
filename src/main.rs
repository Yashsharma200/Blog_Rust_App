use std::env;
use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use utils::db_utils::{get_pool, AppState, DbActor};
mod routes;
mod utils;
mod db_models;
mod messages;
use dotenvy::dotenv;
mod schema;
mod actors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}",database_url);
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&database_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    let _ =  HttpServer::new( move || {
        App::new().app_data(Data::new(AppState{db:db_addr.clone()}))
        .configure(routes::configure_routes::config)
    })
    .bind("localhost:8080")?
    .run()
    .await;
    Ok(())
}

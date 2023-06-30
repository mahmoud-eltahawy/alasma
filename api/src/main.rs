mod config;
mod repo;
mod service;

use chrono::NaiveDate;
use config::{
    get_config_postgres_url, get_configs_server,
    set_debug_configs,
};
use dotenv::dotenv;

use actix_web::{
    middleware::Logger, web::Data, App, HttpServer,
};

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions, types::BigDecimal, Pool,
    Postgres,
};

pub struct AppState {
    pub db: Pool<Postgres>,
}

use service::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    set_debug_configs();

    let db_pool = connect_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_pool.clone(),
            }))
            .wrap(Logger::default())
            .service(sells::scope())
            .service(boughts::scope())
            .service(bill::scope())
    })
    .bind(get_configs_server())?
    .run()
    .await?;
    Ok(())
}

async fn connect_db_pool() -> Pool<Postgres> {
    let p = PgPoolOptions::new()
        .max_connections(10)
        .connect(&get_config_postgres_url())
        .await
        .expect("failed to connect db");

    sqlx::migrate!("db/migrations")
        .run(&p)
        .await
        .expect("migration failed");

    p
}

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Bill {
    id: Uuid,
    bill_number: Option<i64>,
    the_date: Option<NaiveDate>,
    is_sell: bool,
}

#[derive(Serialize, Deserialize)]
struct CargoBill {
    id: Uuid,
    cargo_name: Option<String>,
    bill_id: Option<Uuid>,
    quantity: Option<i64>,
    one_cost: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct BuyBill {
    id: Uuid,
    cargo_name: Option<String>,
    bill_id: Option<Uuid>,
    quantity: Option<i64>,
    one_cost: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize)]
pub struct Client {
    id: Uuid,
    cargo_id: Uuid,
    the_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Company {
    id: Uuid,
    the_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellBill {
    bill_id: Uuid,
    tax_number: Option<i64>,
    company_id: Option<Uuid>,
    client_id: Option<Uuid>,
    total_cost: Option<BigDecimal>,
    discount: BigDecimal,
}

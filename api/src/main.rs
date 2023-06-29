mod config;
mod repo;
mod service;

use config::{get_config_postgres_url, get_configs_server, set_debug_configs};
use dotenv::dotenv;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, types::BigDecimal};

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

// struct Cargo{
//     id : Uuid,
//     the_name :  String,
//     cargo_number :  i64,
// }

// struct Bill{
//    bill_number : i64,
//    the_date : NaiveDate,
// }

// struct CargoBill{
//    id : Uuid,
//    cargo_id : Option<Uuid>,
//    bill_number : Option<i64>,
//    quantity : Option<i64>,
//    one_cost : Option<i64>,
// }

// pub struct BuyBill{
//     id : Uuid,
//     cargo_id : Option<Uuid>,
//     bill_number : Option<i64>,
//     quantity : Option<i64>,
//     one_cost : Option<f64>,
// }

// pub struct Client{
//     id : Uuid,
//     cargo_id : Uuid,
//     the_name : String
// }

// pub struct Company{
//     id : Uuid,
//     the_name : String
// }

#[derive(Serialize,Deserialize)]
pub struct SellBill{
    bill_number : i64,
    tax_number : Option<i64>,
    company_id : Option<Uuid>,
    client_id : Option<Uuid>,
    total_cost : Option<BigDecimal>,
    discount : Option<BigDecimal>,
}

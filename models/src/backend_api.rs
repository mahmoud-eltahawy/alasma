use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bill {
   pub id: Uuid,
   pub bill_number: Option<i64>,
   pub the_date: Option<NaiveDate>,
   pub is_sell: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Cargo {
  pub id: Uuid,
  pub cargo_name: Option<String>,
  pub cargo_number : Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TypeRow {
  pub id: Uuid,
  pub cargo_id: Option<Uuid>,
  pub bill_id: Option<Uuid>,
  pub quantity: Option<i64>,
  pub one_cost: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize)]
pub struct BuyBill {
  pub id: Uuid,
  pub cargo_name: Option<String>,
  pub bill_id: Option<Uuid>,
  pub quantity: Option<i64>,
  pub one_cost: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize)]
pub struct Client {
  pub id: Uuid,
  pub cargo_id: Uuid,
  pub the_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Company {
  pub id: Uuid,
  pub the_name: String,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct SellBill {
  pub bill_id: Uuid,
  pub tax_number: Option<i64>,
  pub company_id: Option<Uuid>,
  pub client_id: Option<Uuid>,
  pub total_cost: Option<BigDecimal>,
  pub discount: BigDecimal,
}

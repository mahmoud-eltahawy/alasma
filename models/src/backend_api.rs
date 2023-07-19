use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq,Default)]
pub enum SheetType {
    #[default]
    Sells,
    Boughts,
    Types,
}

#[derive(Serialize, Deserialize, Debug, Clone,Default)]
pub struct Sheet {
    pub id: Uuid,
    pub the_name: String,
    pub the_date: NaiveDate,
    pub the_type: SheetType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SheetShearchParams {
    pub offset: i64,
    pub begin: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub name: Option<String>,
    pub sheet_type: SheetType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Bill {
    pub id: Uuid,
    pub bill_number: i64,
    pub the_date: NaiveDate,
    pub is_sell: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Cargo {
    pub id: Uuid,
    pub cargo_name: Option<String>,
    pub cargo_number: Option<i64>,
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Name {
    pub id: Uuid,
    pub the_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Client {
    pub id: Uuid,
    pub the_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Company {
    pub id: Uuid,
    pub the_name: String,
}

#[derive(Clone, Default,Serialize,Deserialize)]
pub struct NaiveSellBill {
    pub bill: Bill,
    pub tax_number: u64,
    pub company: Company,
    pub client: Option<Client>,
    pub value: f64,
    pub discount: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SellBill {
    pub bill_id: Uuid,
    pub tax_number: Option<i64>,
    pub company_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub sheet_id: Uuid,
    pub total_cost: Option<BigDecimal>,
    pub discount: BigDecimal,
}

use models::backend_api::{Company, Client, Sheet, Bill, SellBill, Name};
use reqwest::StatusCode;

use crate::AppState;

pub async fn save_company(
    app_state: &AppState,
    company : &Company,
) -> Result<(), Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .post(format!("{origin}/company/"))
        .json(company)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
      Ok(())
    } else {
      Err("failed".into())
    }
}

pub async fn save_client(
    app_state: &AppState,
    client : &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .post(format!("{origin}/client/"))
        .json(client)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
      Ok(())
    } else {
      Err("failed".into())
    }
}

pub async fn save_sheet(
    app_state: &AppState,
    sheet : &Sheet,
) -> Result<(), Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .post(format!("{origin}/sheet/"))
        .json(sheet)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
      Ok(())
    } else {
      Err("failed".into())
    }
}

pub async fn save_bill(
    app_state: &AppState,
    bill : &Bill,
) -> Result<(), Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .post(format!("{origin}/bill/"))
        .json(bill)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
      Ok(())
    } else {
      Err("failed".into())
    }
}

pub async fn save_sellbill(
    app_state: &AppState,
    sellbill : &SellBill,
) -> Result<(), Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .post(format!("{origin}/sellbill/"))
        .json(sellbill)
        .send()
        .await?;

    if res.status() == StatusCode::OK {
      Ok(())
    } else {
      Err("failed".into())
    }
}

pub async fn find_clients(
    app_state: &AppState,
    name : &String,
) -> Result<Vec<Name>, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let clients = reqwest::Client::new()
        .get(format!("{origin}/client/{name}/find"))
        .send()
        .await?
	.json::<Vec<Name>>()
	.await?;

   Ok(clients)
}

pub async fn find_companies(
    app_state: &AppState,
    name : &String,
) -> Result<Vec<Name>, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let companies = reqwest::Client::new()
        .get(format!("{origin}/company/{name}/find"))
        .send()
        .await?
	.json::<Vec<Name>>()
	.await?;

    Ok(companies)
}

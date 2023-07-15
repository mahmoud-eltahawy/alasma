use models::backend_api::{
    Bill, Client, Company, Name, SellBill, Sheet, SheetShearchParams,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::AppState;

pub async fn save_company(
    app_state: &AppState,
    company: &Company,
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
    client: &Client,
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
    sheet: &Sheet,
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
    bill: &Bill,
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
    sellbill: &SellBill,
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
    name: &String,
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

pub async fn search_sheets(
    app_state: &AppState,
    params: &SheetShearchParams,
) -> Result<Vec<Sheet>, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let sheets = reqwest::Client::new()
        .post(format!("{origin}/sheet/search"))
        .json(params)
        .send()
        .await?
        .json::<Vec<Sheet>>()
        .await?;

    Ok(sheets)
}

pub async fn find_sheet_sellbills(
    app_state: &AppState,
    id: Uuid,
) -> Result<Vec<SellBill>, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let bills = reqwest::Client::new()
        .get(format!("{origin}/sellbill/{id}/sheet"))
        .send()
        .await?
        .json::<Vec<SellBill>>()
        .await?;

    Ok(bills)
}

pub async fn find_bill_by_id(
    app_state: &AppState,
    id: Uuid,
) -> Result<Bill, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let bills = reqwest::Client::new()
        .get(format!("{origin}/bill/{id}"))
        .send()
        .await?
        .json::<Bill>()
        .await?;

    Ok(bills)
}

pub async fn find_company_by_id(
    app_state: &AppState,
    id: Uuid,
) -> Result<Company, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let bills = reqwest::Client::new()
        .get(format!("{origin}/company/{id}"))
        .send()
        .await?
        .json::<Company>()
        .await?;

    Ok(bills)
}

pub async fn find_company_id_by_name(
    app_state: &AppState,
    name : String,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let bills = reqwest::Client::new()
        .get(format!("{origin}/company/{name}/id"))
        .send()
        .await?
        .json::<Uuid>()
        .await?;

    Ok(bills)
}

pub async fn find_client_by_id(
    app_state: &AppState,
    id: Uuid,
) -> Result<Client, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let bills = reqwest::Client::new()
        .get(format!("{origin}/client/{id}"))
        .send()
        .await?
        .json::<Client>()
        .await?;

    Ok(bills)
}

pub async fn find_client_id_by_name(
    app_state: &AppState,
    name : String,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let bills = reqwest::Client::new()
        .get(format!("{origin}/client/{name}/id"))
        .send()
        .await?
        .json::<Uuid>()
        .await?;

    Ok(bills)
}

pub async fn find_companies(
    app_state: &AppState,
    name: &String,
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

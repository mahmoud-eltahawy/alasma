// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::env;

use api::{
    find_bill_by_id, find_client_by_id, find_client_id_by_name,
    find_company_by_id, find_company_id_by_name, find_sheet_by_id,
    find_sheet_sellbills,
};
use chrono::Local;
use models::backend_api::*;
use uuid::Uuid;

mod api;
mod excel_writers;

use excel_writers::write_sells;

#[tauri::command]
fn new_id() -> Uuid {
    Uuid::new_v4()
}

#[tauri::command]
async fn save_company(
    app_state: tauri::State<'_, AppState>,
    company: Company,
) -> Result<(), String> {
    println!("Save Company -> {:#?}", company);
    match api::save_company(&app_state, &company).await {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn save_client(
    app_state: tauri::State<'_, AppState>,
    client: Client,
) -> Result<(), String> {
    println!("Save Client -> {:#?}", client);
    match api::save_client(&app_state, &client).await {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn save_sell_sheet(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
    name: String,
) -> Result<(), String> {
    let sheet = Sheet {
        id,
        the_name: name,
        the_date: Local::now().date_naive(),
        the_type: SheetType::Sells,
    };
    println!("Save Sheet -> {:#?}", sheet);
    match api::save_sheet(&app_state, &sheet).await {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn save_bill(
    app_state: tauri::State<'_, AppState>,
    bill: Bill,
) -> Result<(), String> {
    println!("Save Bill -> {:#?}", bill);
    match api::save_bill(&app_state, &bill).await {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn save_sell_bill(
    app_state: tauri::State<'_, AppState>,
    sellbill: SellBill,
) -> Result<(), String> {
    println!("Save Sell Bill -> {:#?}", sellbill);
    match api::save_sellbill(&app_state, &sellbill).await {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn top_5_companies(
    app_state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Vec<Name>, String> {
    println!("Search Company -> {:#?}", name);
    match api::find_companies(&app_state, &name).await {
        Ok(coms) => Ok(coms),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn top_5_clients(
    app_state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Vec<Name>, String> {
    println!("Search Client -> {:#?}", name);
    match api::find_clients(&app_state, &name).await {
        Ok(cls) => Ok(cls),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn top_5_sheets(
    app_state: tauri::State<'_, AppState>,
    params: SheetShearchParams,
) -> Result<Vec<Sheet>, String> {
    match api::search_sheets(&app_state, &params).await {
        Ok(sheets) => Ok(sheets),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_sheet_sellbills(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<SellBill>, String> {
    match find_sheet_sellbills(&app_state, id).await {
        Ok(bills) => Ok(bills),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_bill(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Bill, String> {
    match find_bill_by_id(&app_state, id).await {
        Ok(bills) => Ok(bills),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_company(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Company, String> {
    match find_company_by_id(&app_state, id).await {
        Ok(com) => Ok(com),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_company_id(
    app_state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Uuid, String> {
    println!("check company {}", name);
    match find_company_id_by_name(&app_state, name.trim().to_string()).await {
        Ok(com) => Ok(com),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_client(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Client, String> {
    match find_client_by_id(&app_state, id).await {
        Ok(clt) => Ok(clt),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_sheet(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Sheet, String> {
    match find_sheet_by_id(&app_state, id).await {
        Ok(sheet) => Ok(sheet),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_client_id(
    app_state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Uuid, String> {
    println!("check client {}", name);
    match find_client_id_by_name(&app_state, name.trim().to_string()).await {
        Ok(com) => Ok(com),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn write_sells_excel(
    app_state: tauri::State<'_, AppState>,
    sheetid: Uuid,
    sellbills: Vec<NaiveSellBill>,
) -> Result<(), String> {
    println!("write_sells_excel");
    match write_sells(&app_state, sheetid, sellbills).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            new_id,
            save_company,
            save_client,
            save_sell_sheet,
            save_bill,
            get_bill,
            get_company,
            get_client,
            get_sheet,
            save_sell_bill,
            top_5_companies,
            top_5_clients,
            top_5_sheets,
            get_client_id,
            get_company_id,
            get_sheet_sellbills,
            write_sells_excel,
        ])
        .manage(AppState::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AppState {
    pub origin: String,
}

impl Default for AppState {
    fn default() -> Self {
        let host = env::var("ERA_HOST").expect("invalid host key");
        let port = env::var("ERA_PORT").expect("invalid port key");

        AppState {
            origin: format!("http://{host}:{port}"),
        }
    }
}

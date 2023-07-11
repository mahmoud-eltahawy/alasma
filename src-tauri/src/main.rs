// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

use chrono::Local;
use models::backend_api::*;
use uuid::Uuid;

#[tauri::command]
fn new_id() -> Uuid {
    Uuid::new_v4()
}

#[tauri::command]
async fn save_company(
    app_state: tauri::State<'_, AppState>,
    company : Company
) -> Result<(), String> {
    println!("Save Company -> {:#?}", company);
    match api::save_company(&app_state, &company).await {
	Ok(()) => Ok(()),
	Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn save_client(
    app_state: tauri::State<'_, AppState>,
    client : Client
) -> Result<(), String> {
    println!("Save Client -> {:#?}", client);
    match api::save_client(&app_state, &client).await {
	Ok(()) => Ok(()),
	Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn save_sell_sheet(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
    name: String
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
	Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn save_bill(
    app_state: tauri::State<'_, AppState>,
    bill: Bill
) -> Result<(), String> {
    println!("Save Bill -> {:#?}", bill);
    match api::save_bill(&app_state, &bill).await {
	Ok(()) => Ok(()),
	Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn save_sell_bill(
    app_state: tauri::State<'_, AppState>,
    sellbill: SellBill
) -> Result<(), String> {
    println!("Save Sell Bill -> {:#?}", sellbill);
    match api::save_sellbill(&app_state, &sellbill).await {
	Ok(()) => Ok(()),
	Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn top_5_companies(
    app_state: tauri::State<'_, AppState>,
    name: String
) -> Result<Vec<Name>, String> {
    println!("Search Company -> {:#?}", name);
    match api::find_companies(&app_state, &name).await {
	Ok(coms) => Ok(coms),
	Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
async fn top_5_clients(
    app_state: tauri::State<'_, AppState>,
    name: String
) -> Result<Vec<Name>, String> {
    println!("Search Client -> {:#?}", name);
    match api::find_clients(&app_state, &name).await {
	Ok(cls) => Ok(cls),
	Err(err) => Err(err.to_string())
    }
}

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            new_id,
	    save_company,
	    save_client,
            save_sell_sheet,
            save_bill,
            save_sell_bill,
            top_5_companies,
            top_5_clients,
        ])
	.manage(AppState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AppState{
    pub origin : String,
}

impl AppState {
    pub fn new() -> Self {
        let host = env::var("ERA_HOST").expect("invalid host key");
        let port = env::var("ERA_PORT").expect("invalid port key");

        AppState {
            origin: format!("http://{host}:{port}"),
        }
    }
}

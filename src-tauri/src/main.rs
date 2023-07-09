// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use models::backend_api::{Client, Company, SellBill, Sheet, SheetType, Bill};
use uuid::Uuid;

#[tauri::command]
fn new_id() -> Uuid {
    Uuid::new_v4()
}

#[tauri::command]
fn save_company(company : Company) -> anyhow::Result<(), String> {
    println!("Save Company -> {:#?}", company);
    Ok(())
}

#[tauri::command]
fn save_client(client : Client) -> anyhow::Result<(), String> {
    println!("Save Client -> {:#?}", client);
    Ok(())
}

#[tauri::command]
fn save_sell_sheet(id: Uuid, name: String) -> anyhow::Result<(), String> {
    let sheet = Sheet {
        id,
        the_name: name,
        the_date: Local::now().date_naive(),
        the_type: SheetType::Sells,
    };
    println!("Save Sheet -> {:#?}", sheet);
    Ok(())
}

#[tauri::command]
fn save_bill(bill: Bill) -> anyhow::Result<(), String> {
    println!("Save Bill -> {:#?}", bill);
    Ok(())
}

#[tauri::command]
fn save_sell_bill(sellbill: SellBill) -> anyhow::Result<(), String> {
    println!("Save Sell Bill -> {:#?}", sellbill);
    Ok(())
}

#[tauri::command]
fn top_5_companies(name: String) -> anyhow::Result<Vec<Company>, String> {
    println!("Search Company -> {:#?}", name);
    Ok(vec![
        Company {
            id: Uuid::new_v4(),
            the_name: String::from("com 1"),
        },
        Company {
            id: Uuid::new_v4(),
            the_name: String::from("com 2"),
        },
        Company {
            id: Uuid::new_v4(),
            the_name: String::from("com 3"),
        },
    ])
}

#[tauri::command]
fn top_5_clients(name: String) -> anyhow::Result<Vec<Client>, String> {
    println!("Search Client -> {:#?}", name);
    Ok(vec![
        Client {
            id: Uuid::new_v4(),
            the_name: String::from("clt 1"),
        },
        Client {
            id: Uuid::new_v4(),
            the_name: String::from("clt 2"),
        },
        Client {
            id: Uuid::new_v4(),
            the_name: String::from("clt 3"),
        },
    ])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            new_id,
            save_sell_sheet,
            save_bill,
            save_sell_bill,
            top_5_companies,
            top_5_clients,
	    save_company,
	    save_client,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

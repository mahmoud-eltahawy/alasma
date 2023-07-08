// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use models::backend_api::{Sheet, SheetType, SellBill};
use uuid::Uuid;

#[tauri::command]
fn new_id() -> Uuid {
    Uuid::new_v4()
}

#[tauri::command]
fn save_sell_sheet(
    id : Uuid,
    name : String,
) -> anyhow::Result<(),String>{
    let sheet = Sheet{
	id,
	the_name : name,
	the_date : Local::now().date_naive(),
	the_type : SheetType::Sells,
    };
    println!("{:#?}",sheet);
    Ok(())
}

#[tauri::command]
fn save_sell_bill(
    sellbill : SellBill
) -> anyhow::Result<(),String>{
    println!("{:#?}",sellbill);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
	    new_id,
	    save_sell_sheet,
	    save_sell_bill,
	])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}

use leptos::window;
use serde::{Deserialize, Serialize};
use tauri_sys::tauri::invoke;
use uuid::Uuid;

pub mod component;
pub mod function;

#[derive(Serialize, Deserialize)]
pub struct NameArg {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Non;

pub async fn new_id() -> Uuid {
    invoke::<_, Uuid>("new_id", &Non {}).await.unwrap()
}

pub fn alert(s: &str) {
    let _ = window().alert_with_message(s);
}

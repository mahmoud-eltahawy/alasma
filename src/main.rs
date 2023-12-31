mod app;
mod boughts;
mod cargos;
mod sales;
mod shared;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

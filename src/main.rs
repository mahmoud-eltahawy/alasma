mod app;
mod sales;
mod boughts;
mod cargos;
mod shared;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}

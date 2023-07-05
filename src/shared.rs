use futures::{StreamExt, future};
use leptos::{spawn_local, window};
use serde::de::DeserializeOwned;
use tauri_sys::event::{self, Event};

pub fn listen<T : DeserializeOwned + 'static>(event_name : String,action : impl Fn(Event<T>) + 'static) {
    spawn_local(async move {
        if let Ok(events) = event::listen(&event_name).await {
            events.for_each(|event| {
                action(event);
                future::ready(())
            }).await;
        }
    })
}

pub fn alert(s: &str) {
    let _ = window().alert_with_message(s);
}

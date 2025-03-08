use std::sync::{Arc, Mutex};
use tauri::{command, AppHandle, EventLoopMessage, Manager, Runtime, State};
use tauri_plugin_store::{Store, StoreBuilder};
use tauri_runtime_wry::Wry;

pub struct StoreState(pub Mutex<Store<Wry<EventLoopMessage>>>);

pub fn init_store(
    app: &AppHandle,
) -> Result<Arc<Store<Wry<EventLoopMessage>>>, tauri_plugin_store::Error> {
    let app_dir = app
        .app_handle()
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");

    std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");

    let store_path = app_dir.join("settings.json");
    StoreBuilder::new(app, store_path).build()
}

#[command]
pub fn get_openai_api_key(state: State<'_, StoreState>) -> Result<String, String> {
    let store = state
        .inner()
        .0
        .lock()
        .map_err(|_| "Failed to lock store".to_string())?;

    match store.get("openai_api_key") {
        Some(value) => value
            .as_str()
            .map(String::from)
            .ok_or_else(|| "Invalid API key format".to_string()),
        None => Err("API key not found".to_string()),
    }
}

#[command]
pub fn set_openai_api_key(key: String, state: State<'_, StoreState>) -> Result<(), String> {
    let mut store = state
        .inner()
        .0
        .lock()
        .map_err(|_| "Failed to lock store".to_string())?;

    store.set("openai_api_key", key);
    store.save();
    Ok(())
}

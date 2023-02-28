#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use cvapi::{Checklist, CheckvistClient, Task};
// TODO: this is fine for now, but cvcap/cli may need rearranging to make the lib handy for
// external use 
use cvcap::creds;
use tauri::Manager;

#[tauri::command]
fn item_chosen(item_id: i32) -> String {
    println!("you chose {:?}", item_id);
    "Ok".into()
}

#[tauri::command]
fn get_lists() -> Vec<Checklist> {
    let token = creds::get_api_token_from_keyring("cvcap-api-token").unwrap();
    // TODO: find easiest way to present canned data
    // vec![Checklist{id: 1, name: "Test List".into(), updated_at: "".into(), task_count: 100}, Checklist{id: 2, name: "Listy stuffo".into(), updated_at: "".into(), task_count: 10}]
    let api_client = api_client(&token);
    api_client.get_lists().unwrap()
}

#[tauri::command]
fn get_tasks() -> Vec<Task> {
    let api_client = api_client("dummy");
    api_client.get_tasks(774394).unwrap()
}

fn api_client(token: &str) -> CheckvistClient {
    CheckvistClient::new(
        "https://beta.checkvist.com/",
        token,
        Box::new(|token| println!("token refresh: {}", token)),
    )
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                let size = tauri::LogicalSize::new(800, 1200);
                window.set_size(size).unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![item_chosen, get_tasks, get_lists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

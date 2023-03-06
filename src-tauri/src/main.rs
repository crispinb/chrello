#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use specta::{ Type, collect_types };
use tauri_specta::ts;
use serde::{ Serialize, Deserialize };
use cvapi::{Checklist, CheckvistClient, Task};
use cvcap::creds;
use tauri::Manager;

#[derive(Serialize, Type)]
struct UICard {
    id: u32,
    content: String
}

#[derive(Serialize, Type)]
struct UIColumn {
    name: String,
    id: u32,
    cards: Vec<UICard>
    }

#[derive(Serialize, Type)]
struct UIBoard {
    columns: Vec<UIColumn>
}

#[tauri::command]
#[specta::specta]
fn get_dummy_data() -> UIBoard {
    let card = UICard{ id: 1, content: "a card".into()};
    let card2 = UICard{ id: 12, content: "a card in anovva col".into()};
    let col = UIColumn{id: 1, name: "col1".into(), cards: vec!(card)};
    let col2 = UIColumn{id: 2, name: "col2".into(), cards: vec!(card2)};

    UIBoard{columns: vec!(col, col2)}
}

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
            // open devtools when not prod
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                let size = tauri::LogicalSize::new(800, 1200);
                window.set_size(size).unwrap();
                window.open_devtools();
            }

            // export specta types
            ts::export(collect_types![get_dummy_data], "../src/bindings.ts").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![item_chosen, get_tasks, get_lists, get_dummy_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

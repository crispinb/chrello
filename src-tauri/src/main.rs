#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use cvapi::{Checklist, CheckvistClient, Task};
use cvcap::creds;
use random_string::generate;
use serde::Serialize;
use specta::{collect_types, Type};
use tauri::{Manager, State, Window};
use tauri_specta::ts;


struct SimpleState(Mutex<i32>);

#[derive(Clone, Serialize, Type)]
struct UICard {
    id: u32,
    content: String,
}

#[derive(Clone, Serialize, Type)]
struct UIColumn {
    name: String,
    id: u32,
    cards: Vec<UICard>,
}

#[derive(Clone, Serialize, Type)]
struct UIBoard {
    columns: Vec<UIColumn>,
}

#[tauri::command]
#[specta::specta]
fn get_dummy_data() -> UIBoard {
    let card_text = || {
        generate(
            40,
            "lasjlkja flkj123l4j 9asdofasdf1  asdfasoryuqoj1lk3j4oiasdf",
        )
    };
    let card = UICard {
        id: 1,
        content: card_text(),
    };
    let card2 = UICard {
        id: 12,
        content: card_text(),
    };
    let col = UIColumn {
        id: 1,
        name: "col1".into(),
        cards: vec![card],
    };
    let col2 = UIColumn {
        id: 2,
        name: "col2".into(),
        cards: vec![card2.clone()],
    };
    let col3 = UIColumn {
        id: 3,
        name: "col3".into(),
        cards: vec![card2],
    };

    UIBoard {
        columns: vec![col, col2, col3],
    }
}

#[tauri::command]
#[specta::specta]
fn init_timer(window: Window) {
    println!("(rust) initting the timer)");
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(8));
        window.emit("test-event", get_dummy_data()).unwrap();
    });
}

#[tauri::command]
fn item_chosen(item_id: i32) -> String {
    println!("you chose {:?}", item_id);
    "Ok".into()
}

#[tauri::command]
fn get_lists() -> Vec<Checklist> {
    let token = creds::get_api_token_from_keyring("cvcap-api-token").unwrap();
    // vec![Checklist{id: 1, name: "Test List".into(), updated_at: "".into(), task_count: 100}, Checklist{id: 2, name: "Listy stuffo".into(), updated_at: "".into(), task_count: 10}]
    let api_client = api_client(&token);
    api_client.get_lists().unwrap()
}

#[tauri::command]
fn get_tasks() -> Vec<Task> {
    let api_client = api_client("dummy");
    api_client.get_tasks(774394).unwrap()
}

#[tauri::command]
fn get_some_state(count: State<SimpleState>) -> i32 {
    let c = *count.0.lock().unwrap();
    c
}

// I don't understand the magic involved with these commands
// where do the window and state args come from? I'm not passing them
// And I don't have them at all on some commands
#[specta::specta]
#[tauri::command]
fn set_state(window: Window, new_count: i32, count: State<SimpleState>) {
    println!("rust: --> received state {}", new_count);
    *count.0.lock().unwrap() = new_count;
    window.emit("new-state", new_count).unwrap();
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
                let size = tauri::LogicalSize::new(1000, 1200);
                window.set_size(size).unwrap();
                window.open_devtools();
            }
            // export specta types
            ts::export(collect_types![ init_timer, get_dummy_data, set_state], "../src/bindings.ts").unwrap();
            Ok(())
        })
        .manage(SimpleState(Mutex::new(42)))
        .invoke_handler(tauri::generate_handler![
            init_timer,
            item_chosen,
            get_tasks,
            get_lists,
            get_dummy_data,
            get_some_state,
            set_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

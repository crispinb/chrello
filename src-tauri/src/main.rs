#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use fake::{Dummy, Fake, Faker};
use serde::Serialize;
use specta::{collect_types, Type};
use tauri::{Manager, State, Window};
use tauri_specta::ts;

// use cvapi::{Checklist, CheckvistClient, Task};
// use cvcap::creds;

struct BoardData(Mutex<UIBoard>);

#[derive(Dummy, Debug, Clone, Serialize, Type)]
struct UICard {
    id: u32,
    content: String,
}

#[derive(Dummy, Debug, Clone, Serialize, Type)]
struct UIColumn {
    name: String,
    id: u32,
    #[dummy(faker = "(Faker, 0..20)")]
    cards: Vec<UICard>,
}

#[derive(Dummy, Debug, Clone, Serialize, Type)]
struct UIBoard {
    #[dummy(faker = "(Faker, 1..5)")]
    columns: Vec<UIColumn>,
}

impl std::default::Default for UIBoard {
    fn default() -> Self {
        Faker.fake()
    }
}

#[tauri::command]
#[specta::specta]
// TODO: this is just for testing. It probably won't work like this
fn load_initial_data(window: Window, board: State<BoardData>) {
    let b = board.0.lock().unwrap().clone();
   // TODO: can we use specta to type events 
    window.emit("initial-data", b).unwrap();
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
            ts::export(collect_types![load_initial_data], "../src/bindings.ts").unwrap();
            Ok(())
        })
        .manage(BoardData(Mutex::new(UIBoard::default())))
        .invoke_handler(tauri::generate_handler![load_initial_data,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod test {
    use super::*;
}

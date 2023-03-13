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


#[derive(Dummy, Debug, Clone, Serialize, Type)]
struct UICard {
    id: u32,
    content: String,
}

#[derive(Dummy,  Debug, Clone, Serialize, Type)]
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

#[tauri::command]
#[specta::specta]
fn get_dummy_data() -> UIBoard {
    Faker.fake()
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
            ts::export(collect_types![ get_dummy_data ], "../src/bindings.ts").unwrap();
            Ok(())
        })
        .manage(SimpleState(Mutex::new(42)))
        .invoke_handler(tauri::generate_handler![
            get_dummy_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testtest() {
        let board=  get_dummy_data();
        println!("board: {:?}", board);
        assert_eq!(4, board.columns.len());
        assert_eq!(4, board.columns[0].cards.len());
    }
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_dirs;
mod get_disks;

use get_dirs::get_dirs;
use get_disks::get_disks;
use std::collections::HashMap;
use tauri::{CustomMenuItem, Menu, Submenu};

//TODO clean up the unwrapped results
//TODO make files unclickable
//TODO useRefs or something to store the expensive folder calculations

#[tauri::command]
fn disks() -> HashMap<String, u64> {
    let mut disks: HashMap<String, u64> = HashMap::new();
    get_disks(&mut disks);
    disks
}

#[tauri::command]
fn dirs(dir: &str) -> HashMap<String, u64> {
    let mut dirs: HashMap<String, u64> = HashMap::new();
    get_dirs(&mut dirs, dir);
    dirs
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(close));
    let menu = Menu::new()
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "close" => {
                println!("in close");
                event.window().close().unwrap()
            }
            "hide" => event.window().minimize().unwrap(),
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![disks, dirs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

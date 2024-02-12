// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_handler;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> Vec<db_handler::Bill> {
    // 创建数据库
    let db_handler_struct = db_handler::DbHandlerStruct::new("bill.db");
    let _ = db_handler_struct.create_db();
    db_handler_struct.get_bill().unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

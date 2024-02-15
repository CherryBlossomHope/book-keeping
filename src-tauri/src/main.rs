// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_handler;

fn main() {
    // 创建数据库目录
    db_handler::create_db_dir();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            db_handler::render_get_bill,
            db_handler::render_get_bill_details
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use dirs::data_local_dir;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bill {
    id: i32,
    date: i32,
    total_amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillDetails {
    id: i32,
    bill_type: String,
    amount: i32,
    bill_id: i32,
}

#[derive(Debug)]
pub struct DbHandlerStruct {
    db: Connection,
}

impl DbHandlerStruct {
    pub fn new() -> DbHandlerStruct {
        // 根据操作系统获取存储路径
        let mut app_data_path = data_local_dir().unwrap();
        // 拼接路径
        app_data_path.push("BookKeeping/bill.db");
        DbHandlerStruct {
            db: Connection::open(&app_data_path).unwrap(),
        }
    }

    pub fn create_db(&self) -> rusqlite::Result<()> {
        self.db.execute(
            "create table if not exists bill (
                 id integer primary key,
                 date integer not null unique,
                 total_amount integer not null
             )",
            [],
        )?;
        self.db.execute(
            "create table if not exists bill_item (
                 id integer primary key,
                 type text not null,
                 amount integer not null,
                 bill_id integer not null references bill (id)
             )",
            [],
        )?;
        Ok(())
    }

    pub fn get_bill(&self) -> rusqlite::Result<Vec<Bill>> {
        let mut res = self.db.prepare("SELECT id, date, total_amount FROM bill")?;
        let mut bill_vac: Vec<Bill> = Vec::new();

        let bill_iter = res.query_map([], |row| {
            Ok(Bill {
                id: row.get(0)?,
                date: row.get(1)?,
                total_amount: row.get(2)?,
            })
        })?;

        for b in bill_iter {
            if let Ok(bill) = b {
                bill_vac.push(bill)
            }
        }
        Ok(bill_vac)
    }

    pub fn get_bill_details(&self, id: i32) -> rusqlite::Result<Vec<BillDetails>> {
        let mut sql =
            String::from("SELECT id, type, amount, bill_id FROM bill_item WHERE bill_id = ");
        sql.push_str(&id.to_string());

        let mut res = self.db.prepare(&sql)?;
        let mut bill_vac: Vec<BillDetails> = Vec::new();

        let bill_iter = res.query_map([], |row| {
            Ok(BillDetails {
                id: row.get(0)?,
                bill_type: row.get(1)?,
                amount: row.get(2)?,
                bill_id: row.get(3)?,
            })
        })?;

        for b in bill_iter {
            if let Ok(bill) = b {
                bill_vac.push(bill)
            }
        }
        Ok(bill_vac)
    }
}

/** 创建程序配置目录 */
pub fn create_db_dir() {
    // 根据操作系统获取存储路径
    let mut app_data_path = data_local_dir().unwrap();
    // 拼接路径
    app_data_path.push("BookKeeping");
    // 检查路径是否存在
    let dir_exist = match fs::metadata(&app_data_path) {
        Ok(_) => true,
        Err(_) => false,
    };
    // 创建文件
    if !dir_exist {
        fs::create_dir(app_data_path).unwrap();
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn render_get_bill() -> Vec<Bill> {
    let db_handler_struct = DbHandlerStruct::new();
    let _ = db_handler_struct.create_db();
    db_handler_struct.get_bill().unwrap()
}

#[tauri::command]
pub fn render_get_bill_details(id: i32) -> Vec<BillDetails> {
    let db_handler_struct = DbHandlerStruct::new();
    db_handler_struct.get_bill_details(id).unwrap()
}

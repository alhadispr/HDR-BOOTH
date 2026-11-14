#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use base64::Engine;

#[derive(Serialize)]
struct AppInfo { name: &'static str, version: &'static str, platform: &'static str }
#[derive(Serialize)]
struct Session { id: i64, name: String, event_type: String, created_at: String, status: String, photo_count: i64 }
#[derive(Serialize)]
struct Capture { id: i64, session_id: i64, file_path: String, created_at: String, print_copies: i64, sync_status: String }

fn db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let mut dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    dir.push("hdr_booth.sqlite3");
    let conn = Connection::open(dir).map_err(|e| e.to_string())?;
    conn.execute_batch(include_str!("../migrations/001_init.sql")).map_err(|e| e.to_string())?;
    Ok(conn)
}

#[tauri::command]
fn app_info() -> AppInfo { AppInfo { name: "HDR BOOTH Professional", version: "1.0.0", platform: "Windows desktop" } }

#[tauri::command]
fn local_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let p = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command]
fn create_session(app: tauri::AppHandle, name: String, event_type: Option<String>) -> Result<i64, String> {
    let conn = db(&app)?;
    conn.execute("INSERT INTO sessions (name,event_type) VALUES (?1,?2)", params![name, event_type.unwrap_or("standard".into())]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
fn list_sessions(app: tauri::AppHandle) -> Result<Vec<Session>, String> {
    let conn = db(&app)?;
    let mut stmt = conn.prepare("SELECT s.id,s.name,s.event_type,s.created_at,s.status,COUNT(c.id) FROM sessions s LEFT JOIN captures c ON c.session_id=s.id GROUP BY s.id ORDER BY s.id DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(Session { id:r.get(0)?, name:r.get(1)?, event_type:r.get(2)?, created_at:r.get(3)?, status:r.get(4)?, photo_count:r.get(5)? })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn close_session(app: tauri::AppHandle, session_id: i64) -> Result<(), String> {
    let conn = db(&app)?;
    let changed = conn.execute("UPDATE sessions SET status='closed' WHERE id=?1 AND status='active'", params![session_id]).map_err(|e| e.to_string())?;
    if changed == 0 { return Err("Sesi tidak ditemukan atau sudah ditutup".into()); }
    Ok(())
}

#[tauri::command]
fn reopen_session(app: tauri::AppHandle, session_id: i64) -> Result<(), String> {
    let conn = db(&app)?;
    let changed = conn.execute("UPDATE sessions SET status='active' WHERE id=?1", params![session_id]).map_err(|e| e.to_string())?;
    if changed == 0 { return Err("Sesi tidak ditemukan".into()); }
    Ok(())
}

#[tauri::command]
fn add_capture(app: tauri::AppHandle, session_id: i64, file_path: String, print_copies: Option<i64>) -> Result<i64, String> {
    let conn = db(&app)?;
    conn.execute("INSERT INTO captures(session_id,file_path,print_copies) VALUES (?1,?2,?3)", params![session_id,file_path,print_copies.unwrap_or(1)]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
fn list_captures(app: tauri::AppHandle, session_id: i64) -> Result<Vec<Capture>, String> {
    let conn = db(&app)?;
    let mut stmt = conn.prepare("SELECT id,session_id,file_path,created_at,print_copies,sync_status FROM captures WHERE session_id=?1 ORDER BY id DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![session_id], |r| Ok(Capture { id:r.get(0)?, session_id:r.get(1)?, file_path:r.get(2)?, created_at:r.get(3)?, print_copies:r.get(4)?, sync_status:r.get(5)? })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_setting(app: tauri::AppHandle, key: String, value: String) -> Result<(), String> {
    let conn = db(&app)?;
    conn.execute("INSERT INTO app_settings(key,value) VALUES (?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![key,value]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_setting(app: tauri::AppHandle, key: String) -> Result<Option<String>, String> {
    let conn = db(&app)?;
    conn.query_row("SELECT value FROM app_settings WHERE key=?1", params![key], |r| r.get(0)).optional().map_err(|e| e.to_string())
}

use rusqlite::OptionalExtension;


#[tauri::command]
fn save_capture(app: tauri::AppHandle, session_id: i64, data_url: String, print_copies: Option<i64>) -> Result<String, String> {
    let prefix = "data:image/jpeg;base64,";
    let encoded = data_url.strip_prefix(prefix).ok_or("Format foto tidak valid")?;
    let bytes = base64::engine::general_purpose::STANDARD.decode(encoded).map_err(|e| e.to_string())?;
    if bytes.len() > 25 * 1024 * 1024 { return Err("Ukuran foto melebihi 25 MB".into()); }
    let conn = db(&app)?;
    let session_name: String = conn.query_row("SELECT name FROM sessions WHERE id=?1 AND status='active'", params![session_id], |row| row.get(0)).map_err(|_| "Sesi tidak ditemukan atau sudah ditutup")?;
    let safe: String = session_name.chars().map(|c| if c.is_ascii_alphanumeric() || c=='-' || c=='_' { c } else { '_' }).collect();
    let mut dir = app.path().picture_dir().map_err(|e| e.to_string())?;
    dir.push("HDR BOOTH");
    dir.push(if safe.is_empty() { "Session" } else { &safe });
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let stamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
    let filename = format!("HDR_{}.jpg", stamp);
    let path = dir.join(filename);
    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    let path_string = path.to_string_lossy().to_string();
    if let Err(e) = conn.execute("INSERT INTO captures(session_id,file_path,print_copies) VALUES (?1,?2,?3)", params![session_id,path_string,print_copies.unwrap_or(1)]) {
        let _ = std::fs::remove_file(&path);
        return Err(e.to_string());
    }
    Ok(path_string)
}


#[derive(Serialize)]
struct Voucher {
    id: i64, code: String, discount_type: String, value: i64,
    max_uses: Option<i64>, used_count: i64, expires_at: Option<String>,
    active: bool, created_at: String
}

#[tauri::command]
fn list_vouchers(app: tauri::AppHandle) -> Result<Vec<Voucher>, String> {
    let conn = db(&app)?;
    let mut stmt = conn.prepare(
      "SELECT id,code,discount_type,value,max_uses,used_count,expires_at,active,created_at FROM vouchers ORDER BY id DESC"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |x| Ok(Voucher {
      id:x.get(0)?, code:x.get(1)?, discount_type:x.get(2)?, value:x.get(3)?,
      max_uses:x.get(4)?, used_count:x.get(5)?, expires_at:x.get(6)?,
      active:x.get::<_,i64>(7)? != 0, created_at:x.get(8)?
    })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn create_voucher(app: tauri::AppHandle, code: String, discount_type: String, value: i64,
                  max_uses: Option<i64>, expires_at: Option<String>) -> Result<i64, String> {
    let code = code.trim().to_ascii_uppercase();
    if code.len() < 3 || code.len() > 32 || !code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
      return Err("Kode harus 3-32 karakter: huruf, angka, - atau _".into());
    }
    if value <= 0 { return Err("Nilai diskon harus lebih dari 0".into()); }
    if discount_type == "percent" && value > 100 { return Err("Diskon persentase maksimal 100%".into()); }
    if discount_type != "percent" && discount_type != "fixed" { return Err("Jenis diskon tidak valid".into()); }
    if max_uses.is_some_and(|n| n <= 0) { return Err("Batas pemakaian harus lebih dari 0".into()); }
    let conn = db(&app)?;
    conn.execute("INSERT INTO vouchers(code,discount_type,value,max_uses,expires_at) VALUES (?1,?2,?3,?4,?5)",
      params![code,discount_type,value,max_uses,expires_at.filter(|x| !x.trim().is_empty())])
      .map_err(|e| if e.to_string().contains("UNIQUE") { "Kode voucher sudah digunakan".into() } else { e.to_string() })?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
fn set_voucher_active(app: tauri::AppHandle, voucher_id: i64, active: bool) -> Result<(), String> {
    let conn = db(&app)?;
    let n = conn.execute("UPDATE vouchers SET active=?1 WHERE id=?2", params![if active {1} else {0},voucher_id]).map_err(|e| e.to_string())?;
    if n == 0 { return Err("Voucher tidak ditemukan".into()); }
    Ok(())
}

#[derive(Serialize)]
struct VoucherQuote { code: String, discount_amount: i64, final_amount: i64 }

#[tauri::command]
fn validate_voucher(app: tauri::AppHandle, code: String, subtotal: i64) -> Result<VoucherQuote, String> {
    if subtotal < 0 { return Err("Subtotal tidak valid".into()); }
    let conn = db(&app)?;
    let row: (String,i64,Option<i64>,i64,Option<String>,i64) = conn.query_row(
      "SELECT discount_type,value,max_uses,used_count,expires_at,active FROM vouchers WHERE code=?1",
      params![code.trim().to_ascii_uppercase()],
      |x| Ok((x.get(0)?,x.get(1)?,x.get(2)?,x.get(3)?,x.get(4)?,x.get(5)?))
    ).map_err(|_| "Kode voucher tidak ditemukan".to_string())?;
    if row.5 == 0 { return Err("Voucher tidak aktif".into()); }
    if row.2.is_some_and(|limit| row.3 >= limit) { return Err("Batas pemakaian voucher telah tercapai".into()); }
    if let Some(expiry) = row.4 {
      let today = chrono::Local::now().format("%Y-%m-%d").to_string();
      if expiry < today { return Err("Voucher sudah kedaluwarsa".into()); }
    }
    let discount = if row.0 == "percent" { subtotal.saturating_mul(row.1).saturating_div(100) } else { row.1 };
    let discount = discount.min(subtotal);
    Ok(VoucherQuote { code: code.trim().to_ascii_uppercase(), discount_amount: discount, final_amount: subtotal - discount })
}

#[tauri::command]
fn redeem_voucher(app: tauri::AppHandle, code: String, subtotal: i64, session_id: Option<i64>) -> Result<VoucherQuote, String> {
    let mut conn = db(&app)?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let normalized = code.trim().to_ascii_uppercase();
    let row: (i64,String,i64,Option<i64>,i64,Option<String>,i64) = tx.query_row(
      "SELECT id,discount_type,value,max_uses,used_count,expires_at,active FROM vouchers WHERE code=?1",
      params![normalized], |x| Ok((x.get(0)?,x.get(1)?,x.get(2)?,x.get(3)?,x.get(4)?,x.get(5)?,x.get(6)?))
    ).map_err(|_| "Kode voucher tidak ditemukan".to_string())?;
    if row.6 == 0 { return Err("Voucher tidak aktif".into()); }
    if row.3.is_some_and(|limit| row.4 >= limit) { return Err("Batas pemakaian voucher telah tercapai".into()); }
    if row.5.as_ref().is_some_and(|d| d.as_str() < chrono::Local::now().format("%Y-%m-%d").to_string().as_str()) {
      return Err("Voucher sudah kedaluwarsa".into());
    }
    if subtotal < 0 { return Err("Subtotal tidak valid".into()); }
    let discount = if row.1 == "percent" { subtotal.saturating_mul(row.2).saturating_div(100) } else { row.2 };
    let discount = discount.min(subtotal);
    tx.execute("UPDATE vouchers SET used_count=used_count+1 WHERE id=?1", params![row.0]).map_err(|e| e.to_string())?;
    tx.execute("INSERT INTO voucher_redemptions(voucher_id,session_id,subtotal,discount_amount,final_amount) VALUES (?1,?2,?3,?4,?5)",
      params![row.0,session_id,subtotal,discount,subtotal-discount]).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(VoucherQuote { code: normalized, discount_amount: discount, final_amount: subtotal-discount })
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![app_info, local_data_dir, create_session, list_sessions, close_session, reopen_session, add_capture, list_captures, save_capture, set_setting, get_setting, list_vouchers, create_voucher, set_voucher_active, validate_voucher, redeem_voucher])
    .run(tauri::generate_context!())
    .expect("error while running HDR BOOTH");
}

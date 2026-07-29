use std::sync::Mutex;

use tauri::State;

mod histogram;

use histogram::{Data, Histogram};

pub type AppData = Mutex<Data>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(Mutex::new(Data::new()))
        .invoke_handler(tauri::generate_handler![command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn command(state: State<'_, AppData>, cmd: &str) -> Result<Histogram, String> {
    let mut data = state.lock().unwrap();
    match cmd {
        "clear" => data.clear(),
        "rectangular" => data.add_rectangular(),
        "ushaped" => data.add_ushaped(),
        _ => return Err("Illegal command".to_string()),
    }

    let histogram = data.create_histogram();
    Ok(histogram)
}

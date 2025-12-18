use std::sync::Mutex;

use lib::{Data, Histogram};
use tauri::State;

struct AppState(Mutex<Data>);

#[tauri::command]
fn run_command(command: &str, state: State<AppState>) -> Histogram {
    let mut data = state.0.lock().unwrap();
    match command {
        "clear" => data.clear(),
        "rectangular" => data.add_rectangular(),
        "ushaped" => data.add_ushaped(),
        _ => (),
    };

    data.create_histogram()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(Data::new())))
        .invoke_handler(tauri::generate_handler![run_command])
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

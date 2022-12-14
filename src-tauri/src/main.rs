#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::Builder::default()
  .run(tauri::generate_context!())
  .expect("msg: Failed to initialize Tauri");
}

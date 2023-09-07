use std::thread;

mod content;
mod setup;
mod menu;
mod command;
mod clipboard;

// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]


fn main() {

  let (menu, tray) = menu::generate_menu();
  let context = tauri::generate_context!();

  thread::spawn(|| {
    clipboard::clipboard_listen();
  });   

  tauri::Builder::default()
    .menu(menu)
    .system_tray(tray)
    .on_system_tray_event(menu::generate_tray)
    .invoke_handler(tauri::generate_handler![command::sync_html, command::sync_md])
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(setup::init)
    .run(context)
    .expect("error while running tauri application");


}




use std::thread;

mod content;
mod setup;
mod menu;
mod command;
mod clipboard;
mod logger;

// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]




fn main() {
  logger::init();

  let (menu, tray) = menu::generate_menu();
  let context = tauri::generate_context!();

  // thread::spawn(|| {
  //   clipboard::clipboard_listen();
  // });   

  tauri::Builder::default()
    .menu(menu)
    .system_tray(tray)
    .on_system_tray_event(menu::generate_tray)
    .invoke_handler(tauri::generate_handler![
      command::sync_html, 
      command::sync_md, 
      command::get_clipboard,
      command::get_older,
      command::do_copy,
      command::do_paste
      ])
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(setup::init)
    // .manage(state)
    .run(context)
    .expect("error while running tauri application");


}




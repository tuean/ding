use logger::init_logger;


mod content;
mod setup;
mod menu;
mod command;
mod clipboard;
mod logger;
mod keyboard;
mod util;

fn main() {
  init_logger();

  let (menu, tray) = menu::generate_menu();
  let context = tauri::generate_context!();

  tauri::Builder::default()
    .menu(menu)
    .system_tray(tray)
    .on_system_tray_event(menu::generate_tray)
    .invoke_handler(tauri::generate_handler![
      command::sync_html, 
      command::sync_md, 
      command::get_clipboard,
      command::get_older,
      command::do_paste
      ])
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(setup::init)
    // .manage(state)
    .run(context)
    .expect("error while running tauri application");


}




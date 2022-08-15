use tauri::{Menu, MenuItem, Submenu, SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;
use content::{
  sync_content, sync_source
};

mod content;

// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

#[tauri::command]
fn sync_html(content: String) {
  println!("invoked from JS! {}", content);
  sync_content(content)
}

#[tauri::command]
fn sync_md(content: String) {
  println!("invoked from JS! {}", content);
  sync_source(content)
}



fn main() {

  let shortcut = CustomMenuItem::new("shortcut".to_string(), "shortcut");
  let devtools = CustomMenuItem::new("devtools".to_string(), "devtools");
  let show = CustomMenuItem::new("show".to_string(), "show");
  let hide = CustomMenuItem::new("hide".to_string(), "hide");
  let quit = CustomMenuItem::new("quit".to_string(), "quit");
  let tray_menu = SystemTrayMenu::new()
    .add_item(devtools)
    .add_item(shortcut)
    .add_item(hide)
    .add_item(show)
    .add_item(quit);
    // .add_native_item(SystemTrayMenuItem::Separator)

  let tray = SystemTray::new().with_menu(tray_menu);

  let submenu_main = Submenu::new(
    "Ding".to_string(),
    Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::CloseWindow)
      .add_native_item(MenuItem::Quit),
  );

  let menu = Menu::new().add_submenu(submenu_main);

  tauri::Builder::default()
    .menu(menu)
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event{
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          },
          "hide" => {
            app.get_window("main").unwrap().hide().unwrap();
          }
          "show" => {
            app.get_window("main").unwrap().show().unwrap();
          },
          "shortcut" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          "devtools" => {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![sync_html, sync_md])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



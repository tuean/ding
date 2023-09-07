use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, SystemTray, SystemTrayMenu, SystemTrayEvent,AppHandle};
use tauri::Manager;

pub fn generate_menu() -> (Menu, SystemTray) {
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

    let menu: Menu = Menu::new().add_submenu(submenu_main);
    (menu, tray)
}


pub fn generate_tray(app: &AppHandle, event: SystemTrayEvent) {
    // app, event: SystemTrayEvent| 
    match event{
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
      }
}
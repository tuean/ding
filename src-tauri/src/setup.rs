
use std::{thread};

use clipboard_rs::{ClipboardWatcher, ClipboardWatcherContext};
use tauri::{
    App, AppHandle, GlobalShortcutManager, LogicalPosition, LogicalSize, 
    Manager, Position, Size
};

use crate::clipboard::listen::{Manager as ClipboardManager};
use crate::clipboard::store::init_table;

pub type AppError = Box<(dyn std::error::Error + 'static)>;
pub type SetupResult = Result<(), AppError>;

// pub const NEW_CLIP: &'static str = "CLIPBOARD_UPDATE";

/// 设置窗口
fn set_window_main(app: &mut App) -> SetupResult {
    let win = app.get_window("main").unwrap();

    let monitor = win.current_monitor()
    .expect("failed to get monitor info")
    .expect("failed to get monitor info");

    let pos = monitor.position();

    win.set_position(Position::Physical(
        tauri::PhysicalPosition{
            x: pos.x,
            y: 0
        })
    )?; 
    let _ = win.hide();
    Ok(())
}

fn set_window_clipboard(app: &mut App) -> SetupResult {
    let win = app.get_window("clipboard").unwrap();
    const HEIGHT: f64 = 250.0;
    // 设置大小和位置
    let monitor = win.current_monitor()
        .expect("failed to get monitor info")
        .expect("failed to get monitor info");
    let screen_size = monitor.size().to_logical::<f64>(monitor.scale_factor());
    win.set_size(Size::Logical(LogicalSize::new(screen_size.width as f64, HEIGHT)))?;
    win.set_position(Position::Logical(LogicalPosition::new(0.0, screen_size.height as f64 - HEIGHT)))?;
    // 设置窗口层级为顶级（高于菜单栏和Dock，Tauri默认设置会在Dock栏下面）
    // WindowUtil::set_window_top_level(&win);
    let _ = win.set_always_on_top(true);
    let _ = win.hide();
    Ok(())
}

/// 注册全局快捷键
fn register_shortcut(app: &mut App) -> SetupResult {
    let mut short_cut = app.global_shortcut_manager();
    let app_handler = app.handle();
    short_cut.register("ctrl+alt+v", move || {
        let window = app_handler.get_window("clipboard").unwrap();
        println!("swtich action detected");
        if window.is_visible().unwrap() {
            let _ = window.hide();
        } else {
            let _ = broadcast_new_clipboard_event(&app_handler);
            let _ = window.show();
            window.set_focus().unwrap();
        }
    })?;
    Ok(())
}


pub fn init_app_handle(app:&mut App) -> Result<(), SetupResult> {
    let _ = init_table(); 

    let manager: ClipboardManager = ClipboardManager::new(app);

    thread::spawn(move || {
        let mut watcher: ClipboardWatcherContext<ClipboardManager> = ClipboardWatcherContext::new().unwrap();
        let _: clipboard_rs::WatcherShutdown = watcher.add_handler(manager).get_shutdown_channel();
        println!("start watch!");
        watcher.start_watch()
    });


    println!("init app handle finish");
    Ok(())
}

pub fn broadcast_new_clipboard_event(app_handle: &AppHandle) -> Result<(), SetupResult> {
    // let app_handle = app.app_handle();
    let r = app_handle.emit_all("CLIPBOARD_UPDATE", ());
    match r {
        Ok(_) => println!("event send ok"),
        Err(_) => println!("event send error"),
    }
    println!("broadcast a new event");
    Ok(())
}

// fn move_window_to_other_monitor(window: &Window, i: usize) -> tauri::Result<()> {
//     let monitors = window.available_monitors()?;
//     let monitor = monitors.get(i).ok_or(tauri::Error::CreateWindow)?;
  
//     let pos = monitor.position();
  
//     window.set_position(Position::Physical(
//         tauri::PhysicalPosition{
//             x: pos.x,
//             y: 0
//         })
//     )?;
  
//     window.center()?;
//     Ok(())
//   }

pub fn init(app: &mut App) -> SetupResult {
    set_window_main(app)?;
    set_window_clipboard(app)?;
    register_shortcut(app)?;
    let _ = init_app_handle(app);
    Ok(())
}
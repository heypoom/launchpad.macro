#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::thread;
use crossbeam::channel::unbounded;
use tauri::{GlobalShortcutManager, Manager, WindowBuilder, WindowUrl};

fn main() {
  let url = WindowUrl::App("index.html".into());

  tauri::Builder::default()
    .create_window("macropad", url, move |window_builder, webview_attr| {
      let window = window_builder
        .title("Macro Pad")
        .inner_size(400.0, 400.0)
        .transparent(true)
        .always_on_top(true)
        .center()
        .skip_taskbar(true)
        .decorations(false)
        .resizable(true)
        .visible(false);

      (window, webview_attr)
    })
    .setup(|app| {
      let (tx, rx) = unbounded();

      let mut shortcut = app.global_shortcut_manager();
      let mut macropad_window = app.get_window("macropad").unwrap();

      thread::spawn(move || {
        let tx = tx.clone();

        shortcut.register("Ctrl+Shift+M", move || {
          println!("Ctrl+Shift+M");

          tx.send("toggle_window").unwrap();
        }).unwrap();
      });

      thread::spawn(move || {
        let mut visible = false;
        let rx = rx.clone();

        loop {
          let event = rx.recv().unwrap();
          println!("event: {}", event);

          visible = !visible;

          if visible {
            macropad_window.show();
          } else {
            macropad_window.hide();
          }
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

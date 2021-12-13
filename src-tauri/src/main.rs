#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod game;
mod flappy;
mod launchpad;

#[macro_use]
extern crate text_io;
extern crate midir;

use std::thread;
use std::time::Duration;
use crossbeam::channel::unbounded;
use tauri::{GlobalShortcutManager, Manager, Window, WindowBuilder, WindowUrl};
use crate::game::start_game;

#[derive(serde::Serialize, serde::Deserialize)]
struct KeyPressAction {
  pos: u32,
}

#[tauri::command]
fn on_keypress(action: KeyPressAction) {
  println!("Keypress was received: {}", action.pos);
}

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
        .visible(true);

      (window, webview_attr)
    })
    .invoke_handler(tauri::generate_handler![on_keypress])
    .setup(|app| {
      let (tx, rx) = unbounded();

      let mut shortcut = app.global_shortcut_manager();
      let mut macropad_window = app.get_window("macropad").unwrap();

      thread::spawn(move || {
        start_game();
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

      thread::spawn(move || {
        let tx = tx.clone();

        thread::sleep(Duration::from_millis(500));

        shortcut.register("Ctrl+Shift+M", move || {
          println!("Ctrl+Shift+M");

          tx.send("toggle_window").unwrap();
        }).unwrap();
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

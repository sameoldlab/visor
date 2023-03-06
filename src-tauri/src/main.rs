// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::{BufRead, BufReader}, 
    process::{Command, Stdio},
    sync::mpsc::{channel, Sender},
    thread
};
use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
    Manager, Window, RunEvent, WindowEvent
  };


#[derive(Clone, serde::Serialize)]
struct AppState {
    anvil_output: String,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

async fn _start_anvil(sender: Sender<String>) -> u32 {
    let child = Command::new("anvil")
                    .args(["-b","15"])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("anvil failed to run");

    println!("Started process {}", child.id());
    let process = child.id();
    thread::spawn(move || {
        let mut f = BufReader::new(child.stdout.unwrap());
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    sender.send(buf).unwrap();
                }
                Err(e) => println!("an error!: {:?}", e),
            }
        }
    });
    // format!("{}", process);
    process
}

#[tauri::command]
fn stop_testnet(process:u32) -> u32 {
    Command::new("kill")
    .arg(process.to_string())
    .output()
    .expect("kill failed");
    0
}

#[tauri::command] 
async fn start_testnet(
    window: Window, 
    args: Vec<String>
    //Need to desiralize args
) -> u32 {
    let mut child = Command::new("anvil")
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("anvil failed to run");
    let process: u32 = child.id();
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout).lines();

    thread::spawn(move || {
        for line in reader {
            if let Ok(line) = line {
                println!("{}", line);
                window.emit("anvil-output", AppState {anvil_output: line.into()}).unwrap();
            }
        }
    });
    process
}


#[tauri::command]
fn count(total:i32, diff:i32) -> i32 {
    total + diff
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
            .add_item(CustomMenuItem::new(
                "visibility-toggle".to_string(),
                "Hide"))
            .add_item(CustomMenuItem::new(
                "ls".to_string(),
                "Ls"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "quit".to_string(),
                "Quit"
            ));
    let tray = SystemTray::new().with_menu(tray_menu);
    
/*     let mut builder = tauri::Builder::default()
        .setup(|app| {
            // https://tauri.app/v1/guides/features/events
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();

            // listen to the `event-name` (emitted on the `main` window)
            let id = main_window.listen("anvil-output", |event| {
                println!("got window event-name with payload {:?}", event.payload());
            });
            main_window.unlisten(id);

            // emit the `event-name` event to the `main` window
            // main_window.emit("anvil-output", AppState {anvil_output: ???.into()}).unwrap();
            Ok(())
        });/*  */
        .manage(AppState {anvil_output: Default::default()})
        .invoke_handler(tauri::generate_handler![stop_testnet, start_testnet, count])
        .expect("error while running tauri application"); */


    let app = tauri::Builder::default()
    .setup(|app| {
        let id = app.listen_global("event-name", |event| {
            println!("got event-name with payload {:?}", event.payload());
          });
          // unlisten to the event using the `id` returned on the `listen_global` function
          // an `once_global` API is also exposed on the `App` struct
          app.unlisten(id);
    
          // emit the `event-name` event to all webview windows on the frontend
          app.emit_all("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
        Ok(()) 
    })
    .system_tray(tray)
    .on_system_tray_event(on_tray_event)
    .manage(AppState {anvil_output: Default::default()})
    .invoke_handler(tauri::generate_handler![stop_testnet, start_testnet, count])      
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });        
        
}

fn on_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window =
                        app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(true) => {
                          window.hide().unwrap();
                          item_handle.set_title("Show").unwrap();
                        },
                        Ok(false) => {
                          window.show().expect("msg");
                          item_handle.set_title("Hide").unwrap();

                        },
                        Err(e) => unimplemented!("what kind of errors happen here?"), //TODO: Find errors??
                    }
                }
                "quit" => {
                    Command::new("pkill").arg("anvil");
                    // app.exit(0)
                },
                "ls" => {
                    Command::new("ls").arg("-l");
                    // app.exit(0)
                }
                _ => {}
            }
        }
        _ => {}
    }
}
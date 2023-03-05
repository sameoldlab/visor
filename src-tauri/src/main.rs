// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    io::{BufRead, BufReader}, 
    process::{Command, Stdio},
    sync::mpsc::{channel, Sender},
    thread
};
use tauri::{
    Manager,
    Window,
    RunEvent,
    WindowEvent
  };

#[derive(Clone, serde::Serialize)]
struct AppState {
    anvil_output: String,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

async fn start_anvil(sender: Sender<String>) -> u32 {
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

fn main() {
    let mut builder = tauri::Builder::default()
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
        });/* 
        .manage(AppState {anvil_output: Default::default()})
        .invoke_handler(tauri::generate_handler![stop_testnet, start_testnet, count])
        .expect("error while running tauri application"); */

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
    .manage(AppState {anvil_output: Default::default()})
    .invoke_handler(tauri::generate_handler![stop_testnet, start_testnet, count])      
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        RunEvent::WindowEvent {
            event: WindowEvent::CloseRequested { api, .. },
            ..
          } => {
        // Your cleanup code goes here
        Command::new("pkill")
        .arg("anvil");
    }
    _ => {}
    });        
        
}

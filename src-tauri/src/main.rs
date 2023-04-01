// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::{BufRead, BufReader}, 
    process::{Command, Stdio},
    thread
};
/* use tauri::{
    api::process::{Command, CommandChild, Output},
    AppHandle, Manager 
  };
 */

#[tauri::command]
fn stop_testnet(process: u32) {
    Command::new("kill")
    .arg(process.to_string());
}

#[tauri::command] 
async fn start_testnet(
    args: Vec<String>
) {
    //use tauri::api::process::Command if you come back to this https://discord.com/channels/616186924390023171/1059730246573768704/1059853998062043136
    let mut child = Command::new("anvil")
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("anvil failed to run");
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout).lines();

    thread::spawn(move || {
        for line in reader {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    });
}

fn main() {
/*     let tray_menu = SystemTrayMenu::new()
            .add_item(CustomMenuItem::new(
                "visibility-toggle".to_string(),
                "Hide"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "quit".to_string(),
                "Quit"
            ));
    let _tray = SystemTray::new().with_menu(tray_menu); */
    
/*      .setup(|app| {
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
        });
*/
    
    
    let app = tauri::Builder::default()
/*     .setup(|app| {
        https://tauri.app/v1/guides/features/events
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
    .manage(AppState {anvil_output: Default::default()}) */
    .invoke_handler(tauri::generate_handler![stop_testnet, start_testnet])      
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            // api.prevent_exit();
            // Command::new("mkdir").arg("Iamaclosingprocessfolder");
            // exit(0)
        }
        _ => {}
    });        
        
}

/*
 *  Limbo till I figure out where to listen for window close outside before clicking on a menuitem
 */
/* fn _on_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
) {
    match event { // Does not work on linux
        SystemTrayEvent::RightClick { .. } => {
            println!("Received a Left Click");
            app.get_window("main")
               .unwrap()
               .show()
               .unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            println!("Received a MenuItem Click");
            
            let item_handle = app.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window =
                        app.get_window("main").unwrap();
                        // window.hide().unwrap();
                        // Show only would work...
                    match window.is_visible() {
                        Ok(true) => {
                          window.hide().unwrap();
                          item_handle.set_title("Show").unwrap();
                        },
                        Ok(false) => {
                          window.show().unwrap();
                          item_handle.set_title("Hide").unwrap();

                        },
                        Err(..) => {
                            // unimplemented!("what kind of errors happen here?")
                        } //TODO: Find errors?? App crashes if hidden from somewhere else
                    }
                } 
                "quit" => {
                    app.exit(0)
                },
                _ => {}
            }
        }
        _ => {}
    }
} */
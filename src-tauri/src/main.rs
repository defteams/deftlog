// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct LogPayload(String);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_tcp_server(window: Window) {
  std::thread::spawn(move || {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind to address");

    println!("Listening on 0.0.0.0:8888...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
                let window_clone = window.clone(); // Clone the window
                handle_client(stream, window_clone);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
  });
}

fn handle_client(mut stream: TcpStream, window: Window) {
    let mut buffer = [0; 4096];
    let mut log_string = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 { // End of stream
                    break;
                }

                let received_data = String::from_utf8_lossy(&buffer[..n]);
                log_string.push_str(&received_data);
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }

    // Process the log_string as desired
    println!("Received log entry");

    // let response = "Log entry received\n";
    // stream.write_all(response.as_bytes()).unwrap();
    window.emit("log_entry", LogPayload(log_string)).unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();

            // listen to the `event-name` (emitted on the `main` window)
            // let _id = main_window.listen("event-name", |event| {
            //     println!("got window event-name with payload {:?}", event.payload());
            // });
            // unlisten to the event using the `id` returned on the `listen` function
            // an `once` API is also exposed on the `Window` struct
            // main_window.unlisten(id);

            // emit the `event-name` event to the `main` window
            // main_window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();

            init_tcp_server(main_window);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

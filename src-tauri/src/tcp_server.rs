use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
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
    println!("Received log entry: {}", log_string);

    let response = "Log entry received\n";
    stream.write_all(response.as_bytes()).unwrap();
}

pub fn run(mut app) {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind to address");

    println!("Listening on 0.0.0.0:8888...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
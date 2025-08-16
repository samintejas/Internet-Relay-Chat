use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() -> io::Result<()> {
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let listener =
        TcpListener::bind(format!("{}:{}", HOST, PORT)).expect("Failed to bind to address");
    println!("Server listening on {}:{}", HOST, PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "New client connected: {}",
                    stream.peer_addr().expect("Connected client has no IP?")
                );

                let clients_clone = Arc::clone(&clients);

                let read_stream = stream
                    .try_clone()
                    .expect("Failed to clone stream for reading");
                let read_addr = read_stream.peer_addr().unwrap();

                clients_clone.lock().expect("Mutex poisoned").push(stream);

                thread::spawn(move || {
                    let reader = BufReader::new(read_stream);

                    for line in reader.lines() {
                        match line {
                            Ok(msg) => {
                                println!("Received: {}", msg);
                                broadcast(&clients_clone, &msg);
                            }
                            Err(_) => {
                                println!("Client disconnected");
                                break;
                            }
                        }
                    }

                    {
                        let mut clients_guard = clients_clone.lock().unwrap();
                        clients_guard.retain(|c| c.peer_addr().unwrap() != read_addr);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn broadcast(clients: &Arc<Mutex<Vec<TcpStream>>>, message: &str) {
    let clients_guard = clients.lock().unwrap();
    for client in clients_guard.iter() {
        let mut client = client.try_clone().unwrap();
        if let Err(e) = writeln!(client, "{}", message) {
            eprintln!("Failed to send message: {}", e);
        }
    }
}

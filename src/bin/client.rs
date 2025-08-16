use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 7878;
fn main() -> io::Result<()> {
    let stream = TcpStream::connect(format!("{}:{}", HOST, PORT))?;
    println!("Connected to chat server at {}:{}", HOST, PORT);

    let read_stream = stream.try_clone().expect("Failed to clone stream");
    let write_stream = stream;

    thread::spawn(move || {
        let reader = BufReader::new(read_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("[server] {}", msg),
                Err(_) => {
                    println!("Connection closed by server");
                    break;
                }
            }
        }
    });

    let stdin = io::stdin();
    let mut handle = write_stream;
    for line in stdin.lock().lines() {
        let msg = line?;
        if msg.trim().is_empty() {
            continue;
        }
        if let Err(e) = writeln!(handle, "{}", msg) {
            eprintln!("Failed to send message : {}", e);
            break;
        }
    }
    Ok(())
}

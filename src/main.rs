use std::net::TcpListener;
use std::thread;

pub mod src;
fn main() {
    thread::spawn(|| src::delayed_sending());
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:5701").expect("Failed to bind to port 5701");
    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => src::handle_connection(stream),
            Err(e) => println!("connect failed: {}", e),
        });
    }
}

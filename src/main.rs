extern crate mio;
extern crate cipher;
use std::net::{TcpListener, TcpStream};
use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    fn print_stream(stream: TcpStream) {
        println!("check this {:?}", stream);
    }

    println!("lister {:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Spawning thread...");
                thread::spawn(move || {
                    print_stream(stream);
                });
            }
            Err(e) => { panic!("some error {:?}", e); }
        }
    }
}

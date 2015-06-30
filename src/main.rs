extern crate mio;
extern crate cipher;
use std::net::{TcpListener, TcpStream};
use std::thread;
use cipher::*;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    fn print_stream(stream: TcpStream) {
        println!("check this {:?}", stream);
        let plaintext = "I want this to be a dynamic message :(";
        let ciphertext = feistel_encrypt(plaintext, 141423, 7);
        println!("finsihed encryption {:?}", ciphertext);
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

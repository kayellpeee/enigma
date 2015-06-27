extern crate mio;
extern crate cipher;
use mio::*;
use cipher::*;
fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let sender = event_loop.channel();
    struct GhostStruct;
    impl Handler for GhostStruct {
        type Timeout = ();
        type Message = &'static str;

        fn notify(&mut self,
                   event_loop: &mut EventLoop<GhostStruct>,
                   plaintext: &str) {
            println!("Received Message!\n{:?}", plaintext);
            let ciphertext = feistel_encrypt(plaintext, 1485832, 33);
            println!("Encrypted Message!\n{:?}", ciphertext);
            event_loop.shutdown();
        }
    }

    // here we feed messages into the enigma machine
    let response = sender.send("Oh wow this is transcendent");

    if response.is_err() {
        panic!("EEK, error!\n\t{:?}", response);
    } else {
        println!("yo\n\t{:?}", response);
    }

    event_loop.run(&mut GhostStruct).unwrap();
}

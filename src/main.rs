extern crate mio;
extern crate cipher;
use mio::*;
use mio::tcp::TcpListener;
use cipher::*;
fn main() {
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    let address = "127.0.0.1:3434".parse().unwrap();
    let server = tcp::listen(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server, SERVER).unwrap();

    let (sock, _) = tcp::connect(&address).unwrap();
    event_loop.register(&sock, CLIENT).unwrap();

    struct GhostStruct(NonBlock<TcpListener>);
    impl Handler for GhostStruct {
        type Timeout = ();
        type Message = ();

        fn readable(&mut self,
                    event_loop: &mut EventLoop<GhostStruct>,
                    token: Token,
                    read: ReadHint) {
            match token {
                SERVER => {
                    println!("Matched server, readhint: {:?}", read);
                    let GhostStruct(ref mut server) = *self;
                    let _ = server.accept();
                }
                CLIENT => {
                    println!("Matched client, readhint: {:?}", read);
                    event_loop.shutdown();
                }
                _ => panic!("Umatched Token! readhint: {:?}", read)
            }
        }
    }

    event_loop.run(&mut GhostStruct(server)).unwrap();
}

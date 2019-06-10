extern crate tokio;
extern crate futures;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::{TcpListener,TcpStream};


use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("provide only one port number as argument");
        std::process::exit(1);
    }
	let mut  s= &args[1];
    let addr = s.parse().unwrap();
	//let stream = TcpStream::connect(&addr);
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");
	 println!("Listening on: {}", addr);
	//println!("Incoming address: {}",stream::peer_addr(&addr));
    let server = listener.incoming().map_err(|e| eprintln!("accept failed = {:?}", e)).for_each(|sock| {
            let (reader, writer) = sock.split();
            let bytes_copied = copy(reader, writer);
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("IO error {:?}", err)
            });
            tokio::spawn(handle_conn)
        });
    tokio::run(server);
}

extern crate tokio;

use tokio::io;
use tokio::net::TcpListner;
use tokio::prelude::*;

fn main() 
{
    let listen = "127.0.0.1:3000";
    let addr = listen.parse().unwrap();
    let mut listener = TcpListener::bind(&addr).expect("Could not bind");

    let server = listener.incoming().for_each(|socket| {
        let (reader,writer) = socket.split();

        let No_bytes = io::copy(reader,writer);

        let msg = No_bytes.then(|result|{
            match result{
                Ok((No_bytes,_,_)) => println!("No of bytes:{}",No_bytes),
                Err(e)             => println!("{}",e),
            }

            Ok(())
    });

    tokio::spawn(msg);
    Ok(())
    })
    .map_err(|err| {
        println!("accept error = {:?}", err);
    });

    println!("Server listening on : {}",addr);

    tokio::run(server);
    
}

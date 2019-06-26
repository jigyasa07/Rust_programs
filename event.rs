extern crate mio;
use std::io::prelude::*;
use std::thread;
use mio::*;
use std::net::SocketAddr;
use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Ready, PollOpt, Token};
use std::env;
use std::io;
use std::str;
use std::io::{Read,Write,Error};

fn process_events(e:Event,listener:&TcpListener,poll:&Poll) {
    if (e.token()==Token(0) && e.readiness().is_readable())
    {
        match listener.accept(){
        Ok((mut stream,_)) =>{

            poll.register(&stream,e.token(),Ready::readable(),PollOpt::edge()).unwrap();
        
            println!("Got a client: {:?}",stream.peer_addr().unwrap());

            let mut buffer = [0;512];
           // loop{
                let reader = stream.read(& mut buffer);
                let bytes_no = reader.unwrap()-1;
                //stream.write_all(&buffer[..bytes_no]);
                let s = str::from_utf8(&buffer[..bytes_no]);
                println!("{:?}",s.unwrap());
                println!("No of bytes read : {:?}",bytes_no);
       //     }
       
        }
        Err(e) => panic!("Error"),
        }

    }

}

fn main()
{
    const server: Token = Token(0);
    const client: Token = Token(1); 

    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        eprintln!("Provide the argument");
        std::process::exit(1);
    }
    let listen = &args[1];
    let addr = listen.parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("Could not bind");

    println!("Server listening on : {}",addr);

    let mut eve = Events::with_capacity(1024);

    let poll = Poll::new().unwrap();

    poll.register(&listener,server,Ready::readable(),PollOpt::edge()).unwrap();

    loop{

        poll.poll(&mut eve, None);

        for e in &eve
            {
                process_events(e,&listener,&poll);
            }

        }
}
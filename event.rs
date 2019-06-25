use std::io::prelude::*;
use std::thread;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Ready, PollOpt, Token};
use std::env;
use std::io;
use std::str;
use std::io::{Read,Write,Error};

fn process_events(let e:&Events){

}

fn main()
{
    const server: Token(0);

    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        eprintln!("Provide the argument");
        std::process::exit(1);
    }
    let listen = &args[1];
    let addr:String= listen.parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("Could not bind");
    let eve = Events::with_capacity(1024);
    let poll = Poll::new()?;

    poll.register(&listener,Token(0),Ready::readable()|Ready::writable(),PollOpt::edge())?;

    loop{
        poll.poll(&mut eve, None)?;

        for e in &eve{
            process_events(e);
        }
    }
}
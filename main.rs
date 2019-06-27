extern crate mio;
extern crate fnv;
use std::io::prelude::*;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Ready, PollOpt, Token};
use std::env;
use std::io;
use std::str;
use std::collections::HashMap;
use std::io::{Read,Write,Error};


fn process_events(e:Event,listener:& TcpListener,poll:&Poll, clients:&mut HashMap<Token, TcpStream>,mut count: usize) {


    match e.token(){
    server => {
        match listener.accept(){
        Ok((mut stream,addr)) =>{
            
            println!("Got a client: {:?}",stream.peer_addr().unwrap());  

            count+=1;

            println!("{}",count);

            let new_token = Token(count);
            
            print!(" New token {:?}",new_token);

            print!("2");
        
            poll.register(&stream,new_token,Ready::readable(),PollOpt::edge()).unwrap();

            print!("registration done!!");

            clients.insert(new_token,stream);

        }
        Err(e) => panic!("Error during connection"),
        }
        
    }

    new_token =>
    {
        let mut buf =[0;512];
      //  loop{
            let reader = clients.get_mut(&e.token()).unwrap().read(&mut buf);
            match reader{
                Ok((_)) => {
                    let bytes_no = reader.unwrap()-1;
                    let s = str::from_utf8(&buf[..bytes_no]);
                    println!("{:?}",s.unwrap());
                    println!("No of bytes read : {:?}",bytes_no);
                }
                Err(e)=>{
                    println!("could not read");
                }
            }
       // }
    }

    }
    
}

fn main()
{
    const server: Token = Token(0);
   

    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        eprintln!("Provide the argument");
        std::process::exit(1);
    }
    let listen = &args[1];
    let addr = listen.parse().unwrap();
    let mut listener = TcpListener::bind(&addr).expect("Could not bind");

    println!("Server listening on : {}",addr);

    let mut clients = HashMap::new();

    let mut eve = Events::with_capacity(1024);

    let poll = Poll::new().unwrap();

    let mut count =0;

    poll.register(&listener,server,Ready::readable(),PollOpt::edge()).unwrap();

    loop{

        poll.poll(&mut eve, None);

        for e in &eve
            {
                process_events(e,& listener,&poll,&mut clients,count);
            }
            
        }
}
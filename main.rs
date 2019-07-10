extern crate mio;
extern crate bincode;
use std::io::prelude::*;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Ready, PollOpt, Token};
use std::env;
use std::io;
use std::str;
use std::iter;
use std::iter::repeat;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Read,Write,Error};
use bincode::{serialize, deserialize};

pub struct WorkerBuffer{
    pub buf : Vec<u8>,
    pub expected_size : usize,
    pub currently_read : usize,
}

impl WorkerBuffer 
{
    pub fn new() -> WorkerBuffer
    {
        WorkerBuffer { buf: Vec::new(), expected_size: 8, currently_read: 0}
    }
}


fn process_events(e:Event,listener:& TcpListener,poll:&Poll, clients:&mut HashMap<Token, TcpStream>,mut count: usize) {

    //println!("{:?}",e.token());

    //match e.token(){

    if (e.token()==Token(0) ) {
        
        match listener.accept(){
        Ok((mut stream,addr)) =>{

            let new_token = Token(count);
            println!("new token {:?}",new_token);
            
            poll.register(&stream,new_token,Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap();
            
            println!("Got a client: {:?}",stream.peer_addr().unwrap());  

            clients.insert(new_token,stream);

        }
        Err(e) => panic!("Error during connection{}",e),

        }
        
    }

  if(e.token()!=Token(0) && e.readiness().is_readable()) {

        //let mut buf =[0;512];
        let mut worker = usize::try_from(e.token()).unwrap();
        let mut client_len = clients.len() + 1;
        // println!("worker {:?} , client_len {:?}",worker,clients.len());
        // worker = worker-1;
        println!("worker {:?} , client_len {:?}",worker,client_len);
        let mut worker_buffers: Vec<_> = iter::repeat_with({|| WorkerBuffer::new()}).take(client_len).collect();
        let mut buf = &mut worker_buffers[worker];
        let mut t = e.token();
        //let reader = clients.get_mut(&e.token()).unwrap().read(&mut buf);

            // match reader{
            //     Ok((_)) => {
            //         let size = clients.keys().len();
            //        // if (size==800){
            //        // let bytes_no = reader.unwrap();
            //         //println!("No of bytes read : {:?}, {:?}",bytes_no,e.token());
            //         //println!("Client no {:?}",e.token());
            //       //  }
            //     }
            //     Err(e)=>{
            //         println!("could not read: {}",e);
            //     }

            // }   

        if buf.currently_read <8 
        {
            buf.buf.extend(iter::repeat(0u8).take(8));
            match clients.get_mut(&e.token()).unwrap().read(&mut buf.buf[buf.currently_read..]){
                Ok(n) => buf.currently_read +=n,
                Err(e) => println!("Error reading data from clients {}",e),
            }

            if buf.currently_read == buf.expected_size{
                buf.expected_size = deserialize(&buf.buf[..]).unwrap();
                println!(" buf.expected_size {:?}", buf.expected_size);
                buf.buf.extend(iter::repeat(0u8).take(buf.expected_size - 8));
            }
        }

        if buf.currently_read >=8
        {
            match clients.get_mut(&e.token()).unwrap().read(&mut buf.buf[buf.currently_read..]){
                Ok(n) => buf.currently_read +=n,
                Err(e) => println!("Error reading data from client {}",e),
            }
        }
        //buf.currently_read == buf.expected_size



        poll.reregister(&clients[&e.token()],e.token(),Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap();

    
    }

    
  //  }
    
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

    let mut count = 1;

    poll.register(&listener,server,Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap();

    loop{

        poll.poll(&mut eve, None);

        for e in &eve
            {
                process_events(e,& listener,&poll,&mut clients,count);
                if(e.token()==Token(0))
                {
                count+=1;
                }
                
            }
        poll.reregister(&listener,server,Ready::readable(),PollOpt::edge()|PollOpt::oneshot()).unwrap(); 
        }
     
}
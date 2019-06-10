//#![feature(ip)]
use std::net::{IpAddr, SocketAddr};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::env;
use std::io::{Read,Write,Error};
struct TCPServer {
    address: SocketAddr,
}

impl TCPServer {
    fn new(port: u32) -> Self {
        let address = format!("0.0.0.0:{}", port)
            .parse::<SocketAddr>().unwrap();

        TCPServer {
            address,
        }
    }
		
		fn run(&mut self)
		{	
			let listener = TcpListener::bind(&self.address).expect("Could not bind to port");
			for stream in listener.incoming()
			{
				match stream
				{
					Err(e)=>{eprintln!("failed: {}",e)}
					Ok(stream)=>
					{
						thread::spawn(move||{handle_client(stream).unwrap_or_else(|error|eprintln!("{:?}",error));
						});
					}		
				}
			}
		}
}
fn handle_client(mut stream:TcpStream)->Result<(),Error>
{
	println!("Incoming address: {}",stream.peer_addr()?);
	let mut buf =[0,512];
	loop
	{
		let bytes_read = stream.read(&mut buf)?;
		if bytes_read ==0
    {
      return Ok(());
    }
		stream.write(&buf[..bytes_read])?;
	}
}

fn main()
{
		let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide only one port number as argument");
        std::process::exit(1);
    }
    let mut server = TCPServer::new(args[1].parse::<u32>().expect("Could not parse as u32"));
    server.run();
}

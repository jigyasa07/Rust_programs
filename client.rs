use std::net::{TcpStream};
use std::thread;
use std::io;
use std::io::{Write};

fn handle_client(c:&mut i64){
    let server_addr = String::from("localhost:8080");
    if let Ok(mut stream) = TcpStream::connect(server_addr)
    {
        println!("Connected to server {} ",c);
        let msg = [0;512];
        loop{
            
            stream.write(&msg).unwrap();
        }
    }
    else{
        println!("could not connect");
    }
} 


fn main(){
    let mut c=0;
    for _i in 0..500{
        thread::spawn(move|| handle_client(&mut c));
        c=c+1;
    }

    let mut exit = String::new();
    io::stdin().read_line(&mut exit).expect("could not exit");
}
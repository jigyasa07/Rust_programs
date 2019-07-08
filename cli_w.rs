use std::net::{TcpStream};
use std::thread;
use std::io;
use std::io::{Write};

fn handle_client(c:&mut i64,flag:&mut bool){
    let server_addr = String::from("localhost:3000");
    if let Ok(mut stream) = TcpStream::connect(server_addr)
    {
        println!("Connected to server {} ",c);
        let msg = [0;512];
        let ans = b"start";

        // match stream.read_exact(&mut msg)
        // {
        //     Ok(_) => 
        //     {
       
        //         if &msg==ans{

        //             println!("sending data");   

                    loop{
                        
                        stream.write(&msg).unwrap();
                    }
        //         }
        //     }  
        // }
    }
    else{
        println!("could not connect");
    }
} 


fn main(){
    let mut c=0;
    let mut flag = false;
    for _i in 0..1020{
        thread::spawn(move|| handle_client(&mut c,&mut flag));
        c=c+1;
        // if (_i==4){
        //   flag = true;  
        // }
    }

    let mut exit = String::new();
    io::stdin().read_line(&mut exit).expect("could not exit");
}
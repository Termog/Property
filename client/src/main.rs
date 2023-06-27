use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    echo()
}

//simple client echo function
fn echo() {
    match TcpStream::connect("localhost:3000") {
        Ok(mut stream) => {
            println!("Successfully connected to 3000");

            let msg = b"HI I AM CLIENT";

            stream.write(msg).unwrap();
            println!("Sent message, waiting for a reply");


            let mut data = [0 as u8; 14];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok");
                    } else  {
                        let text = from_utf8(&data).unwrap();
                        println!("unxepected: {}",text);
                    }
                },
                Err(e) => {
                    println!("failed to recieve data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

pub mod board;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let (first,second) = board::roll_dice(&mut rng);
    println!("You've rolled: [{first}],[{second}]");
}


//a simple echo function to test networking
fn echo() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("server listening on 0.0.0.0:3000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("Error, diconnecting {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

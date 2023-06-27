pub mod board;
//use std::net::{TcpListener, TcpStream, Shutdown};
use std::net::{TcpListener, TcpStream};
//use std::io::{Read, Write};
use bincode;
use rand::prelude::*;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("server listening on 0.0.0.0:3000");

    //creates an object representing the game
    let mut game = board::Game::create(2);

    //handels all listener conections
    //maybe should move this to a separate thread that gets killed when the game starts
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                //add error handeling and connect_player should also probably return an error that
                //should be handeled

                //adds player
                //should add more error handeling
                match game.add(connect_player(stream)) {
                    Ok(_) => (),
                    Err(_) => panic!(),
                }
            }
            Err(e) => {
                println!("Error:{}", e);
            }
        }
        //simple condition to start the game
        //should add host player and make him start the game
        if game.get_player_number() == game.get_player_max() {
            break;
        }
    }
    //at this point all players should be connected and the game starts
    println!("Game is starting");

    for player_id in 0..game.get_player_number() {
        //encapsulate message
        let message = api::ServerMessage::GameStarting;
        //extract players stream
        let mut stream = &mut game.get_player_mut(player_id).stream;
        //serialize right into the stream
        //TODO error handeling
        bincode::serialize_into(&mut stream, &message).unwrap();
    }
    //should send out game starting messages to all players
    //drops tcp listener
    drop(listener);

    game.start().unwrap();
    //send initial board update message to all players
    game.send_board_updates();
    //this should be the game loop
    loop {
        game.turn()
    }
}

//function that handels an incoming tcp connection and if the connect message is sent adds the
//returns a player object
fn connect_player(mut stream: TcpStream) -> board::Player {
    //deserializes data in tcp stream into a client message object
    let message: api::ClientMessage = match bincode::deserialize_from(&mut stream) {
        Ok(message) => message,
        //incorrect messages should just be ignored
        Err(_) => panic!(),
    };
    //this matches on the spesified message
    let player_info = match message {
        api::ClientMessage::Connect(player_info) => player_info,
        //add error handeling by sending error on incorrect connection to stream and closing it
        _ => panic!(),
    };
    println!("Player {} connected", player_info.name);
    board::Player::create(&player_info.name, player_info.icon, stream)
}

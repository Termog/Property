//use std::io::{Read, Write};
use std::io::Read;
use std::net::TcpStream;
use std::{io, thread, time::Duration};
//api for client server interactions
use api;
//crate for terminal ui
use tui::{
    backend::CrosstermBackend,
    //layout::{Constraint, Direction, Layout, Rect},
    layout::Rect,
    terminal::Frame,
    //widgets::{Block, Borders, Widget},
    widgets::{Block, Borders},
    Terminal,
};
//backend for tui
use crossterm::{
    //event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
//local module with game objects
pub mod board;

fn main() -> Result<(), io::Error> {
    //should probably move this to a connection function
    //and should make a object for the game interactions
    let stream = match TcpStream::connect("localhost:3000") {
        Ok(mut stream) => {
            let mut username = String::new();
            let icon: char;
            println!("Successfully connected to 3000");
            println!("Input your username");
            //create a structure with player info that will be sent to the server
            std::io::stdin().read_line(&mut username).unwrap();
            println!("Input your charachter");
            //this is bullshit find a better way
            icon = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as char)
                .unwrap();
            let connection_message = api::ConnectionInfo {
                name: username,
                icon,
            };
            //encapsulate message in a ClientMessage enum
            let message = api::ClientMessage::Connect(connection_message);
            //serialize it strait into the tcp stream
            bincode::serialize_into(&mut stream, &message).unwrap();
            stream
        }
        //should write error handeling some day
        Err(_) => {
            panic!();
        }
    };
    //wait for starting message before initializing the playing field
    let mut starting = false;
    while starting != true {
        starting = match bincode::deserialize_from(&stream) {
            Ok(api::ServerMessage::GameStarting) => true,
            _ => false,
        };
    }

    //code to redner alternate terminal and render the game ui
    //needs a lot of work
    //should probably move this to a separate function
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //wait for a message from server and deserialize it on recieving
    let message: api::ServerMessage = match bincode::deserialize_from(&stream) {
        Ok(message) => message,
        Err(_) => panic!(),
    };
    //parse the message and convert into Game object
    let mut gamestate: board::Game = match message {
        api::ServerMessage::Update(gamestate) => gamestate.into(),
        _ => panic!(),
    };

    //draw the updated field
    terminal.draw(|f| render_field(f, &gamestate.player, &gamestate.players))?;

    //there should be infinite loop that handles incoming messages and player inputs
    //placeholder testing shit
    thread::sleep(Duration::from_millis(2500));
    let mut i = 0;
    while i < 2 {
        let message: api::ServerMessage = match bincode::deserialize_from(&stream) {
            Ok(message) => message,
            Err(_) => panic!(),
        };

        //TODO? maybe move each match to a separate function
        match message {
            api::ServerMessage::Update(boardstate) => {
                //TODO Extremely bad, just copy pasted should move this shit out to a function
                //maybe could do conversion inline on match statement
                //updating gamestate
                gamestate = boardstate.into();
                terminal.draw(|f| render_field(f, &gamestate.player, &gamestate.players))?;
                thread::sleep(Duration::from_millis(2500));
                i += 1;
            }
            api::ServerMessage::YourTurn(dice1, dice2) => {
                //TODO pretty dice rendering widget
                //draws dice, however it overrides the whole board
                //TODO resolve this problem
                //maybe make dice a widget
                terminal.draw(|f| {
                    render_field(f, &gamestate.player, &gamestate.players);
                    board::render_dice(f.size(), f, dice1, dice2);
                })?;
                thread::sleep(Duration::from_millis(1500));
                let message = api::ClientMessage::RolledDice;
                bincode::serialize_into(&stream, &message).unwrap();
            }
            _ => panic!(),
        }
    }

    //returns to normal terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

//function that renders the playing field that will be static
//need a lot of work
//TODO add some minimal display size and if the display is smaller show message to resize the
//display or make the font smaller
fn render_field<B, T, K>(f: &mut Frame<B>, player: &T, players: &Vec<K>)
where
    //generic bounds on argumetns, maybe they are unneseccary
    B: tui::backend::Backend,
    T: board::Player,
    K: board::Player,
{
    //rendering the squares
    let (width, height) = board::get_fieldblock_size(f.size());
    for i in 0..11 {
        let block_size = Rect::new(i * width, 0, width, height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
        let block_size = Rect::new(i * width, 10 * height, width, height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
    }
    for j in 1..10 {
        let block_size = Rect::new(0, j * height, width, height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
        let block_size = Rect::new(10 * width, j * height, width, height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
    }
    //render main player
    f.render_widget(player.get_widget(), f.size());
    //render other players
    for player in players {
        f.render_widget(player.get_widget(), f.size());
    }
}

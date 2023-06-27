use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, thread, time::Duration};
//api for client server interactions
use api;
//crate for terminal ui
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Frame,
    widgets::{Block, Borders, Widget},
    Terminal,
};
//backend for tui
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
//local module with game objects
pub mod board;

fn main() -> Result<(), io::Error> {
    //should probably move this to a connection function
    //and should make a object for the game interactions
    /*
        match TcpStream::connect("localhost:3000") {
            Ok(stream) => {
                let mut username = String::new();
                println!("Successfully connected to 3000");
                print!("Input you username");
                //create a structure with player info that will be sent to the server
                std::io::stdin().read_line(&mut username).unwrap();
                let connection_message = api::ConnectionInfo {
                    name: username,
                };
                //encapsulate message in a ClientMessage enum
                let message = api::ClientMessage::Connect(connection_message);
                //serialize it strait into the tcp stream
                bincode::serialize_into(stream,&message).unwrap();
            },
            //should write error handeling some day
            Err(_) => {
                panic!();
            }
        }
    */
    //code to redner alternate terminal and render the game ui
    //needs a lot of work
    //should probably move this to a separate function
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut player = board::PlayerMain::create("termog");
    for i in 0..40 {
        player.update(i);
        terminal.draw(|f| render_field(f, &player))?;
        thread::sleep(Duration::from_millis(500));
    }

    thread::sleep(Duration::from_millis(5000));

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
fn render_field<B>(f: &mut Frame<B>, player: &board::PlayerMain)
where
    B: tui::backend::Backend,
{
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
    f.render_widget(player.get_widget(), f.size());
}

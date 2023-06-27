use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::{io, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget},
    terminal::Frame,
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        render_field(f);
    })?;

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

fn render_field<B> (f: &mut Frame<B>)
where 
    B: tui::backend::Backend,
{
    let size = f.size();
    let width = size.width/11;
    let height = size.height/11;
    for i in 0..11 {
        let block_size = Rect::new(i*width,0,width,height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
        let block_size = Rect::new(i*width,10*height,width,height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
    }
    for j in 1..10 {
        let block_size = Rect::new(0,j*height,width,height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
        let block_size = Rect::new(10*width,j*height,width,height);
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, block_size);
    }
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
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("unxepected: {}", text);
                    }
                }
                Err(e) => {
                    println!("failed to recieve data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

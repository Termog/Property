use crate::board::{self, render_dice, App, Game, Player};
use api::PlayingField;
use std::{io, sync::mpsc};
use tui::{
    backend::{Backend, CrosstermBackend},
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

fn draw_app<B>(f: &mut Frame<B>, ui: &Ui)
where
    B: Backend,
{
    render_field(f, &ui.game.player, &ui.game.players, &ui.field);
}

//messages sent to ui thread
pub enum RenderMessage {
    Game(Game),
    Dice((u16, u16)),
}

//messages sent from ui thread
pub enum InputMessage {}

struct Ui {
    update: bool,
    game: Game,
    dice: Option<(u16, u16)>,
    field: Vec<PlayingField>,
}

//function called to start ui thread
pub fn ui(
    // terminal: Terminal<B>,
    tx: mpsc::Sender<InputMessage>,
    rx: mpsc::Receiver<RenderMessage>,
    field: Vec<PlayingField>,
    initial_game: Game,
) -> Result<(), io::Error> {
    //prepare  the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ui = Ui {
        update: false,
        game: initial_game,
        dice: None,
        field,
    };
    terminal.draw(|f| draw_app(f, &ui))?;
    'main: loop {
        //try to recieve messages and process them
        match rx.try_recv() {
            Ok(message) => {
                handle_render_message(&mut ui, message);
            }
            Err(e) if e == mpsc::TryRecvError::Empty => (),
            Err(e) if e == mpsc::TryRecvError::Disconnected => break 'main,
            //todo throw error i guess
            Err(_) => panic!(),
        }
        //render shit
        if ui.update {
            terminal.draw(|f| draw_app(f, &ui))?;
        }
        //figure out why are dice overriding the field
        if let Some((dice1, dice2)) = ui.dice {
            terminal.draw(|f| render_dice(f.size(), f, dice1, dice2))?;
            std::thread::sleep(std::time::Duration::from_millis(1500));
            ui.dice = None;
            ui.update = true;
        }
    }
    //go back to normal terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn handle_render_message(ui: &mut Ui, message: RenderMessage) {
    match message {
        RenderMessage::Game(game) => {
            ui.game = game;
            ui.update = true
        }
        RenderMessage::Dice(dice) => ui.dice = Some(dice),
    }
}

fn render_field<B, T, K>(
    f: &mut Frame<B>,
    player: &T,
    players: &Vec<K>,
    field: &[api::PlayingField],
) where
    //generic bounds on argumetns, maybe they are unneseccary
    B: tui::backend::Backend,
    T: Player,
    K: Player,
{
    let frame_size = f.size();
    let x_ofset = frame_size.width - frame_size.height * 2;
    let player_frame = Rect {
        x: x_ofset,
        y: 0,
        width: frame_size.width - x_ofset,
        height: frame_size.height,
    };
    //rendering the squares
    let (width, height) = board::get_fieldblock_size(f.size());
    //maping field ids to a two dimentinal grid of the field
    for (id, field) in field.iter().enumerate() {
        let (x, y) = match id {
            i if i < 11 => (i, 0),
            i if i < 21 => (10, i - 10),
            i if i < 31 => (30 - i, 10),
            i if i < 40 => (0, 40 - i),
            _ => panic!(),
        };
        //creating the field title based on its type
        //TODO add color extraction and maybe some symbols
        let title_upper = match field {
            api::PlayingField::IncomeTax(_) => "Income Tax",
            api::PlayingField::Jail => "Jail",
            api::PlayingField::Chance => "Chance",
            api::PlayingField::Go(_) => "Go",
            api::PlayingField::GoToJail => "Go To Jail",
            api::PlayingField::FreeParking => "Free Parking",
            api::PlayingField::Utility(utility) => &utility.name,
            api::PlayingField::ComunityChest => "Comunity Chest",
            api::PlayingField::Property(porperty) => &porperty.name,
            api::PlayingField::RailRoad(rail) => &rail.name,
            api::PlayingField::LuxuryTax(_) => "Luxury Tax",
        };
        let block_size = Rect::new(x as u16 * width + x_ofset, y as u16 * height, width, height);
        //not shure about the title, only about 4 letters are visible on a 1280x720 screen
        //for now it's a check that everything is rendered properly
        let block = Block::default().title(title_upper).borders(Borders::ALL);
        f.render_widget(block, block_size);
    }
    //render main player
    f.render_widget(player.get_widget(), player_frame);
    //render other players
    for player in players {
        f.render_widget(player.get_widget(), player_frame);
    }
}

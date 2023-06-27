use api;
use rand::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub enum Error {
    TooManyPlayers,
    GameFull,
}

//enum contaning all possible game states
pub enum GameState {
    Preparing,
    PreThrow, //change name later
    CubeThrow,
}

//main game structure
pub struct Game {
    players: Vec<Player>,
    player_count: u16,
    player_max: u16,
    player_turn: u16,
    rng: ThreadRng, //main game thread used for all rng elements
}

//should probably make some functions async
impl Game {
    //function to create initial game
    pub fn create(player_max: u16) -> Self {
        let players: Vec<Player> = Vec::with_capacity(player_max as usize);
        Game {
            players,
            player_count: 0,
            player_max,
            player_turn: 0,
            rng: thread_rng(),
        }
    }
    //function adding players to a game
    pub fn add(&mut self, player: Player) -> Result<(), Error> {
        if self.player_count < self.player_max {
            self.players.push(player);
            self.player_count += 1;
            Ok(())
        } else {
            Err(Error::GameFull)
        }
    }
    //function that is called to start the game
    pub fn start(&mut self) -> Result<(), Error> {
        //maybe render the initial game
        self.players.shuffle(&mut self.rng);
        Ok(())
    }

    //function that sends board updates to each player
    //questening its usefullness
    pub fn send_board_updates(&mut self) {
        for player_id in 0..self.get_player_number() {
            //encapsulate message
            let message = api::ServerMessage::Update(self.boardstate(player_id));
            //extract players stream
            let mut stream = &mut self.get_player_mut(player_id).stream;
            //serialize right into the stream
            //TODO error handeling
            bincode::serialize_into(&mut stream, &message).unwrap();
        }
    }

    pub fn turn(&mut self) {
        //dirty way there should be something more ellegant
        {
            //I think I should be able to iter_mut and map over players and create
            //a Vec<&mut Player>
            //And than I should be able to partialy update other players and the main player
            let player = &mut self.players[self.player_turn as usize];
            let (cube1, cube2) = roll_dice(&mut self.rng);

            let message = api::ServerMessage::YourTurn(cube1, cube2);
            let mut stream = &mut player.stream;

            //TODO error handeling
            bincode::serialize_into(&mut stream, &message).unwrap();

            //TODO error handeling
            match bincode::deserialize_from(&mut stream) {
                Ok(api::ClientMessage::RolledDice) => (),
                _ => panic!(),
            };
            player.move_steps(cube1 + cube2);
        }
        self.send_board_updates();

        //let (cube1, cube2) = roll_dice(&mut self.rng);
        //some render function
        //some render position function
        //I think renderer should be included in the player, the game itself is static

        // player.control(); //should be a communication loop
        //some render and game state change function

        //Create Your turn message
        self.player_turn = (self.player_turn + 1) % self.player_count;
    }
    pub fn get_player_number(&self) -> u16 {
        self.player_count
    }
    pub fn get_player_max(&self) -> u16 {
        self.player_max
    }
    //extracts mutable reference to players vector
    pub fn get_players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }
    //extract mutable reference to a specific player
    pub fn get_player_mut(&mut self, id: u16) -> &mut Player {
        self.players.get_mut(id as usize).unwrap()
    }
    //functions that creates an api::Boardstate from current boardstate
    pub fn boardstate(&self, player_id: u16) -> api::BoardState {
        //this is a mess should make everything better
        let mut players: Vec<api::PlayerOther> = self
            .players
            .iter()
            .map(|i| Into::<api::PlayerOther>::into(i))
            .collect();
        players.swap_remove(player_id as usize);
        api::BoardState {
            //TODO add error handeling
            player: self.players.get(player_id as usize).unwrap().into(),
            // player: players.swap_remove(player_id),
            //need to drop the main player from vector or rewrite everything so client parses main
            //player from this arrya (second is probably better)
            players,
            turn: self.player_turn,
        }
    }
}

//structure representing a player
//maybe should make it a trait
//or make it generic and defind a trait for player controls
pub struct Player {
    position: u16,
    name: String,
    icon: char,
    pub stream: TcpStream,
}

//to easily convert from api network objects to internal server objects
impl From<&Player> for api::PlayerMain {
    fn from(player: &Player) -> Self {
        api::PlayerMain {
            name: player.name.clone(),
            position: player.position,
            icon: player.icon,
        }
    }
}
//to easily convert from api network objects to internal server objects
impl From<&Player> for api::PlayerOther {
    fn from(player: &Player) -> Self {
        api::PlayerOther {
            name: player.name.clone(),
            position: player.position,
            icon: player.icon,
        }
    }
}

impl Player {
    fn move_steps(&mut self, steps: u16) {
        self.position = (self.position + steps) % 40;
    }
    fn move_to(&mut self, field_number: u16) {
        self.position = field_number;
    }
    pub fn create(name: &str, icon: char, stream: TcpStream) -> Self {
        Player {
            position: 0,
            name: name.to_owned(),
            icon,
            stream,
        }
    }
    //function that is called to let player make desisions
    pub fn control(&self) {
        ()
    }
}

//should move it to some object (probably player)
pub fn roll_dice(rng: &mut ThreadRng) -> (u16, u16) {
    return (rng.gen_range(1..=6), rng.gen_range(1..=6));
}

use rand::prelude::*;
use std::net::TcpStream;

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
    player_count: u32,
    player_max: u32,
    player_turn: u32,
    rng: ThreadRng, //main game thread used for all rng elements
}
//should probably make some functions async
impl Game {
    //function to create initial game
    pub fn create(player_max: u32) -> Self {
        let players: Vec<Player> = Vec::with_capacity(player_max as usize);
        Game {
            players,
            player_count: 0,
            player_max,
            player_turn: 0,
            rng : thread_rng(),
        }
    } 
    //function adding players to a game
    pub fn add(&mut self, player: Player) -> Result<(),Error> {
        if self.player_count < self.player_max {
            self.players.push(player);
            self.player_count+=1;
            Ok(())
        } else {
            Err(Error::GameFull)
        }
    }
    //function that is called to start the game
    pub fn start(&mut self) -> Result<(),Error> {
        //maybe render the initial game
        self.players.shuffle(&mut self.rng);
        Ok(())
    }
    pub fn turn(&mut self) {
        let player = &mut self.players[self.player_turn as usize];
        player.control(); //should be a communication loop 
        //some render and game state change function
        let (cube1,cube2) = roll_dice(&mut self.rng);
        //some render function
        player.move_steps(cube1 + cube2);
        //some render position function
        //I think renderer should be included in the player, the game itself is static
        self.player_turn = (self.player_turn + 1)  % self.player_count; 
    }
    pub fn get_player_number(&self) -> u32 {
        self.player_count
    }
    pub fn get_player_max(&self) -> u32 {
        self.player_max
    }
}

//structure representing a player
//maybe should make it a trait
//or make it generic and defind a trait for player controls
pub struct Player {
    position: u32,
    name: String,
    stream: TcpStream,
}

impl Player {
    fn move_steps(&mut self,steps: u32) {
        self.position = (self.position + steps) % 40;
    }
    fn move_to(&mut self,field_number: u32) {
        self.position = field_number;
    }
    pub fn create(name: &str,stream: TcpStream) -> Self {
        Player { 
            position: 0,
            name: name.to_owned(),
            stream: stream,
        }
    }
    //function that is called to let player make desisions
    pub fn control(&self) {
        ()
    }
}

//should move it to some object (probably player)
pub fn roll_dice(rng: &mut ThreadRng) -> (u32,u32) {
    return (
        rng.gen_range(1..=6),
        rng.gen_range(1..=6),
        )
}

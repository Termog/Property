use rand::prelude::*;

pub enum Error{
    TooManyPlayers,
}

pub struct Game {
    players: Vec<Player>,
    player_count: u32,
    player_max: u32,
}

impl Game {
    pub fn create(player_max: u32) -> Self {
        let players: Vec<Player> = Vec::with_capacity(player_max as usize);
        Game {
            players,
            player_count: 0,
            player_max,
        }
    }
}

pub struct Player {
    position: u32,
    name: String,
}

impl Player {
    fn move_steps(&mut self,steps: u32) {
        self.position = self.position + steps % 40;
    }
    fn move_to(&mut self,field_number: u32) {
        self.position = field_number;
    }
    fn create(name: &str) -> Self {
        Player { 
            position: 0,
            name: name.to_owned(),
        }
    }
}

pub fn roll_dice(rng: &mut ThreadRng) -> (u32,u32) {
    return (
        rng.gen_range(1..=6),
        rng.gen_range(1..=6),
        )
}

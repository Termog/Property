use serde::{Deserialize, Serialize};

//should rewrite the whole api to use references because inefficient

//Enum representing messages sent from server to client
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    SendBoard(Vec<PlayingField>),
    Update(BoardState),
    GameStarting,
    YourTurn(u16, u16),
}

//Enum representing messages sent from client to server
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Connect(ConnectionInfo),
    RolledDice,
}

//BoardState object
//idea is to send it every game event to update all players fields
#[derive(Serialize, Deserialize, Debug)]
pub struct BoardState {
    pub player: PlayerMain,
    pub players: Vec<PlayerOther>,
    pub turn: u16,
}

//struct representing oponents for each players
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerOther {
    pub name: String,
    pub position: u16,
    pub icon: char,
    pub money: u64,
}

//struct representing players player data
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMain {
    pub name: String,
    pub position: u16,
    pub icon: char,
    pub money: u64,
}

//information that is sent to connect to a game by a player
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInfo {
    pub name: String,
    pub icon: char,
}

enum ServerMessage {
    Update(BoardState)
}

struct BoardState {
    player: Player_main,
    players: Vec<Player_other>,

}

struct Player_other {
    name: String,
    position: u32,
}

struct Player_main {
    name: String,
    position: u32,
}



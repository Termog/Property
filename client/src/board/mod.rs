use api;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

//trait for rendering player widget
//maybe remove it I should remove it I don't know
pub trait Player {
    //function that returns Player Widget
    fn get_widget(&self) -> PlayerWidget;
}

//struct representing clients player
pub struct PlayerMain {
    name: String,
    position: u16,
    icon: char,
}
//struct representing other player
//maybe should not have separate structs but have one and one PlayerMain object that incapsulates
//it
pub struct PlayerOther {
    name: String,
    position: u16,
    icon: char,
}
//implementing functions for PlayerMain
impl PlayerMain {
    //I think this one is uselsess
    pub fn create(name: &str) -> Self {
        PlayerMain {
            name: name.to_owned(),
            position: 0,
            icon: '@',
        }
    }
    //and maybe this one is also useless
    pub fn update(&mut self, position: u16) {
        self.position = position;
    }
}

//maybe have generic implementation with macro or something like this
impl Player for PlayerMain {
    fn get_widget(&self) -> PlayerWidget {
        PlayerWidget {
            position: self.position,
            icon: self.icon,
        }
    }
}

impl Player for PlayerOther {
    fn get_widget(&self) -> PlayerWidget {
        PlayerWidget {
            position: self.position,
            icon: self.icon,
        }
    }
}

//from conversion to easily convert communication api objects into client objecAts
impl From<api::PlayerMain> for PlayerMain {
    fn from(player: api::PlayerMain) -> Self {
        PlayerMain {
            name: player.name,
            position: player.position,
            icon: player.icon,
        }
    }
}
//same thing here
impl From<api::PlayerOther> for PlayerOther {
    fn from(player: api::PlayerOther) -> Self {
        PlayerOther {
            name: player.name,
            position: player.position,
            icon: player.icon,
        }
    }
}

//function to calculate coordinates on board based on field_number
fn calculate_player_coordinates(field_number: u16) -> (u16, u16) {
    let x;
    let y;
    if field_number <= 10 {
        y = 0;
        x = field_number;
    } else if field_number <= 20 {
        y = field_number - 10;
        x = 10;
    } else if field_number <= 30 {
        y = 10;
        x = 30 - field_number;
    } else {
        y = 40 - field_number;
        x = 0;
    }

    //maybe error handeling;
    (x, y)
}

// struct representing playermain wiget
pub struct PlayerWidget {
    position: u16,
    icon: char,
}

//trait to render player as widget
impl Widget for PlayerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = get_fieldblock_size(area);
        let (x, y) = calculate_player_coordinates(self.position);

        let mut offset = 1;
        //offsets player position until it doesn't overlap another player
        //should make some kind of check if it overlaps the bounderies and maybe zoom in on field
        while buf.get(x * width + offset, y * height + 1).symbol != " " {
            offset += 1;
        }
        buf.get_mut(x * width + offset, y * height + 1)
            //should add symbol to player struct and let player pick it
            .set_symbol(&self.icon.to_string());
    }
}

//function to calculate maximum size of a fild block on a given terminal
pub fn get_fieldblock_size(rect: Rect) -> (u16, u16) {
    let (width, height) = if rect.width < rect.height * 2 {
        let width = rect.width / 11;
        let height = width / 2;
        (width, height)
    } else {
        let height = rect.height / 11;
        let width = height * 2;
        (width, height)
    };
    (width, height)
}

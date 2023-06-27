use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

//struct representing clients player
pub struct PlayerMain {
    name: String,
    position: u16,
}

impl PlayerMain {
    pub fn create(name: &str) -> Self {
        PlayerMain {
            name: name.to_owned(),
            position: 0,
        }
    }
    pub fn update(&mut self, position: u16) {
        self.position = position;
    }
    pub fn get_widget(&self) -> PlayerMainWidget {
        PlayerMainWidget {
            position: self.position,
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
pub struct PlayerMainWidget {
    position: u16,
}

//trait to render player as widget
impl Widget for PlayerMainWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = get_fieldblock_size(area);
        let (x, y) = calculate_player_coordinates(self.position);
        //  buf.get_mut(x*width+width/2,y*height+height/2).set_symbol("@");
        let mut offset = 1;
        while buf.get(x * width + 4, y * height + 1).symbol != " " {
            offset += 1;
        }
        buf.get_mut(x * width + offset, y * height + 1)
            .set_symbol("@");
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

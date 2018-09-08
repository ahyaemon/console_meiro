mod map;
mod player;
mod goal;
pub mod pos;
mod state;
mod row;

use self::map::Map;
use command::Command;
use self::player::Player;
use self::goal::Goal;
use text::Text;
use self::pos::Pos;
use self::state::State;

pub struct Stage {
    map: Map,
    player: Player,
    goal: Goal,
    state: State
}

impl Stage {

    pub fn from_text(text: Text) -> Stage {
        let map = Map::new(text.borrow_str());
        let player = Player::new(text.find_position('P'));
        let goal = Goal::new(text.find_position('G'));
        let state = State::Play;
        Stage{ map, player, goal, state}
    }

    pub fn update(&mut self, command: Command) {
        match command {
            Command::Up => {
                let next_pos = self.player.pos_up();
                if self.map.out_of_range(&next_pos) {
                    return
                }

                let next_pos = (next_pos.0 as u8, next_pos.1 as u8);
                if self.map.empty_space(&next_pos) {
                    self.player.up();
                }
            }
            Command::Right => {
                let next_pos = self.player.pos_right();
                if self.map.out_of_range(&next_pos) {
                    return
                }

                let next_pos = (next_pos.0 as u8, next_pos.1 as u8);
                if self.map.empty_space(&next_pos) {
                    self.player.right();
                }
            }
            Command::Down => {
                let next_pos = self.player.pos_down();
                if self.map.out_of_range(&next_pos) {
                    return
                }

                let next_pos = (next_pos.0 as u8, next_pos.1 as u8);
                if self.map.empty_space(&next_pos) {
                    self.player.down();
                }
            }
            Command::Left => {
                let next_pos = self.player.pos_left();
                if self.map.out_of_range(&next_pos) {
                    return
                }

                let next_pos = (next_pos.0 as u8, next_pos.1 as u8);
                if self.map.empty_space(&next_pos) {
                    self.player.left();
                }
            }
            _ => {}
        };
        
        if self.player.pos() == self.goal.pos() {
            self.state = State::Clear
        }

    }

    pub fn clear(&self) -> bool {
        self.state == State::Clear
    }

    pub fn map_text(&self) -> String {
        let rows = self.map.rows();
        let mut text = "".to_string();
        for (irow, row) in rows.iter().enumerate() {
            for (icol, cell) in row.iter().enumerate() {
                let pos = Pos::new(irow as u8, icol as u8);

                if self.player.exists(&pos) {
                    text += "●";
                    continue;
                }
                if self.goal.exists(&pos) {
                    text += "★";
                    continue;
                }
                match cell {
                    0 => text += "　",
                    1 => text += "■",
                    _ => text += "　"
                };
            };
            text += "\n";
        };
        text
    }

    pub fn command_text(&self) -> String {
        "[w]:Up, [a]: Left, [s]: Down, [d]: Right, [q]: Quit".to_string()
    }

}
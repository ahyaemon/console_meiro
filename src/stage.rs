use map::Map;
use command::Command;
use player::Player;
use goal::Goal;
use text::Text;
use pos::Pos;

pub struct Stage {
    map: Map,
    player: Player,
    goal: Goal
}

impl Stage {

    pub fn from_text(text: Text) -> Stage {
        let map = Map::new(text.borrow_str());
        let player = Player::new(text.find_position('P'));
        let goal = Goal::new(text.find_position('G'));

        Stage{ map, player, goal }
    }

    pub fn update(&mut self, command: Command) {
        match command {
            Command::Up => {
                // 上に行けるかどうかの判定
                let pos_up = self.player.pos_up();
            }
            Command::Right => self.player.right(),
            Command::Down => self.player.down(),
            Command::Left => self.player.left(),
            _ => {}
        };
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
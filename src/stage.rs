use map::Map;
use command::Command;
use player::Player;
use goal::Goal;

pub struct Stage {
    map: Map,
    player: Player,
    goal: Goal
}
impl Stage {
    
    pub fn from_text(text: String) -> Stage {
        let map = Map::new(&text);
        let player = Player::new(&text);
        let goal = Goal::new(&text);

        Stage{ map, player, goal }
    }

    pub fn update(&self, command: Command) {
        match command {
            Command::Up => self.player.up(),
            _ => {}
        };
    }

    pub fn map_text(&self) -> String {
        let rows = self.map.rows();
        let mut text = "".to_string();
        for (irow, row) in rows.iter().enumerate() {
            for (icol, cell) in row.iter().enumerate() {
                if self.player.exists(irow as u8, icol as u8) {
                    text += "●";
                    continue;
                }
                if self.goal.exists(irow as u8, icol as u8) {
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
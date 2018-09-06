mod command;
mod stage;
mod screen;
mod row;
mod player;
mod goal;
mod map;

fn main() {
    let screen = screen::Screen::new();
    let text_map = text_map();
    let stage = stage::Stage::from_text(text_map);

    screen.show_title();

    loop {
        // 現在のマップの表示
        {
            let map_text = stage.map_text();
            screen.showln(&map_text);
        }

        // コマンドリストの表示
        {
            let command_text = stage.command_text();
            screen.showln(&command_text);
        }

        // コマンド受付
        let command_result = command::Command::from_input();
        match command_result {
            Ok(command) => {
                // ゲーム終了
                if command.is_quit() {
                    break;
                }
                // ここでステージにコマンドを渡す
                else{
                    stage.update(command);
                }
            },
            Err(_) => {
                screen.showln("!! invalid input !!");
                continue;
            }
        };

        // ステージの状態を取得し、分岐する？

    }

}

fn text_map() -> String {
    r#"
1111
P011
100G
1111
    "#.to_string()
}


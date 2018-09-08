mod command;
mod stage;
mod screen;
mod text;

fn main() {
    let text = text::Text::from_str(map_str());
    let screen = screen::Screen::new();
    let mut stage = stage::Stage::from_text(text);

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

        if stage.clear() {
            screen.show_clear();
            break;
        }
    }

}

fn map_str() -> String {
    r#"
1111111
P010001
1011101
1000001
100100G
1111111
    "#.to_string()
}


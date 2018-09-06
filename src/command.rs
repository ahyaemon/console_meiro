use std::io;
use std::fmt;

pub enum Command {
    Up,
    Right,
    Down,
    Left,
    Quit
}

impl Command{
    pub fn from_input() -> Result<Command, String> {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();
        match guess {
            "a" => Ok(Command::Left),
            "s" => Ok(Command::Down),
            "d" => Ok(Command::Right),
            "w" => Ok(Command::Up),
            "q" => Ok(Command::Quit),
            _ => Err("test".to_string())
        }        
    }

    pub fn is_quit(&self) -> bool {
        match self {
            Command::Quit => true,
            _ => false
        }
    }
}

impl fmt::Display for Command{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>{
        let s = match self {
            Command::Up => "Up",
            Command::Right => "Right",
            Command::Down => "Down",
            Command::Left => "Left",
            Command::Quit => "Quit",
            _ => "???"
        };
        write!(f, "{}", s)
    }
}

// --------------------
// 自動入力のみテスト
// --------------------
// #[test]
// fn test_input_up(){
//     let input = get_key(); // input [w].
//     match input {
//         Ok(key) => {
//             match key {
//                 Key::Up => assert!(true),
//                 _ => assert!(false)
//             }
//         }
//         Err(_) => assert!(false)
//     }
// }

// --------------------
// 自動入力のみテスト
// --------------------
// #[test]
// fn test_input_invalid(){
//     let input = get_key(); // input [q].
//     match input {
//         Ok(_) => assert!(false),
//         Err(_) => assert!(true)
//     }    
// }

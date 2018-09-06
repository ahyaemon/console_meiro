use std::io;

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


// -------------------------
// Tests
// 標準入力が必要となるので、ignoreしておく
// -------------------------
#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    #[ignore]
    fn test_input_up(){
        let input = Command::from_input(); // input [w].
        match input {
            Ok(key) => {
                match key {
                    Command::Up => assert!(true),
                    _ => assert!(false)
                }
            }
            Err(_) => assert!(false)
        }
    }

    #[test]
    #[ignore]
    fn test_input_invalid(){
        let input = Command::from_input(); // input [q].
        match input {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }    
    }

}

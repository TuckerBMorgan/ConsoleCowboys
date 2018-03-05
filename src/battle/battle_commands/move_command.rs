use std::io;
use std::collections::HashMap;
use battle::Battlefield;

use party_manager::Character;

pub struct MoveCommand {
    character_name: String,
}

impl MoveCommand {
    pub fn new() -> MoveCommand {
        MoveCommand {
            character_name: String::from("")
        }
    }

    pub fn set_character_name(&mut self, character_name: String) {
        self.character_name = character_name;
    }

    pub fn execute_move_command(&self, battlefield: &mut Battlefield) {
        
    }

    pub fn set_up_move_command(player_character: &HashMap<String, Character>) -> Option<String>{
        //we look at all characters that have some speed left this turn
        //show them to the player
        //they pick the one they want to move, and then ask them to 
        //enter the tile we want them to move to

        let mut keep_looping = true;
        while keep_looping {
            println!("Which character would you like to move"); 
            let mut choices = vec![];
            let mut count = 1;

            for (k, _) in player_character {
                println!("{}. {}", count, k);
                choices.push(k);
                count += 1;
            }
            println!("Type Exit/Quit to stop making command");
        
            let mut input_raw = String::new();
            let input;
            match io::stdin().read_line(&mut input_raw) {
                Ok(_n) => {
                    input = input_raw.trim();
                    let result = input.parse::<usize>();
                    match result {
                        Ok(index) => {
                            //due to index being a usize, we only need to check that it is atleast 0
                            if index - 1 < choices.len() && index > 0 {
                                return Some(choices[index - 1].clone());
//                                println!("{}", choices[index - 1]);
                            }
                            else {
                                println!("{} is not within the correct values, please enter a valid index", index);
                            }
                        },
                        Err(e) => {
                            if input == "Exit" || input == "Quit" || input == "exit" || input == "quit" {
                                keep_looping = false;
                            }
                            println!("ODIN ERROR {} the input was not a number", e);
                        }
                    }
                },
                Err(e) => {
                    panic!("HEMIDALL ERROR {} with player input in the battlefield", e);
                }
            }
        }
        None
    }
}
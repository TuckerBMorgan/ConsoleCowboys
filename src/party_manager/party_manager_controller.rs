use party_manager::{Caravan,Character, RosterView,PartyView};
use system::ExitCodes;
use std::io;

enum PartyMangerControllerState {
    MenuState,
    RosterViewState,
    PartyViewState,
}

pub struct PartyManagerController {
    party_manager_controller_state: PartyMangerControllerState,
    caravan: Caravan,
    roster_view: RosterView,
    party_view: PartyView,
}

impl PartyManagerController {
    pub fn new() -> PartyManagerController {
        
        //test character, will work on this flow more after battle is fleshed out
        let character = Character::new(String::from("Joachim Murat"), 100, 100, 100);
        let mut caravan = Caravan::new(character);

        for _ in 0..10 {
            let character = Character::create_random_character();
            caravan.add_party_member(character.name.clone(), character);
        }
        PartyManagerController {
            party_manager_controller_state: PartyMangerControllerState::MenuState,
            caravan,
            roster_view: RosterView::new(),
            party_view: PartyView::new()
        }
    }

    fn set_state(&mut self, new_state: PartyMangerControllerState) {
        self.party_manager_controller_state = new_state;
    }

    pub fn update(&mut self) -> ExitCodes{
        match self.party_manager_controller_state {
            PartyMangerControllerState::MenuState => {
                println!("What do you want to in the party manager");
                println!("1. Exit to Map");
                println!("2. Exit game");
                println!("3. To Roster View");
                println!("4. To Party View");

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => {
                        let input = input.trim();
                        if input == "1" {
                            return ExitCodes::ToMapController;
                        }
                        else if input == "2" {
                            return ExitCodes::Exit;
                        }
                        else if input == "3" {
                            self.set_state(PartyMangerControllerState::RosterViewState);
                        }
                        else if input == "4" {
                            self.set_state(PartyMangerControllerState::PartyViewState);
                        }
                    }
                    Err(e) => {
                        panic!("{} HEMIDALL ERROR with the input for party manager controller", e);
                    }
                }

            },
            PartyMangerControllerState::PartyViewState => {
                self.party_view();
            },
            PartyMangerControllerState::RosterViewState => {
                self.roster_view();
            }
        }
    //    self.roster_view.update(&mut self.caravan);
        self.roster_view.draw(&self.caravan);

        ExitCodes::Ok
    }

    #[inline]
    pub fn party_view(&mut self) -> ExitCodes {
        self.party_view.draw(&self.caravan);
        ExitCodes::Ok
    }

    #[inline]
    pub fn roster_view(&mut self) -> ExitCodes {
        self.roster_view.draw(&self.caravan);
        ExitCodes::Ok
    }
}
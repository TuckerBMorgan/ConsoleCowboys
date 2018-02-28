use party_manager::Caravan;

pub struct RosterView {
    
}

impl RosterView {
    
    pub fn new() -> RosterView{
        RosterView {

        }
    }

    pub fn draw(&self, caravan: & Caravan) {
        println!("\n\nThe Caravan roster");
        let party = caravan.get_party();
        for name in caravan.get_active_roster() {
            let v = party.get(name);
            if v.is_none() {
                continue;
            }
            let v = v.as_ref().unwrap();
            println!("-=========================-");
            if caravan.is_lead(&v.name) {
                println!("Lead : {}", v.name);
            }
            else {
                println!("Party Member: {}", v.name);
            }
            println!("Health: {}", v.total_health);
            println!("Attack: {}", v.total_attack);
            println!("Speed: {}", v.total_speed);
        }
        println!("-=========================-");
    }
}


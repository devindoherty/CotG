use std::io::{stdin, stdout, Write};

struct GameState {
    roster: Roster,
    mode: RunMode,
    orders: Vec<String>,
}

enum RunMode {Inputting, Updating, Outputting}

struct Roster {
    watchmen_active: i32,
    men_at_arms: i32,
    sarjents: i32,
    brutes: i32,
    inquisitors: i32,
    rat_catchers: i32,
    witch_hunter: i32,
    sorcerer: i32,
}

struct Event {
    name: String,
    threat: i32,
    desc: String,
}

impl Event {
    fn kill(gs: &mut GameState) {
        gs.roster.men_at_arms -= 1;
    }
}

fn main() {

    // let intro_one = concat!("Congratulations on your appointment, captain. The city \n", 
    //                         "guard were in desparate need of a new leader.\n",
    //                         );


    let roster = Roster {
        watchmen_active: 5,
        men_at_arms: 4,
        sarjents: 1,
        brutes: 0,
        inquisitors: 0,
        rat_catchers: 0,
        witch_hunter: 0,
        sorcerer: 0,
    };
    
    let mut order_stream = Vec::new();

    let mut gs = GameState {
        roster: roster,
        mode: RunMode::Inputting,
        orders: order_stream,
    };

    println!(r" 
         ____            _        _                __   _   _             ____                     _ 
        / ___|__ _ _ __ | |_ __ _(_)_ __     ___  / _| | |_| |__   ___   / ___|_   _  __ _ _ __ __| |
        | |   / _` | '_ \| __/ _` | | '_ \   / _ \| |_  | __| '_ \ / _ \ | |  _| | | |/ _` | '__/ _`|
        | |__| (_| | |_) | || (_| | | | | | | (_) |  _| | |_| | | |  __/ | |_| | |_| | (_| | | | (_||
        \____\__,_| .__/ \__\__,_|_|_| |_|  \___/|_|    \__|_| |_|\___|  \____|\__,_|\__,_|_|  \__,_|"
    );
    println!("\nPress any key to continue...");
    
    println!("ACTIVE DUTY MEN: {}", gs.roster.watchmen_active);
    
    'game: loop {
        input(&mut gs);
        update(&mut gs);
        output(&mut gs);
    }
    

    // let mut s = String::new();
    // println!("There are currently {} active duty watchmen.", watchmen_active);
    // print!("How many watchmen do you wish assigned to the city gates? > ");
    // let _=stdout().flush();
    // stdin().read_line(&mut s).expect("Did not enter a correct string");
    // println!("{} men will be assigned to man the city gates.", s);
    

}

fn input(gs: &mut GameState){
    let mut cmd = String::new();
    println!("Your command?");
    stdin().read_line(&mut cmd).unwrap();
    println!("cmd: {}", cmd);
    gs.orders.push(cmd);
}

fn update(gs: &mut GameState) {
    for order in &gs.orders {
        if order == "kill\r\n" {
            gs.roster.watchmen_active -= 1;
        }
    }
    // &mut gs.orders.pop();
}

fn output(gs: &GameState) {
    println!("CURRENT ACTIVE DUTY MEN: {}", gs.roster.watchmen_active);
    println!("Orders Vec: {:?}", gs.orders);
}

use std::io;

fn main() {
    use std::io::{stdin, stdout, Write};
    let intro_one = concat!("Congratulations on your appointment, captain. The city \n", 
                            "guard were in desparate need of a new leader.\n",
                            );
    
    let intro_two = concat!("Hunters and woodsmen have reported strange sightings in \n",
                     "the forests surrounding the city, captain. It is best we \n",
                     "keep the gates fully manned throughout the night watch. \n",
                     );
    
    let _assn_message = "Captain! There is a master assassin stalking the\n
       Fleastall Burough. He has three wealthy nobleman already.";

    let watchmen_active = 19;
    let men_at_arms_active = 6;
    let sarjents_active = 4;
    let brutes_active = 1;
    let inquisitors_active = 2;
    let rat_catchers_active = 1;
    let wizards_active = 0;


    println!("{}", intro_one);
    println!("{}", intro_two);

    let mut s = String::new();
    println!("There are currently {} active duty watchmen.", watchmen_active);
    print!("How many watchmen do you wish assigned to the city gates? > ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("{} men will be assigned to man the city gates.", s);

    

    
}

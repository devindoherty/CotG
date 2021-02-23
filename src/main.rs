fn main() {
    use std::io::{stdin, stdout, Write};
    let intro_one = "Captain! There is a master assassin stalking the\nFleastall Burough. He has slain five lords already.";
    let intro_two = "Congratulations on your appointment, captain. The city\nguard were in desparate need of a new leader.\n";
    
    println!("{}", intro_one);
    println!("{}", intro_two);

    let mut s = String::new();
    print!("How many watchmen do you wish assigned to the city gates? > ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}", s);

    
}

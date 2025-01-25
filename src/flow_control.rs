use rand::Rng;

pub fn show() {
    if_condition();
    match_expression();
}

fn if_condition() {
    // 🚨 note that, in Rust, IF is an expression, not a statement

    let animal = random_animal();
    println!("IF STATEMENT:\n  {}: ", animal);
    if animal == "dog" {
        println!("Bark!");
    } else if animal == "cat" {
        println!("Meow!");
    } else if animal == "duck" {
        println!("Quack!");

    } else {
        println!("whatever it sounds like...");
    }

    let age = random_age();
    let can_drive = if age >= 16 { "Yep" } else { "Nope" };
    let can_vote = if age >= 18 { "Yep" } else { "Nope" };
    let can_drink = if age >= 21 { "Yep" } else { "Nope" };

    println!("IF EXPRESSION:");
    println!("  Can drive: {}!", can_drive);
    println!("  Can vote:  {}!", can_vote);
    println!("  Can drink: {}!", can_drink);
    println!();
}

fn match_expression() {
    let dice_roll: u8 = random_dice_roll();
    let points: u8 = match dice_roll {
        1 => 100,
        5 => 200,
        _ => 0,
    };

    #[allow(dead_code)]
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(&'static str),
    }

    let coin = Coin::Nickel;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    };

    println!("MATCH EXPRESSION:");
    println!("  Dice stopped at {}, scoring {} points", dice_roll, points);
    println!("  {:?} is worth {} cents", coin, value);
    println!();
}

fn random_animal() -> &'static str {
    let animals = ["dog", "cat", "duck", "chinchilla", "giraffe"];
    let mut rng = rand::thread_rng();
    let animal = animals[rng.gen_range(0..animals.len())];

    animal
}

fn random_age() -> u8 {
    let mut rng = rand::thread_rng();
    let age = rng.gen_range(1..=100);

    age
}

fn random_dice_roll() -> u8 {
    let mut rng = rand::thread_rng();
    let dice_roll = rng.gen_range(1..=6);

    dice_roll
}

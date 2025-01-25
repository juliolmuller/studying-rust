use rand::Rng;

pub fn show() {
    if_condition();
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

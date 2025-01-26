use rand::Rng;

pub fn show() {
    if_condition();
    match_expression();
    lopping_with_loop();
    lopping_with_labeled_loop();
}

fn if_condition() {
    // 🚨 note that, in Rust, IF is an expression, not a statement

    let animal = random_animal();
    print!("IF STATEMENT:\n  {}: ", animal);
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

fn lopping_with_loop() {
    // 🚨 note that, in Rust, LOOP is an expression, not a statement
    println!("LOOP:");

    let mut num1: u16 = 0;
    let mut num2: u16 = 1;
    print!("  Fibonacci Sequence: ");
    let msg = loop {
        print!("{}", num1);

        let next = num1 + num2;
        num1 = num2;
        num2 = next;

        if num2 > 100 {
            break "🚀"
        } else {
            print!(", ");
        }
    };

    println!("... {msg}");
    println!();
}

fn lopping_with_labeled_loop() {
    println!("LABELED LOOPS:");

    let number = (random_age() + 1) as u16;
    let mut i: u16 = 2;
    print!("  Prime numbers between 0 and {}: ", number);
    'outer: loop {
        let mut j = 2;
        let mut is_prime = true;

        #[allow(unused_labels)]
        'inner: loop {
            if i > number {
                println!();
                break 'outer;
            }

            if i * i < j {
                break 'inner;
            }

            if i % j == 0 && i / j != 1 {
                is_prime = false;
                break 'inner;
            }

            j += 1;
        }

        if is_prime {
            if i != 2 {
                print!(", ");
            }

            print!("{}", i);
        }

        i += 1;
    };

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
    let age = rng.gen_range(1..=70);

    age
}

fn random_dice_roll() -> u8 {
    let mut rng = rand::thread_rng();
    let dice_roll = rng.gen_range(1..=6);

    dice_roll
}

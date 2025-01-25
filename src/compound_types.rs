pub fn show() {
    string_types();
    arrays();
    tuples();
    slices();
}

fn string_types() {
    let a: &str = "Hello, there!";
    let mut b: String = String::from("Hello, there!");

    b.push_str(" 👋");

    println!("STRING TYPE:");
    println!("  &st: {}", a);
    println!("  String: {}", b);
    println!();
}

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [f32; 5] = [3.1415; 5];
    let c: [&str; 3] = ["Hello", "there", "!"];

    println!("ARRAY TYPE:");
    println!("  a: {:?}", a);
    println!("  b: {:?}", b);
    println!("  c: {:?}", c);
    println!();
}

fn tuples() {
    let a: (&str, f32, bool) = ("Hello, there!", 3.1415, true);

    println!("TOUPLE TYPE:");
    println!("  a: {:?}", a);
    println!();
}

fn slices() {
    let int_slice: &[i32] = &[1, 2, 3, 4, 5];
    let derived_slice: &[i32] = &int_slice[1..3];
    let animals_slice: &[&str] = &["dog", "cat", "bird", "fish"];
    let movies_slice: &[String] = &[
        "Star Wars".to_string(),
        "Duna".to_string(),
        "Interstellar".to_string(),
    ];

    println!("SLICE TYPE:");
    println!("  derived_slice: {:?}", derived_slice);
    println!("  animals_slice: {:?}", animals_slice);
    println!("  movies_slice:  {:?}", movies_slice);
    println!();
}

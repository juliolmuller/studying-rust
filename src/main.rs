fn main() {
    integer_types();
    floating_point_types();
    boolean_types();
    character_types();
    string_types();
    arrays();
    tuples();
    slices();
}

fn integer_types() {
    // Signed integers
    let a: i8 = i8::MAX;
    let b: i16 = i16::MAX;
    let c: i32 = i32::MAX;
    let d: i64 = i64::MAX;
    let e: i128 = i128::MAX;

    // Unsigned integers
    let f: u8 = u8::MAX;
    let g: u16 = u16::MAX;
    let h: u32 = u32::MAX;
    let i: u64 = u64::MAX;
    let j: u128 = u128::MAX;

    println!("INTEGER TYPE:");
    println!("  Signed integers:   {}, {}, {}, {}, {}", a, b, c, d, e);
    println!("  Unsigned integers: {}, {}, {}, {}, {}", f, g, h, i, j);
    println!();
}

fn floating_point_types() {
    let a: f32 = f32::MIN;
    let b: f32 = f32::MAX;
    let c: f64 = f64::MIN;
    let d: f64 = f64::MAX;

    println!("FLOATING POINT TYPE:");
    println!("  f32: {} to {}", a, b);
    println!("  f64: {} to {}", c, d);
    println!();
}

fn boolean_types() {
    let a: bool = true;
    let b: bool = false;

    println!("BOOLEAN TYPE:");
    println!("  true: {}", a);
    println!("  false: {}", b);
    println!();
}

fn character_types() {
    let a: char = 'a';
    let b: char = 'b';
    let c: char = 'c';

    println!("CHARACTER TYPE:");
    println!("  a: {}", a);
    println!("  b: {}", b);
    println!("  c: {}", c);
    println!();
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

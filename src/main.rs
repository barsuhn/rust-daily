fn main() {
   integer_conversion();
    floating_point();
    string_and_char();
}

fn integer_conversion() {
    let a:u8 = 42;
    let b:u16 = a.into();
    let c:u16 = 256;
    let d:Result<u8,_> = c.try_into();

    print!("b = {b}");

    if d.is_err() {
        println!(" but d has no value.");
    } else {
        println!(" and d has a value");
    }
}

fn floating_point() {
    let a:f32 = 21.0 * 2.0;
    let b:f64 = 42.into();

    println!("a = {}, b = {}", a, b);
}

fn string_and_char() {
    let emotional_greet:&str = "Hi 😀!";

    for c in emotional_greet.chars() {
        let character:char = c;

        print!("{character}");
    }
    println!();
}
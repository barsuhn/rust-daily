fn main() {
    integer_conversion();
    floating_point();
    string_and_char();
    booleans();
    references();
    references_tomut();
    mut_references();
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

fn booleans() {
    let mut x:bool = false;

    x = !x;

    if x {
        println!("True!");
    }
}

fn references() {
    let u:u32 = 42;
    let v:&u32 = &u;

    println!("u:{}, *v:{}", u, *v);
}

fn mut_references() {
    let u:u32 = 21;
    let v:u32 = 2*u;
    let mut w:&u32 = &u;

    print!("u:{}, v:{}, *w:{}", u, v, *w);

    w = &v;

    println!(" -> *w:{}", *w);
}

fn references_tomut() {
    let mut x:u32 = 84;
    let y:&mut u32 = &mut x;

    *y = *y / 2;

    println!("x:{}", x);
}
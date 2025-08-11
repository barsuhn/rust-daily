fn main() {
    declaration();
    mutability();
    shadowing();
    conditional_statement();
    conditional_expression();
    endless_loop();
    while_loop();
    for_loop();
}

struct Value(i32);

impl Value {
    fn new(v: i32) -> Self {
        Value(v)
    }
}

fn declaration() {
    let x = 5;
    let y;

    if x % 2 == 0 {
        y = x;
    } else {
        y = 2 * x;
    }

    let z = Value::new(y);

    println!("The value of z is: {}", z.0);
}

fn mutability() {
    let x = 21;
    let mut y = x;

    y = y * 2;

    println!("x = {x}, y = {y}");
}

fn shadowing() {
    let x = 21;
    let y = 5;

    print!("x = {x}, y = {y} --> ");

    let x = 2 * x;
    let y = "I changed";

    println!("x = {x}, y = {y}");
}

fn conditional_statement() {
    let x = 5;

    if x % 2 == 0 {
       println!("The value is even!");
    } else {
       println!("The value is odd!");
    }
}

fn conditional_expression() {
    let x = 6;
    let kind = if x % 2 == 0 { "even" } else { "odd" };

    println!("The kind of value is {kind}!");
}

fn endless_loop() {
    let mut i = 0;

    let result = loop {
        if i >= 5 { break i; }
        print!("{i}, ");
        i = i + 1;
    };

    println!("{result}");
}

fn while_loop() {
    let mut i = 1;

    print!("{}", i-1);
    while i <= 5 {
        print!(", {i}");
        i = i + 1;
    }

    println!();
}

fn for_loop() {
    print!("0");

    for i in 1..6 {
        print!(", {i}");
    }

    println!();
}
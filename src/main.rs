fn main() {
    println!("Collections");

    tuple_create();
    tuple_patterns();
    tuple_pattern_placeholder();
    tuple_ordering();
}

fn tuple_create() {
    let tuple = (1, true, "Rust".to_string());
    let integer = tuple.0;
    let boolean = tuple.1;
    let string = &tuple.2;

    println!();
    println!("Tuple: {:?} -> {} {} {}", tuple, integer, boolean, string);
}

fn tuple_patterns() {
    let mut tuple = (1, false, "Rust".to_string(), 21.0);
    let (integer, ref mut boolean, ref string, mut number) = tuple;

    *boolean = !*boolean;
    number = 2.0 * number;

    println!();
    println!("Pattern -> {} {} {} {}", integer, boolean, string, number);
    println!("Tuple {:?}", tuple);
}

fn tuple_pattern_placeholder() {
    let numbers = (1, 2, 3, 4, 5);
    let (one,_,_,four,_) = numbers;
    let (first, .., last) = numbers;
    let (.., end) = numbers;
    let (start, ..) = numbers;

    println!();
    println!("Numbers: {:?} -> {} _ _ {} _", numbers, one, four);
    println!("Mid rest: {:?} -> {} .. {}", numbers, first, last);
    println!("Start rest: {:?} -> .. {}", numbers, end);
    println!("End rest: {:?} -> {} ..", numbers, start);
}

fn tuple_ordering() {
    let mut tuples = [(2, "Ferris"), (2, "Bob"), (1, "Rust")];

    println!();
    println!("Tuples: {:?}", tuples);
    println!("{:?} == {:?} -> {}", tuples[0], tuples[1], tuples[0] == tuples[1]);
    println!("{:?} != {:?} -> {}", tuples[0], tuples[1], tuples[0] != tuples[1]);
    println!("{:?} > {:?} -> {}", tuples[0], tuples[1], tuples[0] > tuples[1]);

    tuples.sort();

    println!("Sorted: {:?}", tuples);
}
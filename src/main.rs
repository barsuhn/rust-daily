fn main() {
    println!("Collections");

    tuple_create();
    tuple_patterns();
    tuple_pattern_placeholder();
    tuple_ordering();

    array_patterns();

    range_collect();
    range_patterns();

    vector_create();
    vector_stack();
    vector_extend();
    vector_splice();

    convert_to_slice();
    mut_slices();
    ranged_slices();
    slice_get();
    slice_concat();
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

fn array_patterns() {
    let numbers = [1, 2, 3, 4];
    let [_, two, _, four] = numbers.clone();

    println!();
    println!("{numbers:?} -> {} {two} {} {four}", numbers[0], numbers[2]);

    let empty: [u8;0] = [];
    println!("{empty:?}");
}

fn range_collect() {
    let range = 0..10;

    println!();
    for i in range.clone() {
        print!("{i} ");
    }
    println!("= {range:?}");

    let chars = 'a'..='f';
    let vec = chars.collect::<Vec<char>>();
    println!("{vec:?}");
}

fn range_patterns() {
    let number = 93;

    println!();
    print!("{number} is ");
    match number {
        1 => println!("one"),
        2..=100 => println!("a lot"),
        _ => println!("even more")
    }
}

fn vector_create() {
    let empty: Vec<i32> = vec![];
    let numbers = vec![1, 2, 3, 4, 5];
    let guter_rat = vec!["üben"; 3];

    println!();
    println!("{empty:?}, {numbers:?}, {guter_rat:?}");
}

fn vector_stack() {
    let mut numbers : Vec<i32> = vec![];

    numbers.push(1);
    numbers.push(2);

    let top = numbers.pop();

    numbers.push(3);
    numbers.insert(1,4);
    numbers.remove(0);

    println!();
    println!("{numbers:?}, {top:?}");
}

fn vector_extend()
{
    let mut numbers = vec![1, 2];
    let mut more_numbers = vec![3, 4];

    numbers.append(&mut more_numbers);

    println!();
    println!("append: {numbers:?}, {more_numbers:?}");

    let mut numbers = vec![1, 2];
    let more_numbers = vec![3, 4];

    numbers.extend(more_numbers);

    // println!("extend: {numbers:?}, {more_numbers:?}"); // Fehler!
    println!("extend: {numbers:?}");

    let mut numbers = vec![1, 2];
    let more_numbers = vec![3, 4];

    numbers.extend_from_slice(&more_numbers);

    println!("extend_from_slice: {numbers:?} {more_numbers:?}");
}

fn vector_splice() {
    let mut v = vec![1, 4, 27, 33, 11];

    v.splice(1..1, [2,3]);
    v.splice(4..6, [5,6,7,8,9,10]);
    v.splice(11..11, [12,13,14]);

    println!();
    println!("{v:?}");
}

fn convert_to_slice() {
    let vec = vec![1, 2, 3];
    let array = [4, 5, 6];
    let vector_slice = vec.as_slice();
    let array_slice = array.as_slice();

    println!();
    println!("vector as slice: {vector_slice:?}, array as slice: {array_slice:?}");

    let vector_slice: &[i32] = &vec;
    let array_slice: &[_] = &array;

    println!("&vector: {vector_slice:?}, &array: {array_slice:?}");
}

fn mut_slices() {
    let mut vec = vec![1, 2, 3];
    let mut array = [4, 5, 6];
    let slice = array.as_slice();

    vec.push(slice[0]);

    let vec_slice = vec.as_mut_slice();
    let array_slice: &mut [_] = &mut array;

    (vec_slice[0], array_slice[2]) = (array_slice[2], vec_slice[0]);

    println!();
    println!("vec_slice: {vec_slice:?}, array: {array:?}");
}

fn ranged_slices() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    let slice = &mut vec[2..5];

    (slice[0], slice[2]) = (slice[2], slice[0]);

    println!();
    println!("vec: {vec:?}");
}

fn slice_get() {
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[2..5];

    println!();
    print!("slice get: ");
    for i in 0..5 {
        match slice.get(i) {
            Some(value) => print!("{value} "),
            None => print!(". ")
        }
    }
    println!();
}

fn slice_concat() {
    let parts = ["hello", ",", " ", "world", "!"];
    let concat = parts.concat();

    println!();
    println!("concat result: {concat}");
}
fn main() {
    println!("Strings");

    slice_memory();
    slice_utf8();
    slice_iteration();
    slice_parsing();
    slice_splitting();

    byte_string();
    raw_string();

    string_creation();
    string_format();
    string_replace();
}

fn slice_memory() {
    let greeting = "Hello, world!";

    println!();
    println!("slice: {}", greeting);
    println!("reference: {:p}", greeting); // the reference is a fat pointer
    println!("address: {:p} length: {}", greeting.as_ptr(), greeting.len());
}

fn slice_utf8() {
    let greeting = "Grüße!";

    println!();
    println!("slice: {}", greeting);
    println!("reference: {:p}", greeting);
    println!("address: {:p} length: {} characters: {}", greeting.as_ptr(), greeting.len(), greeting.chars().count());
}

fn slice_iteration() {
    let greeting = "Hello, Länd!";

    println!();
    println!("slice: {}", greeting);
    print!("bytes: ");
    for byte in greeting.bytes() {
        print!("{:x} ", byte);
    }
    println!();

    print!("chars: ");
    for char in greeting.chars() {
        print!("{} ", char);
    }
    println!();

    print!("char indices: ");
    for index in greeting.char_indices() {
        print!("{:?} ", index);
    }
    println!();
}

fn slice_parsing() {
    let number = "42.0".parse::<f64>().unwrap();
    let integer: i32 = "42".parse().unwrap();

    println!();
    println!("slice parsing: {:?} -> {:?}", "42.0", number);
    println!("parsing with type inference: {:?} -> {:?}", "42", integer);
}

fn slice_splitting() {
    let colors = "{red, green , blue , yellow}";
    let parts = colors.split(", ");

    println!();
    print!("Parts: ");
    for part in parts.clone() {
        print!("'{part}' ");
    }
    println!();

    print!("Trimmed: ");
    for part in parts.clone() {
        let trimmed = part.trim();
        print!("'{trimmed}' ");
    }
    println!();

    print!("Colors: ");
    let trim_chars: &[char] = &['{', '}', ' '];
    for part in parts {
        let trimmed = part.trim_matches(trim_chars);
        print!("'{trimmed}' ");
    }
    println!();
}

fn byte_string() {
    let greeting = b"Hello, world!";

    println!();
    println!("byte string: {:?}", greeting);
}

fn raw_string() {
    let path = r"C:\Users\Rust";
    let quoted = r#"In "Anführungszeichen""#;
    let hashed = r##"In #"Extremo"#"##;

    println!();
    println!("raw string: {}", path);
    println!("string with quotes: {}", quoted);
    println!("string with hashed quotes: {}", hashed);
}

fn string_creation() {
    let mut greeting = "Hello".to_string();

    greeting.push_str(", ");
    greeting = greeting + "world";
    greeting.push('!');

    println!();
    println!("string: {}", greeting);
    println!("rusty: {}", String::from("Let's get rusty!"));
}

fn string_format() {
    let number = 42;
    let string = format!("{}", number);
    let value: i32 = string.parse().unwrap();

    println!();
    println!("conversions: {:?} -> {:?} -> {:?}", number, string, value);
}

fn string_replace() {
    let greeting = "Hello, world!";
    let replaced = greeting.replace("world", "Rust");

    println!();
    println!("Replaced: {} -> {}", greeting, replaced);
}
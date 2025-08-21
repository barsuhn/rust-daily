enum Colors {
    Red, Green, Blue
}

pub fn print_colors() {
    let colors = vec![Colors::Red, Colors::Green, Colors::Blue];

    for color in colors.iter() {
        match color {
            Colors::Red => print!("red "),
            Colors::Green => print!("green "),
            Colors::Blue => print!("blue ")
        }
    }
    println!();
}

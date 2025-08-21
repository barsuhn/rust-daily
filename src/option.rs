pub fn print_options() {
    let options = [
        Option::Some("Hello,"),
        Option::None,
        Option::Some("world!")
    ];

    print!("options: ");
    for option in options {
        match option {
            Option::Some(s) => print!("{s}"),
            Option::None => print!(" ")
        };
    }
    println!();
}

pub fn sum_options() {
    let options = [
        Option::Some(22),
        Option::None,
        Option::Some(20),
    ];

    let mut sum = 0;
    for option in options.iter() {
        if let Some(n) = option {
            sum += n;
        }
    }
    println!("if let: {}", sum);

    let mut sum = 0;
    for option in options.iter() {
        let Some(n) = option else {
            continue;
        };

        sum += 2*n;
    }
    println!("let .. else: {}", sum);
}
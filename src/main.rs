mod colors;
mod geometry;
mod option;
mod result;

use colors::print_colors;
use geometry::print_shapes;
use option::{print_options, sum_options};
use crate::result::print_parsed_values;

fn main() {
    print_colors();

    println!();
    print_shapes();

    println!();
    print_options();

    println!();
    sum_options();

    println!();
    print_parsed_values();
}

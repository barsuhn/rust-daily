mod person;
mod exam;

use person::make_person;
use exam::Exam;

fn main() {
    let mut buddy = make_person("Bob", 41);

    println!("Das ist mein Kumpel {}. Er war gestern noch {} Jahre alt.", buddy.name, buddy.age);
    buddy.age += 1;
    println!("Heute ist aber sein Geburtstag und er ist {} Jahre alt geworden.", buddy.age);

    let mut exam = Exam::new(buddy, "Sackhüpfen");

    exam.print_certificate();
    exam.set_grade(3);
    exam.print_certificate();
}

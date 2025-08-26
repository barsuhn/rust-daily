mod person;
mod exam;
mod todo;

use person::make_person;
use exam::Exam;
use todo::ToDo;

fn main() {
    let mut buddy = make_person("Bob", 41);

    println!("Das ist mein Kumpel {}. Er war gestern noch {} Jahre alt.",
             buddy.name, buddy.age);
    buddy.age += 1;
    println!("Heute ist aber sein Geburtstag und er ist {} Jahre alt geworden.",
             buddy.age);

    let mut exam = Exam::new(buddy, "Sackhüpfen");

    exam.print_certificate();
    exam.set_grade(3);
    exam.print_certificate();

    println!();
    process_todos();
}

fn process_todos() {
    let mut todos = [
        ToDo::new(false, "Dinner"),
        ToDo::new(false, "Laundry"),
        ToDo::new(false, "Dishes"),
    ];

    for i in 0..3 {
        let todo = &todos[i];

        println!("Item {}: {} is {}", i, todo.1,
                 if todo.is_done() {"done"} else {"open"});
    }

    todos[0].check();
    todos[2].check();

    println!();
    for (i, todo) in todos.iter().enumerate() {
        println!("Item {}: {} is {}", i, todo.1,
                 if todo.is_done() {"done"} else {"still open"});
    }
}
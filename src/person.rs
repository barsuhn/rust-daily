pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn make_person(name: &str, age: u8) -> Person {
    Person {
        name: name.to_string(),
        age,
    }
}

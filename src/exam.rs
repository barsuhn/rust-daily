use super::person::Person;

pub struct Exam {
    attendee: Person,
    subject: String,
    grade: Option<u8>,
}

impl Exam {
    pub fn new(attendee: Person, subject: &str) -> Self {
        Self {
            attendee,
            subject: subject.to_string(),
            grade: None,
        }
    }

    pub fn set_grade(&mut self, grade: u8) {
        self.grade = Some(grade);
    }

    pub fn print_certificate(&self) {
        match self.grade {
            Some(grade) =>  {
                if grade > 4 {
                    println!("{} hat die Prüfung in {} nicht bestanden.", self.attendee.name, self.subject);
                } else {
                    println!("{} hat die Prüfung in {} mit der Note {} bestanden.", self.attendee.name, self.subject, grade);
                }
            }
            None => println!("{} hat die Prüfung in {} noch nicht absolviert.", self.attendee.name, self.subject),
        }
    }
}
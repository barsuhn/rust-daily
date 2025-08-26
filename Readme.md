# Strukturen

## Module und Sichtbarkeit

Wenn der Code auf mehrere Dateien aufgeteilt wird, bezeichnet man jede Datei als ein Modul. Der Name des Moduls
wird durch den Namen der Datei vorgegeben. Alle Elemente eines Moduls sind normalerweise nur innerhalb des 
Moduls sichtbar. Um ein Programmkonstrukt in einem Modul für andere Module sichtbar zu machen, muss das
das Konstrukt mit dem Schlüsselwort `pub` als öffentlich (public) gekennzeichnet werden. In diesem Abschnitt 
werden alle Strukturdefinitionen in einem eigenständigen Modul definiert. Alles, was in diesem Modul nicht 
mit `pub` markiert wird, bleibt damit für andere Module unsichtbar.

## Deklaration

Zusammengesetzte Datenstrukturen werden in Rust mit dem Schlüsselwort `struct` deklariert. In geschweiften 
Klammern werden dann die Elemente der Struktur aufgelistet. Die einzelnen Elemente bestehen aus einem Namen und
einem Typ der durch einen Doppelpunkt getrennt hinter dem Namen geschrieben wird. Wenn das Strukturelement 
nach außen sichtbar sein soll, dann muss es als öffentlich (`pub`) gekennzeichnet werden. Die Elemente werden 
durch Kommata getrennt, wobei auch hinter dem letzten Element ein Komma stehen darf. Ein Komma hinter alle 
Elemente zu schreiben, erleichtert die spätere Erweiterung der Strukturen.

Zur Erzeugung einer Strukturinstanz wird der Name der Struktur gefolgt von einer Initialisierungsliste
in geschweiften Klammern `{}` angegeben. Die Initialisierungsliste enthält Paare von Namen und Werten, die durch 
Doppelpunkte voneinander getrennt werden. Die Paare werden wiederum durch Kommata getrennt. Wird ein Strukturelement 
mit einer Variablen initialisiert, deren Name mit dem Elementnamen übereinstimmt, so kann einfach der Name der 
Variablen verwendet werden. Im folgenden Beispiel kann statt `age: age` in der Initialisierungsliste einfach `age` 
geschrieben werden. Beim Element `name` ist das nicht der Fall, weil die Parametervariable zwar den gleichen Namen,
aber nicht den gleichen Typ wie das Strukturelement hat.

```rust
pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn person(name: &str, age: u8) -> Person {
    Person {
        name: name.to_string(),
        age,
    }
}
```

## Implementierungen

Strukturen können auch Funktionen zugeordnet werden. Dies wird mithilfe eines `impl` Blocks definiert. Die
Funktionen können dabei als ersten Parameter eine Referenz auf ein Objekt der zugeordneten Struktur erhalten. 
In diesem Fall wird die Funktion den Instanzen der Struktur zugeordnet. Ist die Referenz auf die Strukturinstanz 
zudem veränderbar, so kann die Funktion die Instanz auch verändern. Wenn kein Instanzparameter zugeordnet wird, 
dann wird die Funktion für den Strukturtyp deklariert.

Die Funktion `new` im folgenden Beispiel ist dem Typ `Exam` zugeordnet. Die Funktionen `set_grade()` und
`print_certificate()` haben jedoch einen `self` Parameter, was sie zu Instanzfunktionen macht. Die Referenz 
auf die Instanz muss dabei den Namen `self` haben. Der Typ wird nicht angegeben, da der `impl` Block bereits 
den Typ festlegt. Die Funktion `set_grade()` bekommt self sogar als `&mut`, was eine Veränderung der Instanz 
ermöglicht.

Das kaufmännische und `&` wird in diesem Kontext verwendet, um eine sogenannte Referenz zu deklarieren. Wenn 
das `&` fehlt, dann würde der Parameter als Wert übergeben, was in den meisten Fällen bedeutet, das das 
Funktionsargument für den Aufrufer ungültig wird. Man spricht dabei von der Übertragung der Eigentümerschaft 
am übergebenen Wert an die Funktion. Referenzen werden hingegen nur *geborgt*. Die Eigentümerschaft bleibt 
dabei beim Aufrufer. Eigentümerschaft und *borgen* (*borrowing*) werden in einem anderen Abschnitt ausführlicher 
erläutert.

```rust
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
                    println!("{} hat die Prüfung in {} nicht bestanden.",
                             self.attendee.name, self.subject);
                } else {
                    println!("{} hat die Prüfung in {} mit der Note {} bestanden.",
                             self.attendee.name, self.subject, grade);
                }
            }
            None => println!("{} hat die Prüfung in {} noch nicht absolviert.",
                             self.attendee.name, self.subject),
        }
    }
}
```

## Tupel-Strukturen

Strukturen können alternativ auch als Tupel definiert werden. In diesem Fall werden die Elemente in runden Klammern
als Liste von Typen angegeben. Um ein Element auch außerhalb des Moduls sichtbar zu machen, muss vor dem entsprechenden
Typeintrag in der Liste das Schlüsselwort `pub` angegeben werden.

```rust
pub struct ToDo(bool, pub String);
```

Auch für Tupel-Strukturen können Implementierungen angegeben werden. Auf die Elemente einer Tupel-Struktur wird, wie 
auf die Elemente eines Tupels mit dem Index des Elements zugegriffen.

```rust
impl ToDo {
    pub fn new(status: bool, task: &str) -> Self {
        ToDo(status, task.to_string())
    }

    pub fn check(&mut self) {
        self.0 = true;
    }

    pub fn is_done(&self) -> bool {
        self.0
    }
}
```

## Verwendung

Im übergeordneten Modul müssen Untermodule mit dem Schlüsselwort `mod` deklariert werden. In diesem Fall ist das
übergeordnete Modul die Datei `main.rs`, welche die Wurzel des Modulbaums der ausführbaren Datei bildet. Zudem 
müssen Typen und Funktionen aus einem anderen Modul mittels `use` importiert werden, wenn diese in einem 
anderen Modul verwendet werden sollen.

Die Variable `buddy` ist eine Instanz der Struktur `Person`. Mit dem Punkt-Operator `.` kann man auf die Elemente
einer Struktur zugreifen. Da sie mit `let mut` deklariert wurde, kann ein Strukturelement sogar auf der linken Seite
einer Zuweisung stehen.

Die Variable `exam` erzeugt mithilfe der Funktion `new` eine neue Instanz der Struktur `Exam`. Es handelt sich um
eine Typfunktion und wird deshalb mit dem Typnamen gefolgt von zwei Doppelpunkten `::` verwendet. Da die 
verändernde Instanzfunktion `set_grade()` von `exam` aufgerufen wird, muss die Variable ebenfalls mit `let mut` 
deklariert werden. Instanzfunktionen werden mit der Instanzvariablen aufgerufen und mit dem Punkt-Operator `.` 
vom Variablennamen getrennt.

In der Funktion `process_todos()` wird der Tupel-Strukturtyp `ToDo` verwendet. Das Element `.0` des Tupels ist aber
außerhalb des `todo`-Moduls nicht sichtbar. Es kann nur indirekt über die Funktionen `is_done()` und `check()` 
verwendet werden. Ds das Element `.1` mit `pub` als öffentlich deklariert wurde, kann auch außerhalb des Moduls
direkt darauf zugegriffen werden. Es ist jedoch fast immer besser, wenn außerhalb des definierenden Moduls nur 
über Funktionen auf die Elemente zugegriffen wird, da sie im Allgemeinen besser lesbar sind.

```rust
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
```

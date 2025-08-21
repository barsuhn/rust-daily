# Enumerationen und Pattern Matching

Enumerationen sind Typen mit denen zwischen verschiedenen Varianten eines Werts unterschieden werden kann. Analog
zu Strukturen können auch für Enumerationen `impl` Blöcke definiert werden. 

## Eigene Enumerationstypen

Eine Enumeration ist im Grunde genommen eine Fallunterscheidung, die eine feste Anzahl von Werten annehmen kann. 
Die Werte einer Enumeration sind dabei selbst Bezeichner mit den gleichen Namensregeln wie für Typen und Variablen.
Mit einem `match` Ausdruck können die Fälle einer Enumeration behandelt werden. Das nennt man *Pattern Matching*.

```rust
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
    // red green blue
}
```

Bei Enumerationen in Rust können den einzelnen Fällen einer Enumeration beliebige zusätzliche Daten zugewiesen
werden. Diese Werte können mit einem `match`-Ausdruck ebenfalls extrahiert werden.

Im folgenden Beispiel werden den Elementen der `Shape` Enumeration jeweils Struktur-Instanzen mit Geometriedaten 
zugewiesen. Diese Daten können im `match`-Ausdruck extrahiert werden.

```rust
#[derive(Clone)]
struct Point {
    pub x: f64,
    pub y: f64
}

struct Line {
    pub start: Point,
    pub end: Point
}

struct Circle {
    pub center: Point,
    pub radius: f64
}

enum Shape {
    LineShape(Line),
    CircleShape(Circle)
}

pub fn print_shapes() {
    let points = [
        Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 },
        Point { x: 1.0, y: 0.0 }, Point { x: 1.0, y: 1.0 },
    ];

    let shapes = [
        Shape::LineShape(Line {start:points[1].clone(), end: points[2].clone()}),
        Shape::CircleShape(Circle { center: points[0].clone(), radius: 1.0})
    ];

    for shape in shapes {
        match shape {
            Shape::LineShape(l) =>
                println!("line: from ({}, {}) to ({}, {})", l.start.x, l.start.y, l.end.x, l.end.y),
            Shape::CircleShape(c) =>
                println!("circle: at ({}, {}) radius {}", c.center.x, c.center.y, c.radius),
        }
    }
    
    // line: from (0, 1) to (1, 0)
    // circle: at (0, 0) radius 1
}
```

## Option

Der Standardtyp `Option<T>` ist ebenfalls ein Enumerationstyp. Er wird in der Standardbibliothek als generischer Typ 
wie folgt definiert. Bei der Verwendung kann ein beliebiger Typ für den Typparameter `T` verwendet werden.

```rust
enum Option<T> {
    Some(T),
    None
}
```

Ein `Option<T>`-Wert kann wie jeder andere Enumerationstyp mit einem `match`-Ausdruck ausgewertet werden.

```rust
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
// options: Hello, world!
```

Manchmal ist die Verwendung eines `match`-Ausdrucks etwas umständlich. Wenn beispielsweise `None` Werte einfach 
ignoriert werden können, dann kann man das auch mit einem `if let`-Ausdruck oder mit umgekehrter Logik mit 
`let else` schreiben.

```rust
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
// if let: 42

let mut sum = 0;
for option in options.iter() {
    let Some(n) = option else {
        continue;
    };

    sum += 2*n;
}

println!("let .. else: {}", sum);
// let .. else: 84
```

## Result

Der Ergebnistyp Result wird in der Standardbibliothek als generischer Enumerationstyp definiert.

```rust
enum Result<T,E> {
    Ok(T),
    Err(E)
}
```

Hier bestehen die gleichen Möglichkeiten den Fehler mittels *Pattern Matching* zu behandeln.

### Fehler weiterleiten

Mithilfe des `?`-Operators kann im Fehlerfall die Ausführung einer Funktion beendet werden. Dazu muss die Funktion 
einen `Result`-Typ mit dem gleichen Fehlertyp zurückgeben.

```rust
fn as_hex_string(input: &str) -> Result<String, ParseIntError> {
    let value:u32 = input.parse()?;

    Ok(format!("{:02x}", value))
}
```

Die Beispielfunktion `as_hex_string()` liefert einen Fehlerwert `Result<String,ParseIntError>::Err(_)` zurück, wenn 
die aufgerufene Funktion `parse()` einen Fehler des Typs `Result<u32,ParseIntError>:Err(_)` erzeugt. Ansonsten 
wird numerische Wert in eine Zeichenkette mit dem gleichwertigen Hexadezimalwert umgewandelt. Dieser Wert wird 
im Erfolgsfall als `Result<String,ParseIntError>::Ok(_)` zurückgegeben.

```rust
let inputs = vec!["10", "15", "-1", "16", "42"];

for input in inputs {
    match as_hex_string(input) {
        Ok(hex) => print!("{} ", hex),
        Err(_) => print!("{} ", input),
    }
}
println!();
// 0a 0f -1 10 2a
```
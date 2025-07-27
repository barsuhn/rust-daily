# Collections

In diesem Teil geht es um Collections in Rust.

## Tupel

Mit Tupeln können mehrere Elemente unterschiedlichen Typs zu einem Objekt zusammengefasst werden. Tupel haben immer
eine statische Größe, wobei jede Variation an der Größe und den Elementtypen einen anderen Typ definiert. Zur 
Definition von Tupeln werden die Elemente in runden Klammern angegeben. Auf die Elemente eines Tupels kann dann über 
Ihre Position im Tupel zugegriffen werden.

```rust
    let tuple = (1, true, "Rust".to_string());
let integer = tuple.0;
let boolean = tuple.1;
let string = &tuple.2;

println!("Tuple: {:?} -> {} {} {}", tuple, integer, boolean, string);
// Tuple: (1, true, "Rust") -> 1 true Rust
```

Eleganter wird der Zugriff auf Tupel aber, wenn Muster verwendet werden. Bei Mustern muss aber `ref` anstelle 
von `&` angegeben werden, wenn dem Musterausdruck eine Referenz zugewiesen werden soll. Falls eine veränderbare
Variable generiert werden soll, dann wird `mut` verwendet. Wenn der Typ bei einer Musterzuweisung nicht `Copy` 
ist, dann wird eine Move-Zuweisung ausgeführt, welche das Muster für die weitere Verwendung ungültig macht.

```rust
let mut tuple = (1, false, "Rust".to_string(), 21.0);
let (integer, ref mut boolean, ref string, mut number) = tuple;

*boolean = !*boolean;
number = 2.0 * number;

println!("Pattern -> {} {} {} {}", integer, boolean, string, number);
println!("Tuple {:?}", tuple);
// Pattern -> 1 true Rust 42
// Tuple (1, true, "Rust", 21.0)
```

Bei Musterzuweisungen können zudem der Platzhalter `_` und das Rest-Muster `..` verwendet werden. Das Rest-Muster 
darf dabei nur ein mal an einer beliebigen Position eingesetzt werden.

```rust
let numbers = (1, 2, 3, 4, 5);
let (one,_,_,four,_) = numbers;
let (first, .., last) = numbers;
let (.., end) = numbers;
let (start, ..) = numbers;

println!();
println!("Numbers: {:?} -> {} _ _ {} _", numbers, one, four);
println!("Mid rest: {:?} -> {} .. {}", numbers, first, last);
println!("Start rest: {:?} -> .. {}", numbers, end);
println!("End rest: {:?} -> {} ..", numbers, start);
```

Tupel gleichen Typs können mit verglichen werden. Dabei kann nicht nur Gleichheit, sondern auch die Ordnung von Tupeln 
bestimmt werden, falls die Elemente geordnet werden können. 

```rust
let mut tuples = [(2, "Ferris"), (2, "Bob"), (1, "Rust")];

println!();
println!("Tuples: {:?}", tuples);
println!("{:?} == {:?} -> {}", tuples[0], tuples[1], tuples[0] == tuples[1]);
println!("{:?} != {:?} -> {}", tuples[0], tuples[1], tuples[0] != tuples[1]);
println!("{:?} > {:?} -> {}", tuples[0], tuples[1], tuples[0] > tuples[1]);

tuples.sort();

println!("Sorted: {:?}", tuples);
```

### Das leere Tupel

Eine besondere Rolle bekommt in Rust der sogenannte *Unit-Typ*. Dieser wird durch das leere Tupel repräsentiert und 
stellt einen Typen dar, von dem es nur einen Wert gibt. Dieser Typ wird in Rust zur Repräsentation eines nicht
vorhandenen Werts verwendet. Wenn eine Funktion beispielsweise keinen Wert zurückgibt, wird dies mithilfe des 
Unit-Typs deklariert.
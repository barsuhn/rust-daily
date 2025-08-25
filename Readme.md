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

Eleganter wird der Zugriff auf Tupel, wenn Muster verwendet werden. Wenn man Referenzen auf die Tupelelemente 
benötigt, dann kann man dies jedoch nicht mithilfe von `&` angeben. Dies wäre dann Teil des Musters. Statt dessen
muss bei Mustern `ref` anstelle von `&` angegeben werden, wenn dem Musterausdruck eine Referenz zugewiesen 
werden soll. Falls eine veränderbare Variable generiert werden soll, dann wird `mut` verwendet. Wenn der 
Typ bei einer Musterzuweisung nicht `Copy` ist, dann wird eine Move-Zuweisung ausgeführt, welche das Muster 
für die weitere Verwendung ungültig macht.

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

Tupel gleichen Typs können miteinander verglichen werden. Dabei kann nicht nur Gleichheit, sondern auch die Ordnung 
von Tupeln bestimmt werden, falls die Elemente geordnet werden können. 

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

## Arrays

Arrays können Sequenzen von Werten aufnehmen. Die Länge der Sequenz gehört dabei zum Typ. Beispielsweise definiert
`[u32;10]` ein Array mit zehn Werten vom Typ `u32`. Mittels Typinferenz kann der Typ des Arrays bei der Zuweisung 
durch den Compiler bestimmt werden. Darüber hinaus können die Elemente eines Arrays wie bei den Tupeln auch mit 
Hilfe eines Musters zugewiesen werden. Ein leeres Arrray wird mit einem leeren Klammernpaar `[]` initialisiert.

```rust
let numbers = [1, 2, 3, 4];
let [_, two, _, four] = numbers.clone();

println!("{numbers:?} -> {} {two} {} {four}", numbers[0], numbers[2]);
// [1, 2, 3, 4] -> 1 2 3 4

let empty: [u8;0] = [];
println!("{empty:?}");
// []
```

## Bereiche

Mit Range-Typen können Sequenzen aufeinanderfolgender Elemente definiert werden. Beispielsweise kann eine Folge
ganzer Zahlen mit dem *halboffenen* Bereich `0..10` definiert werden. Halboffen bedeutet dabei, dass das Ende 
der Bereichsangabe nicht im Bereich enthalten ist. Der inklusive Bereich `'a'..='f'` enthält hingegen auch das 
letzte Element. Da `Range` und `RangeInclusive` den Trait `Iterator` implementieren, können alle 
Iterator-Funktionen auf einem Bereich ausgeführt werden. So können insbesondere auch for-Schleifen über 
Bereiche laufen. Die Iteration über einen Bereich verbraucht diesen Bereich. Wenn der Bereich noch benötigt 
wird, kann er mit `clone()` vervielfältigt werden.

```rust
let range = 0..10;

for i in range.clone() {
    print!("{i} ");
}
println!("= {range:?}");
// 0 1 2 3 4 5 6 7 8 9 = 1..10

let chars = 'a'..='f';
let vec = chars.collect::<Vec<char>>();
println!("{vec:?}");
// ['a', 'b', 'c', 'd', 'e', 'f']
```

Bereiche können sogar als Muster eingesetzt werden.

```rust
let number = 93;

println!();
print!("{number} is ");
match number {
    1 => println!("one"),
    2..=100 => println!("a lot"),
    _ => println!("even more")
}
// 93 is a lot
```

## Vektoren

Im Gegensatz zu Arrays werden Vektoren im Heap abgelegt und können während der Laufzeit wachsen und schrumpfen. Der 
Vektor `Vec<T>` ist ein generischer Typ, dessen Parameter `T` den Typ der Elemente festlegt. Man kann einen Vektor 
mit der Funktion `Vec<T>::new()` erzeugen. Eine komfortablere Möglichkeit bietet aber das Makro `vec![]`.

```rust
let empty: Vec<i32> = vec![];
let numbers = vec![1, 2, 3, 4, 5];
let guter_rat = vec!["üben"; 3];

println!();
println!("{empty:?}, {numbers:?}, {guter_rat:?}");
// [], [1, 2, 3, 4, 5], ["üben", "üben", "üben"]
```

Vektoren können mit Hilfe der Funktionen `push()` und `insert()` erweitert werden. Zum Entfernen von Elementen gibt
es die Funktionen `pop()` und `remove()`.

```rust
let mut numbers : Vec<i32> = vec![];

numbers.push(1);
numbers.push(2);

let top = numbers.pop();

numbers.push(3);
numbers.insert(1,4)
numbers.remove(0);

printls!();
println!("{numbers:?}, {top:?}");
// [4, 3], Some(2)
```

Ein Vektor kann auch um mehrere Elemente auf einmal erweitert werden. Die Funktion `append()` fügt die Elemente 
eines anderen Vektors an den Vektor an. Dabei werden diese Elemente aus dem anderen Vektor entnommen, sodass dieser
nach dem Funktionsaufruf leer ist. Die Funktion `extend()` fügt eine Sequenz in Form eines Iterators an den Vektor
an. Dabei werden Elemente, die nicht Copy sind bewegt, sodass die Quelle der Iteration ungültig wird. Weniger 
Destruktiv wirkt `extend_from_slice()`. Hier werden Elemente, die nicht Copy sind automatisch geklont.

```rust
let mut numbers = vec![1, 2];
let mut more_numbers = vec![3, 4];

numbers.append(&mut more_numbers);

println!();
println!("append: {numbers:?}, {more_numbers:?}");
// append: [1, 2, 3, 4], []

let mut numbers = vec![1, 2];
let more_numbers = vec![3, 4];

numbers.extend(more_numbers);

// println!("extend: {numbers:?}, {more_numbers:?}"); // Fehler!
println!("extend: {numbers:?}");
// extend: [1, 2, 3, 4]

let mut numbers = vec![1, 2];
let more_numbers = vec![3, 4];

numbers.extend_from_slice(&more_numbers);

println!("extend_from_slice: {numbers:?} {more_numbers:?}");
// extend_from_slice: [1, 2, 3, 4] [3, 4]
```

Mit der Funktion `splice()` können Elementbereiche eines Vektors durch eine andere Sequenz ersetzt werden.

```rust
let mut v = vec![1, 4, 27, 33, 11];

v.splice(1..1, [2,3]);
v.splice(4..6, [5,6,7,8,9,10]);
v.splice(11..11, [12,13,14]);

println!();
println!("{v:?}");
// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
```

## Slices

Ein Slice ist ein beliebiger Ausschnitt aus einem Array oder Vektor ohne eine vorgegebene Länge. Es handelt sich
dabei immer um eine Referenz mit dem Typ `&[T]`, wobei `T` ein Platzhalter für einen beliebigen Typ ist. Arrays 
und Vektoren können mithilfe der Funktion `.as_slice()` in einen Slice umgewandelt werden. Eine Referenz auf einen 
Array oder Vektor wird aber ebenfalls automatisch umgewandelt. 

```rust
let vec = vec![1, 2, 3];
let array = [4, 5, 6];
let vector_slice = vec.as_slice();
let array_slice = array.as_slice();

println!("vector as slice: {vector_slice:?}, array as slice: {array_slice:?}");
// vector as slice: [1, 2, 3], array as slice: [4, 5, 6]

let vector_slice: &[i32] = &vec;
let array_slice: &[_] = &array;

println!("&vector: {vector_slice:?}, &array: {array_slice:?}");
// &vector: [1, 2, 3], &array: [4, 5, 6]
```

Mit `as_mut_slice()` oder einer veränderbaren Referenz können veränderbare Slices erzeugt werden. Die Elemente der 
Slices können dann verändert werden. Es ist jedoch nicht möglich ein Vektor-Slice zu erweitern. Mit dem 
Index-Operator `[]` kann auf einzelne Elemente eines Slices, Vektors oder Arrays zugegriffen werden. Bei 
veränderbaren Variablen kann ein Index-Ausdruck auch auf der linken Seite der Zuweisung stehen. Im folgenden 
Beispiel wird die Tupel-Musterzuweisung verwendet, um mehrere Elemente gleichzeitig zuzuweisen.

```rust
let mut vec = vec![1, 2, 3];
let mut array = [4, 5, 6];
let slice = array.as_slice();

vec.push(slice[0]);

let vec_slice = vec.as_mut_slice();
let array_slice: &mut [_] = &mut array;

(vec_slice[0], array_slice[2]) = (array_slice[2], vec_slice[0]);

println!("vec_slice: {vec_slice:?}, array: {array:?}");
// vec_slice: [6, 2, 3, 4], array: [4, 5, 1]
```

Eine weitere Möglichkeit zur Erzeugung von Slices besteht darin, Bereiche aus einem Vektor oder Array mit dem 
Bereichsoperator zu extrahieren.

```rust
let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
let slice = &mut vec[2..5];

(slice[0], slice[2]) = (slice[2], slice[0]);

println!("vec: {vec:?}");
// vec: [1, 2, 5, 4, 3, 6, 7]
```

Mit der Funktion `.len()` kann die Anzahl der Elemente in einem Array, Vektor oder Slice bestimmt werden. Der 
Indexoperator erzeugt eine Panik, wenn der Index ungültig, also größer oder gleich der Länge ist. Die Funktion 
`get()` erlaubt einen Zugriff ohne die Gefahr einer Panik und liefert ein `Option<T>` als Ergebnis, sodass der 
Fehler vom aufrufenden Code behandelt werden kann. Der Typ `Option<T>` hat entweder einen 
Wert in der Form `Some(value)` oder keinen Wert `None`. Das folgende Beispiel verwendet Pattern Matching, um das
Ergebnis auszuwerten. Pattern Matching soll in einem anderen Abschnitt detaillierter betrachtet werden.

```rust
let vec = vec![1, 2, 3, 4, 5];
let slice = &vec[2..5];

println!("Slice has {} items.", slice.len());
// Slice has 3 items.
print!("slice get: ");
for i in 0..5 {
    match slice.get(i) {
        Some(value) => print!("{value} "),
        None => print!(". ")
    }
}
println!();
// slice get: 3 4 5 . .
```

Abschließend soll noch die Funktion `concat()` betrachtet werden. Sie *verflacht* die übergebenen Slices und fügt 
sie zu einem Ergebnisobjekt zusammen. Der Typ des Ergebnisobjekts ist dabei abhängig vom Elementtyp der Slices. 
Wenn ein Slice mit Strinng-Slices mittels `concat()` zusammengefügt wird, entsteht beispielsweise ein Ergebnis vom 
Typ `String`.

```rust
let parts = ["hello", ",", " ", "world", "!"];
let concat = parts.concat();

println!();
println!("concat result: {concat}");
// concat result: hello, world!
```

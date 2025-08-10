# Anweisungen und Ausdrücke

In diesem Abschnitt geht es um Anweisungen und Ausdrücke in Rust. Eine Anweisung hat dabei keinen Wert, der 
zugewiesen werden kann, wogegen Ausdrücke immer einen Rückgabewert haben. Im Vergleich zu anderen Sprachen 
fallen deutlich mehr Sprachkonstrukte in die Kategorie Ausdruck.

## Anweisungen

Zu den Anweisungen zählen in Rust Variablen-Deklarationen, Typ-Definitionen, Implementierungen und Funktionen.

### Deklarationen

Die Deklaration einer Variablen beginnt in Rust mit dem Schlüsselwort `let`. Die Deklaration kann mit oder ohne 
Initialisierung erfolgen. Variablen haben in Rust immer einen festen Typ. Man muss bei der Deklaration aber keinen
Typ angeben, wenn der Typ aus dem Programmkontext abgeleitet werden kann. Man nett das *Typ-Inferenz*.

Im folgenden Beispiel wird `x` ein ganzzahliger Wert zugewiesen. Der Standardtyp für Ganzzahlige Werte ist `i32`. 
Somit hat `x` den Typ `i32`. Bei der Variablen `y` wird erst bei der Zuweisung eines Wertes deutlich, welchen 
Typ `y` hat. Da die Zuweisung an `y` an zwei verschiedenen Codestellen erfolgt, müssen beide Zuweisungen den
gleichen Typ ergeben. Ansonsten würde der Compiler einen Fehler ausgeben. Da beide Zuweisungsanweisungen aber 
ebenfalls einen Ausdruck vom Typ `i32` verwenden, hat die Variable `y` ebenfalls den Typ `i32`.

```rust
let x = 5;
let y;

if x % 2 == 0 {
  y = x;
} else {
  y = 2 * x;
}

println!("The value of y is: {}", y);
```

### Typ-Deklarationen, Implementierungen und Funktionen

Typ-Deklarationen, `impl`-Blöcke und Funktionsdefinitionen sind ebenfalls Anweisungen. Sie werden hier nur 
beispielhaft gezeigt und in anderen Abschnitten näher betrachtet.

```rust
struct Value(i32);

impl Value {
    fn new(v: i32) -> Self {
        Value(v)
    }
}
```

## Ausdrücke

Wie in den meisten Sprachen sind algebraische Konstrukte wie beispielsweise `2 + 2` Ausdrücke mit einem Wert.
Der Ausdruck `2 + 2` hat erwartungsgemäß den Wert `4`. Auch der Aufruf einer Funktion (z.B. `sin(pi)`) ist - wie in den meisten 
anderen Sprachen - ein Ausdruck.

### Operatoren

Die Operatoren, die in Rust verwendet werden, findet man ebenfalls in den meisten Sprachen der `Algol`-Sprachfamilie.
Dazu gehören die algebraischen Operatoren:

- `%` - Rest der Ganzzahldivision: `a % b`
- `*` - Multiplikation: `a * b`
- `+` - Addition: `a + b`
- `-` - Subtraktion oder Negation: `a - b` oder `-a`
- `/` - Division: `a / b`

die logischen Operatoren

- `!` - Bitweises oder logisches *nicht* (Komplement): `!a`
- `&&` - Logisches *und*: `a && b`
- `||` - logisches *oder*: `a || b`

die binären Operatoren

- `&` - Bitweises *und*: `a & b`
- `|` - Bitweises *oder*: `a | b`
- `^` - Bitweises *exklusiv oder*: `a ^ b`
- `<<` - Bitverschiebung nach links: `a << b`
- `>>` - Bitverschiebung nach rechts: `a >> b`

und die Vergleichsoperatoren

- `<` - Kleiner als: `a < b`
- `>` - Größer als_ `a > b`
- `<=` - Kleiner oder gleich: `a <= b`
- `>=` - Größer oder gleich: `a >= b`
- `==` - Gleich: `a == b`
- `!=` - Ungleich: `a != b`

### Implizites `return`

### Bedingte Anweisungen

Die bedingte Anweisung mit `if` ist in Rust ein Ausdruck, der stets einen Wert hat. Falls der Ausdruck 
keinen expliziten Wert hat, dann ist der Wert das *leere Tupel* `()`. Das leere Tupel ist ein Spezialfall
des Tupeltyps und ist vergleichbar mit dem Typ `void` in vielen Sprachen der `Algol`-Familie.

Das folgende Beispiel zeigt einen `if` Ausdruck, der keinen expliziten Wert hat. Der Wert, den der 
`if/else`-Block hat ist in diesem Fall `()`, was in diesem Fall aber irrelevant wäre, da der Wert 
nicht verwendet wird.

```rust
let x = 5;

if x % 2 == 0 {
  println!("The value is even!");
} else {
  println!("The value is odd!");
}
```

Alternativ kann man einen `if` Ausdruck verwenden, der einen Wert zurückgibt. Im folgenden Beispiel wird dabei 
eine Besonderheit von Rust verwendet. Bei dem letzten Ausdruck eines Blocks kann das Semikolon am Ende der Zeile
weg gelassen werden. Das bedeutet, dass der Wert des Ausdrucks zurückgegeben wird. Der Rückgabewert einer Funktion
kann alternativ auch mit einer `return` Anweisung zurückgegeben werden. Für den `if`-Ausdruck ist das aber nicht
möglich, da ein `return` immer die umgebende Funktion verlassen würde.

```rust
let x = 6;
let kind = if x % 2 == 0 { "even" } else { "odd" };

println!("The kind of value is {kind}!");
```

Die Bedingung eines `if` Ausdrucks wird - wie man sieht - nicht in Klammern geschrieben. Dafür dürfen die 
geschweiften Klammern für die Blöcke aber nicht weg gelassen werden, auch wenn der Block nur aus einer einzelnen 
Anweisung besteht.

### Bedingte Zuweisungen und Pattern Matching

Die bedingten Zuweisungen `if let` und `let ... else`, sowie das Pattern Matching mit dem `match`-Ausdruck 
werden später behandelt.

### Schleifen

Es gibt neben den bekannten Schleifen auch die Endlosschleife `loop`.

#### Die Endlosschleife

Die Endlosschleife wird ohne Bedingung endlos ausgeführt. Sie kann lediglich mit einem `break` oder 
einem `return` Ausdruck beendet werden. Im folgenden Beispiel wird der Schleife beim Beenden mittels
`break i` ein Rückgabewert zugeordnet.

```rust
let mut i = 0;

let result = loop {
  if i >= 5 { break i; }
  print!("{i}, ");
  i = i + 1;
};

println!("{result}");
```

#### Die 'while' Schleife

Die `while` Schleife ist - wie in anderen Sprachen auch - eine *abweisende* Schleife mit einer Eintrittsbedingung. 
Die Schleife wird so lange ausgeführt, bis die Bedingung falsch wird. Wie bei `if` werden keine runden Klammern 
für die Bedingung verwendet.

```rust
let i = 1;

print!("{}", i-1);
while i <= 5 {
    print!(", {i}");
}

println!();
```

#### Die `for ... in` Schleife

Die `for ... in` Schleife verwendet einen sogenannten Iterator, der die Werte der *Laufvariablen* definiert. 
Die Schleife läuft so lange, bis das Ende des Iterators erreicht ist und weist der Laufvariablen bei jedem 
Durchlauf den aktuellen Wert des Iterators zu. Das folgende Besipiel verewendet als Iterator einen Bereichsausdruck 
der Form `a..b`. Dieser Ausdruck definiert einen Iterator, der von `a` bis zum größten Wert kleiner als `b` zählt. 
Der Wert `b` gehört also nicht zum Iterator. Wenn der letzte Wert des Bereichs erreicht werden soll, dann kann 
der inklusive Bereichsausdruck `a..=b` verwendet werden.

```rust
print!("1");
for i in 2..6 {
  print!(",{i}");
}
println!();
```
# Types

Rust verwendet ein sehr striktes statisches Typmodell. Umwandlungen, die in anderen Sprachen automatisch 
durchgeführt werden, müssen in Rust explizit ausgeführt werden.

## Ganzzahlige Typen

Rust unterscheidet wir andere Sprachen zwischen vorzeichenlosen und vorzeichenbehafteten Typen. Die vorzeichenlosen 
Ganzzahltypen sind

- `u8` - Eine vorzeichenlose ganze Zahl mit dem Wertebereich von 0 bis 255.
- `u16` - Eine vorzeichenlose ganze Zahl mit dem Wertebereich von 0 bis 65535.
- `u32` - Eine vorzeichenlose ganze Zahl mit dem Wertebereich von 0 bis 4294976295.
- `u64` - Eine vorzeichenlose ganze Zahl mit einem 64 Bit Wertebereich.
- `u128` - Eine vorzeichenlose ganze Zahl mit einem 128 Bit Wertebereich.
- `usize` - Eine vorzeichenlose ganze Zahl mit der Elemente in einem Bereich indiziert werden können. Der Wertebereich ist dabei Maschinenabhängig.

Die vorzeichenbehafteten Ganzzahltypen sind

- `i8` - Eine vorzeichenbehaftete ganze Zahl mit dem Wertebereich von -128 bis 127.
- `i16` - Eine vorzeichenbehaftete ganze Zahl mit dem Wertebereich von -32768 bis 32767.
- `i32` - Eine vorzeichenbehaftete ganze Zahl mit dem Wertebereich von -2147483648 bis 2147483647.
- `i64` - Eine vorzeichenbehaftete ganze Zahl mit einem 64 Bit Wertebereich.
- `i128` - Eine vorzeichenbehaftete ganze Zahl mit einem 128 Bit Wertebereich.
- `isize` - Eine vorzeichenbehaftete ganze Zahl, die von der Größe des addressierbaren Speichers des Maschinentyps abhängt.

Konstante Werte für ganzzahlige Datentypen werden als Ziffernfolge angegeben, wobei der Unterstrich `_` zur Trennung 
von Zifferngruppen verwendet werden kann. Beispielsweise zur Tausendertrennung bei Dezimalzahlen. Falls der Typ 
einer ganzen Zahl nicht durch Inferenz bestimmt werden kann, wird `i32` als Standardtyp verwendet. Der Typ einer 
Konstante kann aber auch explizit durch anhängen des Typnamens mit einem Unterstrich als Trennzeichen festgelegt.
Mit Hilfe der Präfixe `0x`, `0o` und `0b` können Zahlen auch zur Basis 16 (hexadezimal), 8 (oktal) oder 2 (binär)
angegeben werden. Beispiele:

- `-123` - Die vorzeichenbehaftete ganze Zahl `-123`.
- `123_456` - Die ganze Zahl `123456` mit Tausendertrenung.
- `123_u8` - Die ganze Zahl `123` vom Typ `u8`.
- `0xbeef` - Die hexadezimale Zahl `beef` (entspricht 48879 dezimal).
- `0b1001_0001_1000_1010u16` - Die binäre Zahl `1001000110001010` (entspricht dezimal 37258) vom Typ `u16`.

Eine Umwandlung zwischen ganzen Zahlen muss immer explizit angegeben werden. Wenn eine Umwandlung scheitern kann, 
weil der Wert zu groß oder zu klein für den Zieldatentyp ist, dann muss ggf. auch eine Fehlerbehandlung erfolgen.
Im folgenden Beispiel ist die zweite Umwandlung des Werts `256` in den Typ `u8` nicht möglich. Die Funktion `into()`
ist nur für sichere Umwandlungen verfügbar. Die unsichere Umwandlung erfolgt mit `try_into()` und liefert einen 
`Result` Typ zurück. Ob die Umwandlung erfolgreich war, kann mit den Funktionen `is_ok()` oder `is_err()` 
abgefragt werden. Zur Bestimmung des Wertes im Erfolgsfall verwendet man im allgemeinen Musterausdrücke. Diese 
werden in einem anderen Abschnitt beschrieben.

```rust
let a:u8 = 42;
let b:u16 = a.into();
let c:u16 = 256;
let d:Result<u8,_> = c.try_into();

print!("b = {b}");

if d.is_err() {
  println!(" but d has no value.");
} else {
  println!(" and d has a value");
}
```

## Gleitkommatypen

In Rust gibt es drei Typen für Gleitkommazahlen: `f32`, `f64` und `f128`. Gleitkommakonstanten müssen einen 
Dezimalpunkt enthalten. Ohne Dezimalpunkt handelt es sich um eine Konstante für eine ganze Zahl und diese müsste 
erst in ein Gleitkommazahl umgewandelt werden.

```rust
let a:f32 = 21.0 * 2.0;
let b:f64 = 42.into();

println!("a = {}, b = {}", a, b);
```

## Zeichenketten und Zeichen

Der Typ `&str` wird *String-Slice* genannt und repräsentiert eine konstante Zeichenkette. Man kann mit der Funktion
`.chars()` über die Zeichen der Zeichenkette iterieren. Zeichen sind vom Typ `char` und werden in Rust immer in 
Unicode kodiert. Deshalb kann auf einen sehr großen Zeichenvorrat zurückgegriffen werden.

```rust
let emotional_greet:&str = "Hi 😀!";

for c in emotional_greet.chars() {
  let character:char = c;
  
  print!("{character}");
}
println!();
```

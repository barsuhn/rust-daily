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
- `0b1001_0001_1000_1010_u16` - Die binäre Zahl `1001000110001010` (entspricht dezimal 37258) vom Typ `u16`.

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

In Rust gibt es zwei Typen für Gleitkommazahlen: `f32` und `f64`. Gleitkommakonstanten müssen einen 
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

## Logische Werte

Für logische Werte gibt es den Typ `bool`. Variablen vom Typ `bool` können den Wert `true` oder `false` haben.
Überall, wo eine Bedingung erwartet wird, kann auch ein `bool` Ausdruck verwendet werden.

```rust
let mut x:bool = false;

x = !x;

if x {
  println!("True!");
}
```

## Referenzen

Bei Referenzen handelt es sich nicht um einen eigenständigen Typ. Referenzen können auf beliebige Rust-Variablen mit
beliebigem Typ verweisen. Der Typ der Referenz auf eine Variable ist dann ein Referenztyp des Variablentyps. 
Referenztypen werden mit dem Referenzoperator `&` gekennzeichnet. Wenn eine Variable beispielsweise den Typ `f32` 
hat, dann hat eine Referenz auf die Variable den Typ `&f32`. Man kann auch eine Referenz auf eine Referenzvariable 
bilden. Der Typ ist dann eine mehrfache Referenz. Eine Referenz auf `&f32` hat also den Typ `&&f32`. Referenzen sind 
dabei analog zu normalen Variablen nicht veränderbar. Eine veränderbare Referenz wird mit `&mut` gekennzeichnet. Das 
setzt natürlich voraus, dass die referenzierte Variable ebenfalls veränderbar ist.

Eine Referenz auf eine Variable erhält man, indem man den Referenzoperator `&` oder `&mut` auf eine Variable anwendet.
Die Referenz kann danach stellvertretend für die Variable verwendet werden. Um auf den Wert der Variablen zuzugreifen,
muss man aber den Dereferenzierungsoperator `*` verwenden.

Im folgenden Beispiel wird eine unveränderbare Referenz `v` auf die Variable `u` definiert. Anschließend wird eine 
veränderbare Variable `x` definiert, die mit dem Wert der Variablen `u` initialisiert wird. Auf den Wert der Variablen 
`u` wird dabei mithilfe der Referenzvariablen `v` zugegriffen, indem diese mit dem `*`-Operator dereferenziert wird. 
Damit ist `x` zunächst eine Kopie der Variablen `u`. Da es sich um eine veränderbare Variable handelt, kann eine 
veränderbare Referenz durch den Ausdruck `&mut x` erzeugt und in der Variablen `y` abgelegt werden. Damit kann der 
Wert der Variablen `x` über die Referenz `y` verändert werden. Hier  ist zu beachten, dass `y` sowohl im Ausdruck 
`*y / 2` auf der rechten Seite der Zuweisung, als auch auf der linken Seite der Zuweisung dereferenziert werden muss.

Ohne die Dereferenzierung auf der linken Seite würde die Variable `y` verändert werden. Das geht aus zwei Gründen 
nicht. Zum einen, weil die Variable `y` selbst nicht veränderbar ist. Lediglich die Referenz ist veränderbar. Für 
eine veränderbare Variable hätte die Deklaration `let mut y:&mut u32` lauten müssen. Aber selbst dann könnte man 
der Variablen nur eine andere Referenz zuweisen. Der Ausdruck `*y / 2` ist aber ein Wert mit dem Typ `u32` und 
dieser kann nicht an eine Referenz zugewiesen werden.

```rust
let u:u32 = 84;
let v:&u32 = &u;
let mut x:u32 = *v;
let y:&mut u32 = &mut x;

*y = *y / 2;

println!("u: {} x: {}", u, x);
```

Die Verwendung einer Referenz bezeichnet man in Rust als *borgen* (*borrowing*) und für die Verwendung von Referenzen 
gibt es strenge Regeln:
- Es kann *beliebig viele* unveränderbare Referenzen geben. Während diese Referenzen verwendet werden, darf die Variable 
  selbst jedoch nicht verändert werden, auch wenn die Variable selbst veränderbar ist. Folglich dürfen in diesem
  Zeitraum auch keine veränderbaren Referenzen auf die Variable erzeugt werden.
- Es kann immer *nur eine* veränderbare Referenz gleichzeitig geben und während die veränderbare Referenz verwendet 
  wird, darf die Variable selbst nicht verändert werden. Zudem dürfen in diesem Zeitraum auch keine unveränderbaren 
  Referenzen auf die Variable gebildet werden.

Mit diesen Regeln wird sichergestellt, dass eine Variable zu jedem Zeitpunkt nur an einer einzigen Stelle verändert 
werden kann. Das vereinfacht insbesondere die Entwicklung von parallelen Programmen, bei denen mehrere CPU-Kerne 
gleichzeitig auf eine Variable zugreifen könnten. Das geht nur mit unveränderbaren Referenzen und damit ist dann auch 
sichergestellt, dass der Wert der Variablen nicht verändert werden kann. Man spricht deshalb bei unveränderbaren 
Referenzen auch von *geteilten Referenzen*.

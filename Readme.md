# Zeichenketten

Es gibt zwei Typen zur Repräsentation von Zeichenketten: `String` und das *String-Slice* `&str`.

## Slices

Eine Zeichenkettenkonstante hat in Rust den Typ `&str`. Es handelt sich dabei immer um eine unveränderbare Referenz, 
ohne dass eine Variable existiert, die referenziert wurde. Das wird dadurch ermöglicht, dass jedes ausführbare 
Binärprogramm üblicherweise einen Bereich für initialisierte Daten enthält. Dieser Bereich wird aus der Programmdatei 
direkt in den Speicher geladen. Zeichenkettenkonstanten werden bei der Erzeugung der Programmdatei in diesen 
Bereich geschrieben, sodass die Adressen der Zeichenketten bereits gültig sind, bevor die erste Maschineninstruktion
ausgeführt wird.

Da es sich um eine geteilte Referenz handelt, kann die Konstante an mehreren Stellen im Programm verwendet werden, 
muss jedoch nur ein mal im Speicher vorhanden sein. Intern wird das Slice durch einen Zeiger und eine Länge (in bytes) 
repräsentiert. Ein solches Konstrukt nennt man in Rust auch einen *fat pointer*. Im folgenden wird das Ausgabeformat 
`{:p}` verwendet, mit dem eine Referenz als *fat pointer* und ein Zeiger als Speicheradresse ausgegeben wird.

```rust
let greeting = "Hello, world!";

println!("slice: {}", greeting);
println!("reference: {:p}", greeting); // the reference is a fat pointer
println!("address: {:p} length: {}", greeting.as_ptr(), greeting.len());

// slice: Hello, world!
// reference: Pointer { addr: 0x100552e78, metadata: 13 }
// address: 0x100552e78 length: 13
```

### Iteration

Die Zeichenkette selbst ist eigentlich ein Array aus bytes. Also könnte eine Zeichenkette auch als Array-Slice 
`&[u8]` definiert werden. Der Typ `&str` stellt aber zusätzlich sicher, dass das Byte-Array ausschließlich 
gültige UTF8 Kodierungen enthält. Ein Zeichen in einer Zeichenkette kann in UTF8 mit mehreren Bytes kodiert 
werden, sodass es neben der physischen Repräsentation aus Bytes auch eine logische Repräsentation als Sequenz 
von Zeichen (char) gibt. 

Aus diesem Grund gibt es mehrere Iteratoren zur Aufzählung dieser Sequenzen. 
- `.bytes()` iteriert über die Bytes aus denen die Zeichenkette besteht.
- `.chars()` iteriert über die Unicode-Zeichen einer Zeichenkette.
- `.char_indices()` iteriert über die Byte-Indices der Unicode-Zeichen. Dabei ist jedes Element der Iteration ein 
                    Tupel bestehend aus dem dem Byte-Index und dem Unicode-Zeichen.

An der Ausgabe der `char_indices()` kann man beispielsweise erkennen, dass ASCII-Zeichen mit einem Byte codiert
werden. Der Buchstabe `ä` gehört aber nicht zu ASCII und wird mit zwei Bytes codiert.

```rust
let greeting = "Hello, Länd!";

println!("slice: {}", greeting); 
// slice: Hello, Länd!

print!("bytes: ");
for byte in greeting.bytes() {
    print!("{:x} ", byte);
}
println!();
// bytes: 48 65 6c 6c 6f 2c 20 4c c3 a4 6e 64 21

print!("chars: ");
for char in greeting.chars() {
    print!("{} ", char);
}
println!();
// chars: H e l l o ,   L ä n d !

print!("char indices: ");
for index in greeting.char_indices() {
    print!("{:?} ", index);
}
println!();
// char indices: (0, 'H') (1, 'e') (2, 'l') (3, 'l') (4, 'o') (5, ',') (6, ' ') (7, 'L') (8, 'ä') (10, 'n') (11, 'd') (12, '!')
```

### Werte parsen

Werte können aus String-Slices mittels der Funktion `parse()` gelesen werden. Der Zieltyp wird dabei entweder als
Typ-Parameter angegeben oder per Typ-Inferenz bestimmt. Intern verwendet die Funktion `parse()` den Trait `FromStr`,
um die Umwandlung durchzuführen. Folglich kann jeder Typ geparst werden, der `FromStr` implementiert.

Wenn der Zieltyp, der beim Parsen einer Zeichenkette entstehen soll, vom Compiler nicht automatisch bestimmt werden 
kann, muss man den Typ ggf. explizit angeben. Im Zweifelsfall schadet es nicht, den Typ immer explizit anzugeben. 
Um den Typ festzulegen gibt es zwei Möglichkeiten: durch anhängen des Typs an den Funktionsaufruf nach dem Muster 
`parse::<T>()`für den Zieltyp `T` oder durch explizite Angabe eines Typs in der Deklaration der Variablen, an die 
das Ergebnis zugewiesen wird.

Die `parse()` Funktion ein Ergebnis vom Typ `Result<T,E>` zurück, wobei `T` der Zieltyp und `E` ein Fehlertyp ist. 
Um den Wert aus dem Ergebnis auszulesen wird im folgenden Beispiel die Funktion `.unwrap()` verwendet. Diese Funktion 
löst eine *Panik* aus, die das Programm sofort beendet, wenn `parse()` einen Fehler zurückliefert. Diese Art der 
Fehlerbehandlung ist in den meisten Fällen unerwünscht. Wir werden uns später mit *Pattern Matching* und *Error 
Propagation* beschäftigen, die eine wesentlich elegantere Art der Fehlerbehandlung ermöglichen.

```rust
let number = "42.0".parse::<f64>().unwrap();
let integer: i32 = "42".parse().unwrap();

println!("slice parsing: {:?} -> {:?}", "42.0", number);
// slice parsing: "42.0" -> 42.0
println!("parsing with type inference: {:?} -> {:?}", "42", integer);
// parsing with type inference: "42" -> 42
```

### Slices auftrennen

Die Funktion `split()` trennt einen Slice mithilfe einer Teilzeichenkette in mehrere Teile auf. Der Rückgabewert der 
Funktion ist ein Iterator über die getrennten Teile. Mit der Funktion `trim()` werden führende und folgende Leerzeichen
aus der Zeichenkette entfernt. Mithilfe von `trim_matches()` können sogar verschiedene Zeichen am Anfang und am Ende der 
Zeichenkette entfernt werden. Da Iteratoren bei der Verwendung *verbraucht* werden kann man einen Iterator nicht 
mehrmals verwenden. Deshalb wird der von `split()` zurückgelieferte Iterator im folgenden Beispiel zwei mal mittels 
der Funktion `clone()` kopiert. Ohne Verwendung von `clone()` würde der Compiler in den darauf folgenden 
`for`-Schleifen einen Übersetzungsfehler ausgeben.

```rust
let colors = "{red, green , blue , yellow}";
let parts = colors.split(", ");

print!("Parts: ");
for part in parts.clone() {
  print!("'{part}' ");
}
println!();
// Parts: '{red' 'green ' 'blue ' 'yellow}'

print!("Trimmed: ");
for part in parts.clone() {
  let trimmed = part.trim();
  print!("'{trimmed}' ");
}
println!();
// Trimmed: '{red' 'green' 'blue' 'yellow}'

print!("Colors: ");
let trim_chars: &[char] = &['{', '}', ' '];
for part in parts {
  let trimmed = part.trim_matches(trim_chars);
  print!("'{trimmed}' ");
}
println!();
// Colors: 'red' 'green' 'blue' 'yellow'
```

### Binary String Literal

Mit einem binary string Literal kann ein u8-Array anstelle eines Slices definiert werden. Diese Literale dürfen 
aber nur ASCII-Zeichen enthalten. Der Typ von `greeting` im folgenden Beispiel ist `&[u8;13]`, also eine Referenz 
auf ein Array mit 13 Elementen vom Typ `u8`.

```rust
let greeting = b"Hello, world!";

println!("byte string: {:?}", greeting);
// byte string: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
```

### Raw Strings

Die Zeichen `\` und `"` haben in Zeichenketten eine besondere Bedeutung. Das Zeichen `"` würde offensichtlich als
das Ende der Zeichenkette interpretiert. Mit dem Zeichen `\` können darüber hinaus Sonderzeichen in einer 
Zeichenkette eingefügt werden. Man spricht bei einer Sequenz beginnend mit dem `\` auch von einer *Maskierung*
(*escaping*). Folgende Sonderzeichen können unter anderem verwendet werden:
- `\r` fügt ein *Zeilenrücklauf*-Zeichen (*carriage return*) ein.
- `\n` fügt ein *Zeilenvorschub*-Zeichen (*line feed*) ein.
- `\t` fügt ein *Tabulator*-Zeichen ein.
- `\\` fügt das Zeichen `\` ein.
- `\"` fügt das Zeichen `"` ein, ohne die Zeichenketet abzuschließen.
- 
Mit dem Präfix `r` werden sogenannte *raw string slices* definiert. In einem *raw* Literal müssen die Zeichen
`\` und `"` nicht *maskiert* werden. Zwischen `r` und dem einleitenden `"` können beliebig viele
`#` Zeichen hinzugefügt werden. Eine solche Zeichenkette muss mit einem `"` gefolgt von der gleichen 
Anzahl an `#` Zeichen beendet werden. Auf diese Weise können auch `"`, `"#`, `"##` innerhalb einer Zeichenkette
verwendet werden.

```rust
let path = r"C:\Users\Rust";
let quoted = r#"In "Anführungszeichen""#;
```

## Strings

Strings sind veränderbare Zeichenketten. Da der Speicherbedarf für einen solchen Wert zur Übersetzungszeit im 
Allgemeinen nicht bekannt ist, kann der Speicher für die Zeichenkette nicht auf dem *Stapelspeicher* reserviert 
werden. Dort können nur Elemente mit fester Speichergröße abgelegt werden. Stattdessen muss der Platz im sogenannten
*Heap*-Speicher reserviert werden. Die Reservierung von Heap-Speicher ist im Vergleich zur Verwendung des 
Stapelspeichers deutlich aufwändiger, sodass Variablen, die den Heap verwenden, bei Zuweisungen an andere Variablen 
oder bei der Übergabe an Funktionen nicht automatisch kopiert werden. Stattdessen werden die Werte bei Zuweisung 
oder Übergabe verbraucht und können anschließend nicht weiter verwendet werden. Wenn der Wert einer Variablen 
nicht verbraucht werden soll, kann das auf zwei verschiedene Weisen erreicht werden:
- Durch Verwendung von Referenzen, da Referenzen eine feste Größe haben. Eine Zeichenkettenreferenz vom Typ `&String` 
  wird bei der Zuweisung oder bei einem Funktionsaufruf sogar automatisch in Slice-Typ `&str` umgewandelt.
- Durch explizites Kopieren mittels der `clone()`-Funktion. 

```rust
let mut greeting = "Hello".to_string();

greeting.push_str(", ");
greeting = greeting + "world";
greeting.push('!');

println!();
println!("string: {}", greeting);
// string: Hello, world!
println!("rusty: {}", String::from("Let's get rusty!"));
// rusty: Let's get rusty!
```

### Werte formatieren

Zur Umwandlung zwischen Zahlen in Strings gibt es das `format!` Makro. Die Rückumwandlung mit der
Slice-Funktion `parse<T>(&self) -> Result<T,E>` haben wir bereits kennengelernt.

```rust
let number = 42;
let string = format!("{}", number);
let value: i32 = string.parse().unwrap();

println!("conversions: {:?} -> {:?} -> {:?}", number, string, value);
// conversions: 42 -> "42" -> 42
```

Die Makros zur Formatierung verwenden Paare von geschweiften Klammern `{}` als Platzhalter für einen Wert. Diese 
Platzhalter können auch weitere Angaben zur Formatierung enthalten. Es ist sogar möglich, die Variable, die formatiert 
werden soll, direkt im Klammernpaar anzugeben. Folgende Vorgaben zur Formatierung können zusätzlich angegeben werden:

- `:?` - Formatierung mithilfe des Debug-Traits.
- `:o` - Ganzzahligen Wert als Oktalzahl ausgeben.
- `:x` - Ganzzahligen Wert als Hexadezimalzahl mit Kleinbuchstaben ausgeben.
- `:X` - Ganzzahligen Wert als Hexadezimalzahl mit Großbuchstaben ausgeben.
- `:b` - Ganzzahligen Wert als Binärzahl ausgeben.
- `:p` - Wert als Speicheradresse (pointer) ausgeben.
- `:e` - Gleitkommazahl in wissenschaftlicher Notation mit kleinem "e" ausgeben.
- `:E` - Gleitkommazahl in wissenschaftlicher Notation mit großem "E" ausgeben.
- `:.{n}` - Gleitkommazahl mit `n` Nachkommastellen ausgeben.
- `:.{n}$` - Gleitkommazahl mit `k` Nachkommastellen ausgeben, wobei `k` der `n`-te Parameter des Makros ist.
- `:^{n}` - Zentriert in einem `n` Zeichen großen Bereich, der mit Leerzeichen gefüllt wird.
- `:<{n}` - Linksbündig in einem `n` Zeichen großen Bereich, der mit Leerzeichen gefüllt wird.
- `:>{n}` - Rechtsbündig in einem `n` Zeichen großen Bereich, der mit Leerzeichen gefüllt wird.
- `:{c}^{n}` - Zentriert in einem `n` Zeichen großen Bereich, der mit dem Zeichen `c` gefüllt wird.
- `:{c}<{n}` - Linksbündig in einem `n` Zeichen großen Bereich, der mit dem Zeichen `c` gefüllt wird.
- `:{c}>{n}` - Rechtsbündig in einem `n` Zeichen großen Bereich, der mit dem Zeichen `c` gefüllt wird.

### Teilzeichenketten ersetzen

Mithilfe der Funktion `replace` können Teilzeichenketten eines Slices durch andere Zeichenketten ersetzt werden.
Da die Größe des Ergebnisses zur Übersetzungszeit nicht bekannt ist, muss das Ergebnis als `String` erzeugt werden.

```rust
let greeting = "Hello, world!";
let replaced = greeting.replace("world", "Rust");

println!("Replaced: {} -> {}", greeting, replaced);
// Replaced: Hello, world! -> Hello, Rust!
```
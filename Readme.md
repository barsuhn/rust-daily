# Zeichenketten

Es gibt zwei Typen zur Repräsentation von Zeichenketten: `String` und das "Slice" `&str`.

## Slices

Ein Zeichenketten Literal hat in Rust den Typ `&str`. Es handelt sich dabei immer um eine unveränderbare Referenz. 
Intern wird das Slice durch einen Zeiger und eine Länge (in bytes) repräsentiert. Ein solches Konstrukt nennt man in Rust 
auch einen *fat pointer*.

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
von Zeichen (char) gibt. Aus diesem Grund gibt es mehrere Iteratoren zur Aufzählung dieser Sequenzen. Mit dem
Iterator `char_indices` kann zudem über die byte-Indices der Zeichen in einer Zeichenkette iteriert werden. Die
einzelnen Werte der Iteration sind dabei Tupel aus dem Index und dem Zeichen an diesem Index.

```rust
let greeting = "Hello, Länd!";

println!("slice: {}", greeting);

print!("bytes: ");
for byte in greeting.bytes() {
    print!("{:x} ", byte);
}
println!();

print!("chars: ");
for char in greeting.chars() {
    print!("{} ", char);
}
println!();

print!("char indices: ");
for index in greeting.char_indices() {
    print!("{:?} ", index);
}
println!();
```

### Werte parsen

Werte können aus String-Slices mittels der Funktion `parse` gelesen werden. Der Zieltyp wird dabei entweder als
Typ-Parameter angegeben oder per Typ-Inferenz bestimmt. Intern verwendet die Funktion `parse` den Trait `FromStr`,
um die Umwandlung durchzuführen. Folglich kann jeder Typ geparst werden, der `FromStr` implementiert.

```rust
let number = "42.0".parse::<f64>().unwrap();
let integer: i32 = "42".parse().unwrap();

println!();
println!("slice parsing: {:?} -> {:?}", "42.0", number);
println!("parsing with type inference: {:?} -> {:?}", "42", integer);
```

### Slices auftrennen

Die Funktion `split` trennt einen Slice mit Hilfe einer Teilzeichenkette in mehrere Teile auf. Der Rückgabewert der 
Funktion ist ein Iterator über die getrennten Teile.

```rust
let colors = "{red, green , blue , yellow}";
let parts = colors.split(", ");

print!("Parts: ");
for part in parts.clone() {
    print!("'{part}' ");
}
println!();

print!("Trimmed: ");
for trimmed in parts.clone().map(|p| p.trim()) {
    print!("'{trimmed}' ");
}
println!();

print!("Colors: ");
let trim_chars: &[char] = &['{', '}', ' '];
for trimmed in parts.map(|p| p.trim_matches(trim_chars)) {
    print!("'{trimmed}' ");
}
println!();
```

### Binary String Literal

Mit einem binary string Literal kann ein u8-Array anstelle eines Slices definiert werden. Diese Literale dürfen 
aber nur ASCII-Zeichen enthalten.


```rust
let greeting = b"Hello, world!";

println!("byte string: {:?}", greeting);
// byte string: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
```

### Raw Strings

Mit dem Präfix `r` werden sogenannte raw string slices definiert. In einem raw literal müssen die Zeichen
`\` und `"` nicht maskiert (engl. escape) werden. 

```rust
let path = r"C:\Users\Rust";
let quoted = r#"In "Anführungszeichen""#;
```

Um Sequenzen von `#` und `"` in einem Raw-String zu erlauben, können mehrere `#` Zeichen in der äußeren 
Klammerung verwendet werden.

## Strings

Strings sind veränderbare Zeichenketten. Der Speicher für diese Zeichenketten wird automatisch im Heap 
verwaltet. 

```rust
let mut greeting = "Hello".to_string();

greeting.push_str(", ");
greeting = greeting + "world";
greeting.push('!');

println!();
println!("string: {}", greeting);
println!("rusty: {}", String::from("Let's get rusty!"));
```

### Werte formatieren

Zur Umwandlung zwischen Zahlen in Strings gibt es das `format!` Makro. Die Rückumwandlung kann mit der
String-Funktion `parse<T>(&self) -> Result<T,E>` ausgeführt werden.

```rust
let number = 42;
let string = format!("{}", number);
let value: i32 = string.parse().unwrap();

println!("conversions: {:?} -> {:?} -> {:?}", number, string, value);
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
- `:^{n}` - Ausgabe zentriert in einem `n` Zeichen großen Bereich ausgeben, der mit Leerzeichen gefüllt wird.
- `:<{n}` - Ausgabe linksbündig in einem `n` Zeichen großen Bereich ausgeben, der mit dem Zeichen `c` gefüllt wird.
- `:>{n}` - Ausgabe rechtsbündig in einem `n` Zeichen großen Bereich ausgeben, der mit dem Zeichen `c` gefüllt wird.
- `:{c}^{n}` - Ausgabe zentriert in einem `n` Zeichen großen Bereich ausgeben, der mit dem Zeichen `c` gefüllt wird.
- `:{c}<{n}` - Ausgabe linksbündig in einem `n` Zeichen großen Bereich ausgeben, der mit dem Zeichen `c` gefüllt wird.
- `:{c}>{n}` - Ausgabe rechtsbündig in einem `n` Zeichen großen Bereich ausgeben, der mit dem Zeichen `c` gefüllt wird.

### Teilzeichenketten ersetzen

Mithilfe der Funktion `replace` können Teilzeichenketten eines Slices durch andere Zeichenketten ersetzt werden.
Da die Größe des Ergebnisses zur Übersetzungszeit nicht bekannt ist, muss das Ergebnis als `String` erzeugt werden.

```rust
let greeting = "Hello, world!";
let replaced = greeting.replace("world", "Rust");

println!("Replaced: {} -> {}", greeting, replaced);
// Replaced: Hello, world! -> Hello, Rust!
```
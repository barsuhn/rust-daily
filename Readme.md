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

### Binary String Literal
Die Zeichenkette selbst ist dabei eigentlich ein Array aus bytes: `&[u8]`. Der Typ `&str` stellt aber zusätzlich 
sicher, dass das Byte-Array ausschließlich gültige UTF8 Kodierungen enthält. Mit einem binary string Literal kann
aber ein Array anstelle eines Slices definiert werden. Diese Literale dürfen aber nur ASCII-Zeichen enthalten.

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
let string = "Hello, world!".to_string();
```

### Werte parsen

Zur Umwandlung zwischen Zahlen in Strings gibt es das `format!` Makro. Die Rückunwandlung kann mit der
String-Funktion `parse<T>(&self) -> Result<T,E>` ausgeführt werden.


```rust
let number = 42;
let string = format!("{}", number);
let value: i32 = string.parse().unwrap();
let slice_value = "42.0".parse::<f64>().unwrap();

println!("conversions: {:?} -> {:?} -> {:?}", number, string, value);
println!("slice parsing: {:?} -> {:?}", "42.0", slice_value);
```
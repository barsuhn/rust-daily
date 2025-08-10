# Funktionen 

## Globale Funktionen

Funktionen werden in Rust mit dem Schlüsselwort `fn` deklariert. Es folgt der Funktionsname, die Argumentliste 
und der Rückgabetyp. Der Rückgabetyp kann weg gelassen werden. In diesem Fall wird als Rückgabetyp implizit der 
*Unit-Typ* `()` verwendet. Dieser Typ bezeichnet das leere Tupel und kann nur einen Wert haben: `()`.

Die folgende Funktion `fib` berechnet die `n`-te Fibonacci-Zahl. Da die Ausdrücke in den `if/else` Zweigen kein 
Semikolon verwenden, hat der gesamte `if`-Block einen Wert und da der `if` Block ebenfalls kein Semikolon 
am Ende verwendet, ist dies auch der Rückgabewert der Funktion. Die Funktion erhält den Wert `n` als Argument.
Alternativ könnte man hier auch die Schreibweise `return fib(n-1) + fib(n-2);` verwenden.

Typangaben von Variablen werden in Rust immer mit einem Doppelpunkt getrennt hinter dem Variablennamen angegeben. 
Die Argumentvariable `n` hat folglich den Typ `u32`. Der Rückgabetyp der Funktion wird ebenfalls hinter der 
Funktionsdeklaration geschrieben und mit der Zeichenfolge `->` getrennt. 

```rust
fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
```

Bei einer Funktion, die keinen Wert zurückgibt, kann man auf die Angabe des Rückgabetyps `()` verzichten.

```rust
fn printfib(n: u32) {
    for i in 0..n {
        print!("{}, ", fib(i));
    }
    println!("{}", fib(n));
}
```
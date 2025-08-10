fn main() {
    printfib(10);
}

fn printfib(n: u32) {
    for i in 0..n {
        print!("{}, ", fib(i));
    }
    println!("{}", fib(n));
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}

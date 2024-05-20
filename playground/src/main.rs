use std::collections::btree_map::Range;

fn main() {
    let a = [-1.0; 10];
    let tup = ("Hello","World",a[0]);
    println!("{word1} {word2} {val}",
        word1 = tup.0, 
        word2 = tup.1, 
        val = tup.2
    );
    let a = 20;
    let a = iterate(a);
    println!("{a}");

    for x in (0..30) {
        println!("Fibonacci for {x}: {fib}",fib = fibonacci(x));
    }
    let f: f64 = 212.0;
    println!("Temp: {f}F, {c}C, {f2}F",c=fah_to_cel(f),f2=fah_to_cel(cel_to_fah(f)));
}

fn fibonacci(n: i32) -> i32 {
    return 
    if n == 0 {0} 
    else if n == 1 {1} 
    else {fibonacci(n-1) + fibonacci(n-2)};
}

fn cel_to_fah(cel: f64) -> f64 {
    return cel/100.0*(212.0-32.0) + 32.0;
}
fn fah_to_cel(fah: f64) -> f64 {
    return (fah - 32.0)/(212.0-32.0)*100.0;
}

fn iterate(x: i32) -> i32 {
    x + 1
}

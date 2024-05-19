fn main() {
    let a = [-1.0; 10];
    let tup = ("Hello","World",a[0]);
    println!("{word1} {word2} {val}",
        word1 = tup.0, 
        word2 = tup.1, 
        val = tup.2
    );
}

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    println!("{number:0>width$}", number=1, width=6);
    println!("{a}, {b} print!", a="Apple", b="Banana");
    println!("{:?}", (3,4));

    use std::io::Write;

    let mut w = Vec::new();
    writeln!(&mut w).unwrap();
    writeln!(&mut w, "test").unwrap();
    writeln!(&mut w, "formatted {}", "arguments").unwrap();

    assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());

    use std::io;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
}

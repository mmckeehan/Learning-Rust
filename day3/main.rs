fn greet(name: &str) {
    println!("Hello, {}! Welcome to Rust.", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon = this value is returned
}

fn main() {
    greet("M");
    let result = add(10,5);
    println!("10 + 5 = {}", result);
}
mod hello;

fn main() {
    // This will simply return the string "Hello World"
    let greeting = hello::greeting();

    // Show it to the world
    println!("{hello}", hello=greeting);
}


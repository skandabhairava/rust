use utils::input;

fn main() {
    println!("Test input: ");
    let guess = input("What is your name?").unwrap();
    println!("Hello {}", guess);
}
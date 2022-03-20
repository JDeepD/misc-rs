use std::io;

fn main() {
    let mut guess = String::new();
    println!("Guess a number: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

use std::io;

fn main(){
    println!("Guess the number");
    println!("enter yo guess:");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failure to read line!");

    println!("You guessed: {}", guess);
}
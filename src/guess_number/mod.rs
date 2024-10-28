use std::{cmp::Ordering, io};
use rand::Rng;

pub fn guess() {
  println!("Guess the number! ");

  let selected_number = rand::thread_rng().gen_range(1..=100);
  println!("the selected number is {selected_number}");

  loop {
    let mut guess = String::new();

    println!("Please input your guess.");

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line.");

    println!("U guessed: {guess}");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&selected_number) {
      Ordering::Less => println!("Less than secret~"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => { 
        println!("U WIN !");
        break;
      }
    }
  }
}
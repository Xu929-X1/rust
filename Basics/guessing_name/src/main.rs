use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*
    the latter defined guess shadows the previous one.
    The guess in the right side of the equality mark refers to the previously defined guess, which is a string!
    the :u32 after the guess var that is newly defined means we want to let RustC know that the parsed type should be a unsigned 32 bit integer.
    */
    loop {
        println!("Please Guess a Number!");
        let mut guess = String::new(); //rust vars are immutable by defaults
        io::stdin()
            .read_line(&mut guess) //The & symbol indicates that the guess var is a reference, &mut means this is a mutable reference
            //note that the Result here is a enum type, which only have: Ok(T) or Err(E)
            .expect("Fail to read line"); //when the state of the result is Err, expect function will be handling errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(reason) => {
                println!("{reason}");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number too small!"),
            Ordering::Greater => println!("Number too big!"),
            Ordering::Equal => {
                println!("Your Guess: {} Matches!", guess);
                break;
            }
        }
    }
}

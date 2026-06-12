//use std::io;
use rand::prelude::*;

fn main(){
    // score variables

    // play variables
    let mut to_guess: i32 = rand::rng().random_range(0..10);

    // testing
    println!("{}",to_guess);
}
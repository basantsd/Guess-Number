use colored::Colorize;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut rng = rand::thread_rng();
    let securetnum = rng.gen_range(0..100);
    

    loop {
        let mut guessnum = String::new();
        println!("{}","Please Enter Your Guess Number : ".blink());
        io::stdin()
            .read_line(&mut guessnum)
            .expect("Something Went Wrong");

        let guessnum: u32 = match guessnum.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guessnum.cmp(&securetnum) {
            Ordering::Less => println!("{}", "Too Small !".yellow()),
            Ordering::Greater => println!("{}", "Too High!".red()),
            Ordering::Equal => {
                println!("{}", "You Win !".green());
                break;
            }
        }
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Starting Simple Game of Gussesing the number between 1 and 100");
    let secrete_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut guess_times: i32 = 0; 

    loop {
        let mut guess_number: String = String::new();
        println!("Please enter a random number between 1 and 100:");
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Anything wrong happened!");

        let guess_number: i32 = match guess_number.trim().parse(){
            Ok(v) => {v},
            Err(_) => {continue;}
        };

        match guess_number.cmp(&secrete_number) {
            Ordering::Equal => {
                println!("You are right!You have try {guess_times} times!");
                break;
            },
            Ordering::Less => {
                guess_times = guess_times + 1;
                println!("{guess_number} is too small");
            },
            Ordering::Greater => {
                guess_times = guess_times + 1;
                println!("{guess_number} is too big");
            }
        }
    }
}

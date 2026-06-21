use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    //init varible
    let mut guess: i32;
    let target: i32 = rand::thread_rng().gen_range(1..=100);

    //print hint
    println!("guess your number(1-100)!");
    println!("whisper: the number is {}",target);
    
    guess = get_input_num();

    while cmp_and_rty(guess , target) {
        guess = get_input_num();
    }
}

fn get_input_num() -> i32 {
    let mut guess = String::new();
    println!("please input:");
    io::stdin()
        .read_line(&mut guess)
        .expect("failed!");
    let guess: i32 = guess.trim().parse().expect("please enter a number!");
    println!("you guessed {guess} !");

    guess    
}

fn cmp_and_rty(gue: i32 , tar: i32) -> bool {
    match gue.cmp(&tar) {
        Ordering::Less => { println!("less! zako~zako~");true },
        Ordering::Less => { println!("zako~zako~");true },
        Ordering::Greater => { println!("greater zako~zako~");true },
        Ordering::Equal => { println!("good job!");false },
    }
}
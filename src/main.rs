use std::time::Duration;
use tokio::time::sleep;
use std::io::Write;
use std::io;
use colored::*;

#[tokio::main]
async fn main() {
    print!("Miliseconds: ");
    io::stdout().flush().unwrap();

    let mut duration = String::new(); 

    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line.");

    let duration = duration.trim()
        .parse()
        .expect("Please input a number");

    for i in 0..3 {
        println!("{}", 3 - i);
        sleep(Duration::from_millis(250)).await;
    }

    const SIZE: usize = 4700;
    let block: [char; SIZE] = [''; SIZE];
    let block: String = block.iter().collect();

    println!("{}", block.red());
    sleep(Duration::from_millis(duration)).await;
    println!("{}", block.green());

}

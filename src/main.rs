extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
  let mut miner_addr = String::new();
  let mut difficulty = String::new();
  let mut choice = String::new();

  print!("Input a miner address: ");
  io::stdout().flush().expect("Failed to flush stdout");
  io::stdin().read_line(&mut miner_addr).expect("Failed to read from user");

  print!("Difficulty: ");
  io::stdout().flush().expect("Failed to flush stdout");
  io::stdin().read_line(&mut difficulty).expect("Failed to read from user");
  let diff = difficulty.trim().parse::<u32>().expect("We need an integer");
  println!("Generating genesis block");
  let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

  loop {
    println!("Menu:");
    println!("1: New transaction");
    println!("2: Mine block");
    println!("3: Change difficulty");
    println!("4: Change reward");
    println!("0: Exit");
    io::stdout().flush().expect("Failed to flush stdout");
    choice.clear();
    io::stdin().read_line(&mut choice).expect("Failed to read from user");

    match choice.trim().parse().unwrap() {
      0 => {
        println!("Exiting");
        process::exit(0);
      },
      1 => {
        let mut sender = String::new();
        let mut receiver = String::new();
        let mut amount = String::new();
        print!("Enter sender address: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut sender).expect("Failed to read from user");
        print!("Enter receiver address: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut receiver).expect("Failed to read from user");
        print!("Enter amount: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut amount).expect("Failed to read from user");

        let res = chain.new_transaction(
          sender.trim().to_string(),
          receiver.trim().to_string(),
          amount.trim().parse().unwrap());

        match res {
          true => println!("transaction added"),
          false => println!("transaction failed")
        }
      },
      2 => {
        println!("Generating block");
        let res = chain.generate_new_block();
        match res {
          true => println!("Block geenrated successfully"),
          false => println!("Block generation failed")
        }
      },
      3 => {
        let mut new_diff = String::new();
        print!("Enter new difficulty: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut new_diff).expect("Failed to read from user");
        let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
        match res {
          true => println!("Updated difficulty"),
          false => println!("Failed update")
        }
      },
      4 => {
        let mut new_reward = String::new();
        print!("Enter new reward: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut new_reward).expect("Failed to read from user");
        let res = chain.update_reward(new_reward.trim().parse().unwrap());
        match res {
          true => println!("Updated reward"),
          false => println!("Failed update")
        }
      },
      _ => {
        println!("Invalid choice please try again");
      }
    }
  }
}

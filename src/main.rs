mod sieve;
mod utils;
mod trial_division;
mod little_fermat;

use std::process::exit;
use crate::sieve::start_sieve;

use crate::little_fermat::start_little_fermat;

use crate::trial_division::start_trial_division;

use crate::utils::read_number;
use crate::utils::show_prompt;

fn main() {
    println!("Starting blazingly fast math solution for exercise sheet 4");


    show_prompt();

    let mut selected_program = String::new();

    while !selected_program.eq("-1") {

        println!("\n\nThe following options are available:");
        println!("{}",format!("{}{}{}","1. Execute the trial division for a given number\n", "2. Execute \
    the Sieve of Eratosthenes for a given number\n","3. Execute the little Fermat Prime test\n"));

        read_number(&mut selected_program);


        match selected_program.trim() {
            "1" => start_trial_division(),
            "2" => start_sieve(),
            "3" => start_little_fermat(),
            "-1"=> {
                println!("Exiting ...");
                exit(0)
            },
            _ => println!("This program is not registered"),
        }
        selected_program = String::new();
    }
    exit(0)
}









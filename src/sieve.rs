use std::collections::LinkedList;

use crate::utils::show_prompt;
use crate::utils::read_number;

pub fn start_sieve(){
    println!("Starting sieve of Eratosthenes");
    show_prompt();
    let mut input = String::new();

    read_number(&mut input);
    println!("Calculating sieve of eratostenes with number {}",input);

    let res =  input.trim().parse::<f64>();
    match res {
        Ok(val) => do_division(val),
        Err(why) => println!("Doesn't look like a number ({}). Exiting ... ", why),
    }
}



/**
 * This function calculates the prime numbers up to the given number
 */
fn do_division (res:f64) {
    let mut primes = vec![true; res as usize];
    let mut prime_nums = LinkedList::new();
    let limit:usize = res as usize;

    for prime in 2..limit {
        if !primes[prime]{
            continue;
        }
        for multiple in ((2 * prime)..limit).step_by(prime) {
            println!("Setting {} to false",multiple);
            primes[multiple] = false;
        }
    }

    let mut counter = 2;
    for i in 2..limit {
        println!("Number {} is {}",i, primes[i]);
        if primes[i]{
            prime_nums.push_back(counter);
        }
        counter += 1;
    }

    println!("Primes up to {} are: {:?}",res, prime_nums);
}
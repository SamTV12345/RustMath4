use std::collections::LinkedList;
use std::io;
use std::io::Write;

fn main() {
    println!("Starting blazingly fast math solution for exercise sheet 4");
    println!("The following options are available:");
    println!("{}",format!("{}{}{}","1. Execute the trial division for a given number\n", "2. Execute \
    the Sieve of Eratosthenes for a given number\n","3. Execute the little Fermat Prime test\n"));

    show_prompt();
    let mut selected_program = String::new();
    read_number(&mut selected_program);

    match selected_program.trim() {
        "1" => start_trial_division(),
        "2" => start_sieve(),
        "3" => start_little_fermat(),
        _ => println!("This program is not registered"),
    }
}


fn start_sieve(){
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

fn start_little_fermat(){

}


fn start_trial_division (){
    println!("Starting trial division with a given number");
    show_prompt();
    let mut input = String::new();
    read_number(&mut input);

    let number_for_trial_division = input.trim().parse::<i32>();

    match number_for_trial_division {
        Ok(val) => do_trial_division(val),
        Err(why) => println!("Doesn't look like a number ({}). Exiting ... ", why),
    }
}


fn read_number(mut input: &mut String) {
    io::stdin().read_line(&mut input).unwrap();
    while input.trim().len() == 0 {
        show_prompt();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn do_trial_division(number_to_test: i32){
    let mut copy_of_number = number_to_test;
    let mut divider:LinkedList<i32> = LinkedList::new();
    let mut starting_divider = 2;

    while copy_of_number % 2 == 0{
        divider.push_back(2);
        copy_of_number/=2;
    }
    starting_divider = 3;

    while starting_divider <= copy_of_number{
        if copy_of_number % starting_divider == 0{
            divider.push_back(starting_divider);
            copy_of_number/=starting_divider;
        }else{
            starting_divider+=2;
        }
    }

    if copy_of_number!=1{
        divider.push_back(copy_of_number);
    }

    println!("The prime factors of {} are:",number_to_test);
    divider.iter()
           .for_each(|x| print!("{} ",x));
}

fn show_prompt() {
    print!("Please enter a number: ");
    io::stdout().flush().unwrap();
}


fn do_fermat(number_to_test: i32){

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

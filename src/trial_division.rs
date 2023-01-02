use std::collections::LinkedList;
use crate::utils::show_prompt;
use crate::utils::read_number;

pub fn start_trial_division (){
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


fn do_trial_division(number_to_test: i32){
    let mut copy_of_number = number_to_test;
    let mut divider:LinkedList<i32> = LinkedList::new();
    let  mut starting_divider = 2;

    println!("Starting trial division with divider {}",starting_divider);

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
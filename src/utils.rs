use std::io;
use std::io::Write;

pub fn show_prompt() {
    print!("Please enter a number: ");
    io::stdout().flush().unwrap();
}

pub fn read_number(mut input: &mut String) {
    io::stdin().read_line(&mut input).unwrap();
    while input.trim().len() == 0 {
        show_prompt();
        io::stdin().read_line(&mut input).unwrap();
    }
}
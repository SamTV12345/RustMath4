use read_number;
use utils::show_prompt;

static CARMICHAEL_NUMS: [i32; 33] = [561, 1105, 1729, 2465, 2821, 6601, 8911, 10585, 15841,
    29341,
    41041, 46657, 52633, 62745, 63973, 75361, 101101, 115921, 126217, 162401, 172081, 188461, 252601, 278545, 294409, 314821, 334153, 340561, 399001, 410041, 449065, 488881, 512461];

fn calc_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return calc_gcd(b, a % b);
}


fn modular_exponentiation(base: i32, exponent: i32, modulus: i32) -> i32 {
    if modulus == 1 {
        return 0;
    }
    let mut c = 1;
    for _ in 0..exponent {
        c = (c * base) % modulus;
    }
    return c;
}

fn little_fermat_test(number: i32) -> bool {
    if CARMICHAEL_NUMS.contains(&number) {
        return false;
    }

    let mut a = 2;
    while a < number {
        if calc_gcd(a, number) != 1 {
            a += 1;
            continue;
        }
        if modular_exponentiation(a, number - 1, number) != 1 {
            return false;
        }
        a += 1;
    }
    return true;
}

pub fn start_little_fermat(){
    let mut number = String::new();
    println!("Starting little Fermat test with a given number");
    show_prompt();
    read_number(&mut number);

    let number_for_little_fermat = number.trim().parse::<i32>();

    match number_for_little_fermat {
        Ok(val) => {
            let test = little_fermat_test(val);
            if test {
                println!("{} is a prime number", val);
            } else {
                println!("{} is not a prime number", val);
            }
        },
        Err(why) => println!("Doesn't look like a number ({}). Exiting ... ", why),
    }
}
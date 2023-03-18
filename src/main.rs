use num_bigint::BigUint;
use num_traits::One;
use text_io::read;
use std::env;

/// Calculate large factorial numbers.
/// 
/// # Arguments
/// 
/// * `n` - The number to calculate the factorial of.
/// 
/// # Examples
/// 
/// ```
/// fact(BigUint::from(i))
/// ```
fn fact(n: BigUint) -> BigUint {
    let mut f = BigUint::one();
    let mut i = BigUint::one();
    while i <= n {
        f *= &i;
        i += BigUint::one();
    }
    f
}

/// Read a number from the user.
/// 
/// # Examples
/// 
/// ```
/// interactive_mode();
/// ```
fn interactive_mode() {
    print!("Enter a number: ");
    let input: String = read!();
    parse_args(input);
}

/// Parse the arguments given to the program.
/// 
/// # Arguments
/// 
/// * `input` - The argument to parse.
/// 
/// # Examples
/// 
/// ```
/// parse_args("5".to_string());
/// ```
fn parse_args(input: String) {
    if input == "help" || input == "-h" || input == "--help" {
        println!("Usage: factorial [number]");
        return;
    } else if input == "interactive" || input == "-i" || input == "--interactive" {
        interactive_mode();
        return;
    } else if input == "exit" || input == "-e" || input == "--exit" {
        println!("Exiting...");
        return;
    } else if input == "quit" || input == "-q" || input == "--quit" {
        println!("Exiting...");
        return;
    } else if input.parse::<usize>().is_ok() {
        println!("{}! = {}", input, fact(BigUint::from(input.parse::<usize>().unwrap())));
        return;
    }
    println!("Invalid argument, try again.");
}

fn main() {
    // Read a number from the user.
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let arg = &args[1];
        parse_args(arg.to_string());
        return;
    } else {
        println!("No argument given, using interactive mode.");
        interactive_mode();
    }
}

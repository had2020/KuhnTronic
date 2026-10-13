use crate::nnet::*;
use input_method::input;
use std::env;

mod nnet;

fn main() {
    if env::args().len() > 1 {
        let arg0: String = env::args().nth(1).unwrap();
        if arg0.eq_ignore_ascii_case("Debug") {
            println!("Raw debugging mode enabled.")
        } else if arg0.eq_ignore_ascii_case("Train") {
            println!("Training process begin.");
        } else {
            println!(
                "Improper CLI argument! \n{}{}",
                "\nYou can also re-run with these commands:",
                "\n\t'Debug' for detached debugger, or 'Train' for detached trainner"
            )
        }
    } else {
        println!(
            "The Rust interface is running, press control+c to kill it.\n{}{}",
            "\nYou can also re-run with these commands:",
            "\n\t'Debug' for detached debugger, or 'Train' for detached trainner"
        );
    }
}

use crate::nnet::*;
use input_method::input;
use std::env;

mod nnet;

fn main() {
    let mut model: Model = Model::new();

    if env::args().len() > 1 {
        let arg0: String = env::args().nth(1).expect("Failed to parse CLI args!");

        if arg0.eq_ignore_ascii_case("Debug") {
            println!("Raw debugging mode enabled.");
        } else if arg0.eq_ignore_ascii_case("Train") {
            println!("Training process begin.");
        } else {
            println!(
                "IMPROPPER CLI ARGUMENT! \n{}{}",
                "\nYou can also re-run with these commands:",
                "\n\t'Debug' for detached debugger, or 'Train' for detached trainner"
            );
        }
    } else {
        if std::fs::exists("model/model.json").unwrap() {
            println!("Loading model file!");
            model.load_model_json("model/model.json");

            println!(
                "The Rust interface is running, press control+c to kill it.\n{}{}",
                "\nYou can also re-run with these commands:",
                "\n\t'Debug' for detached debugger, or 'Train' for detached trainner"
            );
        } else {
            println!(
                "No model file found!\nCheck if you have model file on you filesystem, as {}",
                "'model/model.json'\nOr re-run with command 'Train' for a detached trainner."
            );
        }
    }
}

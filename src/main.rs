use crate::nnet::*;
use input_method::input;
use rusqlite::{Connection, Result};
use std::env;

mod nnet;

fn main() {
    let mut model: Model = Model::new();

    if env::args().len() > 1 {
        let arg0: String = env::args().nth(1).expect("Failed to parse CLI args!");

        if arg0.eq_ignore_ascii_case("Debug") {
            println!("Raw debugging mode enabled.");
            //TODO parsing input into model input vector.
        } else if arg0.eq_ignore_ascii_case("Train") {
            println!("Training process started.");

            let conn = Connection::open("training_data.db");
            //TODO read docs: https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html#method.open
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

            //TODO possible run a Rust Rocket HTTP server with POST and GET
        } else {
            println!(
                "No model file found!\nCheck if you have model file on you filesystem, as {}",
                "'model/model.json'\n\tOr re-run with command 'Train' for a detached trainner."
            );
        }
    }
}

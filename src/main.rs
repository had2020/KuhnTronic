use crate::nnet::*;
use input_method::input;
use rusqlite::{Connection, Params, Result};
use std::env;

mod nnet;

pub enum GameState {
    // -> next best move
    KEmpty = 0,    // -> KCheck
    KCheck = 1,    // -> KCheckBet
    KBet = 2,      // -> KBetCall
    KCheckBet = 3, // -> KCheckBetCall
    // End game states
    KCheckCheck = 4,
    KCheckBetFold = 5,
    KCheckBetCall = 6,
    KBetFold = 7,
    KBetCall = 8,

    QEmpty = 9,     // -> QCheck
    QCheck = 10,    // -> QCheckCheck
    QBet = 11,      // -> QBetFold
    QCheckBet = 12, // -> QCheckBetFold
    // End game states
    QCheckCheck = 13,
    QCheckBetFold = 14,
    QCheckBetCall = 15,
    QBetFold = 16,
    QBetCall = 17,

    JEmpty = 18,    // -> JBet
    JCheck = 19,    // -> JCheckBet
    JBet = 20,      // -> JBetFold
    JCheckBet = 21, // -> JCheckBetFold
    // End game states
    JCheckCheck = 22,
    JCheckBetFold = 23,
    JCheckBetCall = 24,
    JBetFold = 25,
    JBetCall = 26,

    NoAction = 27,
}

/*
let next_action_tree = [
    GameState::KCheck,
    GameState::KCheckBet,
    GameState::KBetCall,
    GameState::KCheckBetCall,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::QCheck,
    GameState::QCheckCheck,
    GameState::QBetFold,
    GameState::QCheckBetFold,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::JBet,
    GameState::JCheckBet,
    GameState::JBetFold,
    GameState::JCheckBetFold,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
    GameState::NoAction,
];*/

fn main() {
    let mut model: Model = Model::new();

    if env::args().len() > 1 {
        let arg0: String = env::args().nth(1).expect("Failed to parse CLI args!");

        if arg0.eq_ignore_ascii_case("Debug") {
            println!("Raw debugging mode enabled.");
            //TODO parsing input into model input vector.
        } else if arg0.eq_ignore_ascii_case("Train") {
            println!("Training process started.");
            /*
            let db: Connection =
                Connection::open("kuhn_training_data.db").expect("Could not open database object!");
            */
            /*
            db.execute(
                "CREATE TABLE IF NOT EXISTS dataset (
                    id INTEGER PRIMARY KEY,
                    input0 DOUBLE NOT NULL,
                    input1 DOUBLE NOT NULL,
                    input2 DOUBLE NOT NULL,
                    input3 DOUBLE NOT NULL,
                    input4 DOUBLE NOT NULL,
                    input5 DOUBLE NOT NULL,
                    output DOUBLE NOT NULL,
                )",
                (),
            )
            .expect("SQL failed to execute!"); */
            //TODO read docs: https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html#method.open

            /* Model layout:
            First 3 inputs classify the AI's card:
                [1.0, 0.0, 0.0] -> King, [0.0, 1.0, 0.0] -> Queen, [0.0, 0.0, 1.0] -> joker.
            Last 2 inputs classify the last action sequence in the game:
                Action types: {Bet: 1.0, Pass: -1.0, NULL: 0.0}
                bets: Call, Bet
                passes: Fold, Check
            */

            const N_STATES: usize = 27;

            let mut dataset = DataSet {
                inputs: vec![[0.0; 5]; N_STATES],
                outputs: vec![[0.0]; N_STATES],
            };

            // starter card for AI
            let mut last_idx = 0;
            for i in 0..N_STATES {
                dataset.inputs[i][last_idx] = 1.0;
                if last_idx != 3 {
                    last_idx += 1;
                } else {
                    last_idx = 0;
                }

                //?Empty
                if (i == 0) || (i == 9) || (i == 18) {
                    dataset.inputs[i][3] = -1.0;
                    match last_idx {
                        // k
                        0 => {
                            dataset.outputs[i][0] = 1.0;
                        }
                        // q
                        1 => {
                            dataset.outputs[i][0] = 1.0;
                        }
                        // j
                        _ => {
                            dataset.outputs[i][0] = 1.0;
                        }
                    }
                }

                //?Check
                if (i == 1) || (i == 10) || (i == 19) {
                    dataset.inputs[i][3] = -1.0;
                    match last_idx {
                        // k
                        0 => {
                            dataset.outputs[i][0] = 1.0;
                        }
                        // q
                        1 => {
                            dataset.outputs[i][0] = -1.0;
                        }
                        // j
                        _ => {
                            dataset.outputs[i][0] = 1.0;
                        }
                    }
                }

                //?Bet
                if (i == 2) || (i == 11) || (i == 20) {
                    dataset.inputs[i][3] = 1.0;
                    match last_idx {
                        // k
                        0 => {
                            dataset.outputs[i][0] = 1.0;
                        }
                        // q
                        1 => {
                            dataset.outputs[i][0] = -1.0;
                        }
                        // j
                        _ => {
                            dataset.outputs[i][0] = -1.0;
                        }
                    }
                }

                //?CheckBet
                if (i == 3) == (i == 12) || (i == 21) {
                    dataset.inputs[i][3] = -1.0;
                    dataset.inputs[i][4] = 1.0;
                    match last_idx {
                        // k
                        0 => {
                            dataset.outputs[i][0] = 1.0;
                        }
                        // q
                        1 => {
                            dataset.outputs[i][0] = -1.0;
                        }
                        // j
                        _ => {
                            dataset.outputs[i][0] = -1.0;
                        }
                    }
                }
            }

            model.train(dataset, 100000);
            let mut output: f64 = 0.0;
            {
                let mut model_clone: Model = model.clone();
                model_clone.forward_pass([1.0, 0.0, 0.0, -1.0, 1.0]); // should be pos
                //model_clone.forward_pass([0.0, 0.0, 1.0, -1.0, 1.0]); // should be neg
                output = model_clone.output_layer[0];
            }
            println!("{output}");
            //
        } else if (arg0.eq_ignore_ascii_case("Generate")) {
            println!("Using Simulator to generate data points.");
            /*
            let db: Connection =
                Connection::open("kuhn_training_data.db").expect("Could not open database object!");

            db.execute(
                "CREATE TABLE IF NOT EXISTS dataset (
                    id INTEGER PRIMARY KEY,
                    input0 DOUBLE NOT NULL,
                    input1 DOUBLE NOT NULL,
                    input2 DOUBLE NOT NULL,
                    input3 DOUBLE NOT NULL,
                    input4 DOUBLE NOT NULL,
                    input5 DOUBLE NOT NULL,
                    output DOUBLE NOT NULL
                )",
                (),
            )
            .expect("SQL failed to execute, createing table if not exists!");
            */

            // the max number of finite states
            //const OWN_CARD_FS: u8 = 3;

            //TODO in future verison with database
            //for i in OWN_CARD_FS..0 {}
        } else {
            println!(
                "IMPROPPER CLI ARGUMENT! \n{}{}{}",
                "\nYou can also re-run with these commands:",
                "\n\t'Debug' for detached debugger, or 'Train' for detached trainner",
                "\n\t'Generate' for a detached simulator to generate data."
            );
        }
    } else {
        if std::fs::exists("model/model.json").unwrap() {
            println!("Loading model file!");
            model.load_model_json("model/model.json");

            println!(
                "The Rust interface is running, press control+c to kill it.\n{}{}{}",
                "\nYou can also re-run with these commands:",
                "\n\t'Debug' for detached debugger, or 'Train' for detached trainner",
                "\n\t'Generate' for a detached simulator to generate data."
            );

            //TODO possible run a Rust Rocket HTTP server with POST and GET
        } else {
            println!(
                "No model file found!\nCheck if you have model file on you filesystem, as {}{}",
                "'model/model.json'\n\tOr re-run with command 'Train' for a detached trainner.",
                "\n\t'Generate' for a detached simulator to generate data."
            );
        }
    }
}

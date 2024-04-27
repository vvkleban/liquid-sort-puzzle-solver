#![allow(non_snake_case)]
mod bottle;
mod bfs;
mod astar;
mod traits;

use std::io::{self, BufRead};
use std::process;
use std::rc::Rc;
use clap::{Arg, ArgAction, Command};
use bottle::*;
use bfs::position_bfs::*;
use astar::position_astar::*;
use bfs::bfs::*;
use astar::astar::*;
use traits::position::*;

fn printSolution(possibleSolution: Option<Vec<Rc<dyn Position>>>) {
    if let Some(solution) = possibleSolution {
        let mut oldPosition: Option<Rc<dyn Position>>= None;
        for i in 0..solution.len() {
            println!("Step {}\n{}", i, solution[i].toString(&oldPosition).unwrap());
            oldPosition= Some(solution[i].clone());
        }
    } else {
        println!("No solution was found");
    }
}

/// Reads and processes input data from standard input to initialize and execute a BFS (Breadth-First Search)
/// based solution finding process for a liquid sort puzzle
///
/// # Operation
/// - Reads lines of input where each line represents a bottle configuration.
/// - Each line should exactly contain four ASCII characters representing the content of the bottle.
/// - Lines starting with '#' are considered as comments and are ignored.
/// - Lines that are entirely whitespace are also ignored.
/// - Validates that all characters in each line are ASCII and each line has exactly four characters.
///
/// # Returns
/// - `Ok(())` if the input is successfully read, validated, and processed to find a solution.
/// - `Err(())` if there are any input errors like non-ASCII characters, incorrect line lengths,
///   or if any other runtime error occurs during processing.
///
/// # Errors
/// - Returns `Err(())` and prints an error message if any line contains non-ASCII characters.
/// - Returns `Err(())` and prints an error message if any line does not contain exactly four characters.
///
/// # Example Usage
/// This function is intended to be called at the start of the program to handle the setup of game state
/// from standard input and to kick off the solution process.
///
/// ```
/// fn main() {
///     if let Err(_) = handleInputData() {
///         eprintln!("Failed to process input data.");
///     }
/// }
/// ```
fn handleInputData() -> Result<Vec<Bottle>, String> {
    // Prepare to read lines from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // This vector will hold all the vectors of chars
    let mut data: Vec<Bottle> = Vec::new();

    // Read each line from standard input
    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        
        // Skip empty or whitespace-only lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if !line.is_ascii() {
            return Err("Error: All characters must be ASCII".to_string());
        }

        // Check if the line contains exactly four characters
        if line.len() == 4 {
            let array: [u8; 4] = line.as_bytes().to_vec().try_into().unwrap();
            data.push(Bottle::new(array));
        } else {
            return Err("Error: Each line must contain exactly four characters.".to_string());
        }
    }
    Ok(data)
} 

fn main() {
    let matches = Command::new("Bottle Sort Puzzle Solver")
    .version("1.0")
    .author("Volodymyr Kleban")
    .about("Uses an algorithm based on command line arguments to sort liquids in bottles")
    .override_usage("liquid_sort_solver [OPTIONS] <puzzle data")
    .arg(Arg::new("bfs")
         .long("bfs")
         .action(ArgAction::SetTrue)
         .help("Use the BFS algorithm"))
    .arg(Arg::new("astar")
         .long("astar")
         .action(ArgAction::SetTrue)
         .help("Use the A* algorithm (default)"))
    .get_matches();

    match handleInputData() {
        Ok(data) => {
            if matches.get_flag("bfs") {
                let position= PositionBFS::new(data, 0);
                if let Some(error) = position.isValid().err() {
                    eprintln!("{}", error);
                    process::exit(1);
                }
                let mut bfs= BFS::new(position);
                printSolution(bfs.solve()); 
            } else {
                let position= PositionAstar::new(data);
                if let Some(error) = position.isValid().err() {
                    eprintln!("{}", error);
                    process::exit(1);
                }
                let mut astar= Astar::new(position);
                printSolution(astar.solve());
            }
        },
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

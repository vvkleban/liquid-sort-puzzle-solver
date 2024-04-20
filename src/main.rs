#![allow(non_snake_case)]
mod bottle;
mod position;
mod bfs;

use std::io::{self, BufRead};
use std::process;
use bottle::*;
use position::*;
use bfs::*;

fn printSolution(possibleSolution: Option<Vec<Position>>) {
    if let Some(solution) = possibleSolution {
        let mut oldPosition: Option<&Position>= None;
        for i in 0..solution.len() {
            println!("{}", solution[i].toString(oldPosition).unwrap());
            oldPosition= Some(&solution[i]);
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
fn handleInputData() -> Result<(), ()> {
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
            println!("Error: All characters must be ASCII");
            return Err(());
        }

        // Check if the line contains exactly four characters
        if line.len() == 4 {
            let array: [u8; 4] = line.as_bytes().to_vec().try_into().unwrap();
            data.push(Bottle::new(array));
        } else {
            println!("Error: Each line must contain exactly four characters.");
            return Err(());
        }
    }
    let position= Position::new(data, 0);
    if let Some(error) = position.isValid().err() {
        println!("{}", error);
        return Err(());
    }
    let mut bfs= BFS::new(position);
    printSolution(bfs.solve());
    Ok(())
} 

fn main() {
    handleInputData().unwrap_or_else(|()| process::exit(1));
}

#![allow(non_snake_case)]
mod bottle;
mod bfs;
mod astar;
mod traits;

use std::io;
use std::io::Read;
use std::fmt::Write;
use std::process;
use std::rc::Rc;
use clap::{Arg, ArgAction, Command};
use bottle::*;
use bfs::position_bfs::*;
use astar::position_astar::*;
use bfs::bfs::*;
use astar::astar::*;
use traits::position::*;

fn formatPosition(position: &Rc<dyn Position>, possiblePrevious: &Option<Rc<dyn Position>>, row_layout: &[usize]) -> Result<String, String> {
    let mut out= String::new();
    let bottles = position.getBottles();
    let length = bottles.len();
    if let Some(previous) = possiblePrevious {
        if previous.getBottles().len() != length {
            return Err("Two positions have different length!".to_string());
        }
    }
    if row_layout.iter().sum::<usize>() != length {
        return Err("Row layout does not match bottle count!".to_string());
    }
    let mut row_start = 0;
    for (row_index, row_len) in row_layout.iter().enumerate() {
        for i in (0..4).rev() {
            for j in 0..*row_len {
                let index = row_start + j;
                let ourBottle= &bottles[index];
                if let Some(_) = possiblePrevious.as_ref().filter(|p| ourBottle != &p.getBottles()[index]) {
                    write!(out, "❚{}❚", ourBottle.content[i] as char).unwrap();
                } else {
                    write!(out, "|{}|", ourBottle.content[i] as char).unwrap();
                }
            }
            writeln!(out).unwrap();
        }
        row_start += *row_len;
        if row_index + 1 < row_layout.len() {
            writeln!(out).unwrap();
        }
    }
    writeln!(out, "------------------------------------------------").unwrap();
    Ok(out)
}

fn printSolution(possibleSolution: Option<Vec<Rc<dyn Position>>>, row_layout: &[usize]) {
    if let Some(solution) = possibleSolution {
        let mut oldPosition: Option<Rc<dyn Position>>= None;
        for i in 0..solution.len() {
            println!("Step {}\n{}", i, formatPosition(&solution[i], &oldPosition, row_layout).unwrap());
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
/// - Reads lines of input where each line represents a row of bottle configurations separated by ';'.
/// - Each bottle can contain up to four ASCII characters representing its content; shorter entries are padded with spaces.
/// - Lines starting with '#' are considered as comments and are ignored.
/// - Lines that are entirely whitespace are also ignored.
/// - Validates that all characters in each bottle are ASCII and each bottle has at most four characters.
///
/// # Returns
/// - `Ok((Vec<Bottle>, Vec<usize>))` containing bottle data and row layout if the input is successfully read and validated.
/// - `Err(String)` if there are any input errors like non-ASCII characters, incorrect bottle lengths,
///   or if any other runtime error occurs during processing.
///
/// # Errors
/// - Returns `Err(String)` and prints an error message if any line contains non-ASCII characters.
/// - Returns `Err(String)` and prints an error message if any bottle contains more than four characters.
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
fn handleInputData() -> Result<(Vec<Bottle>, Vec<usize>), String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut data: Vec<Bottle>= Vec::new();
    let mut row_layout: Vec<usize>= Vec::new();
    // Read each line from standard input
    for raw_line in input.lines() {
        let line = raw_line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        if line.trim_start().starts_with('#') {
            continue;
        }
        let mut row_count = 0;
        for bottle in line.split(';') {
            if !bottle.is_ascii() {
                return Err("Error: All characters must be ASCII".to_string());
            }
            if bottle.len() > 4 {
                return Err("Error: Each bottle must contain at most four characters.".to_string());
            }
            let paddedBottle= format!("{:width$}", bottle, width = 4);
            let array: [u8; 4] = paddedBottle.as_bytes().to_vec().try_into().unwrap();
            data.push(Bottle::new(array));
            row_count += 1;
        }
        row_layout.push(row_count);
    }
    if data.is_empty() {
        return Err("Error: No bottle data found.".to_string());
    }
    Ok((data, row_layout))
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
        Ok((data, row_layout)) => {
            if matches.get_flag("bfs") {
                let position= PositionBFS::new(data, 0);
                if let Some(error) = position.isValid().err() {
                    eprintln!("{}", error);
                    process::exit(1);
                }
                let mut bfs= BFS::new(position);
                printSolution(bfs.solve(), &row_layout); 
            } else {
                let position= PositionAstar::new(data);
                if let Some(error) = position.isValid().err() {
                    eprintln!("{}", error);
                    process::exit(1);
                }
                let mut astar= Astar::new(position);
                printSolution(astar.solve(), &row_layout);
            }
        },
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

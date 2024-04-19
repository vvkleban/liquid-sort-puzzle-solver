#![allow(non_snake_case)]
//#[derive(Debug, Clone)]
//struct Student {
//    name: String,
//    grade: u8,
//}

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
//    // Create a few students
//    let student1 = Student { name: String::from("Alice"), grade: 90 };
//    let student2 = Student { name: String::from("Bob"), grade: 85 };
//    let student3 = Student { name: String::from("Charlie"), grade: 92 };
//    let student4 = Student { name: String::from("Dana"), grade: 88 };
//
//    // Create a classroom with some students
//    let classroom1 = vec![student1.clone(), student2];
//    let classroom2 = vec![student3, student4];
//
//    // Create the school as a vector of classrooms
//    let school = vec![classroom1, classroom2];
//
//    // Now, school is a vector of vectors of Student objects
//    println!("{:?}", school);
//    println!("{:?}", student1);
//      let mut bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
//      let mut bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
//      let mut bottle3= Bottle::new([ 'B', 'B', 'A', ' ']);
//      println!("{:?}", bottle1);
//      println!("{:?}", bottle2);
//      let position= Position::new(vec![bottle1, bottle2, bottle3], 0);
//      let mut bfs= BFS::new(position);
//      let solution= bfs.solve();
//      println!("{:?}", solution);

//    let b1= Bottle::new([ 'A', 'B', 'C', 'D']);
//    let b2= Bottle::new([ 'E', 'A', 'C', 'D']);
//    let b3= Bottle::new([ 'F', 'G', 'E', 'A']);
//    let b4= Bottle::new([ 'H', 'I', 'J', 'J']);
//    let b5= Bottle::new([ 'K', 'H', 'L', 'J']);
//    let b6= Bottle::new([ 'M', 'M', 'H', 'L']);
//
//    let b7= Bottle::new([ 'M', 'K', 'G', 'N']);
//    let b8= Bottle::new([ 'M', 'H', 'N', 'B']);
//    let b9= Bottle::new([ 'F', 'O', 'O', 'L']);
//    let b10= Bottle::new([ 'K', 'G', 'G', 'B']);
//    let b11= Bottle::new([ 'A', 'O', 'D', 'L']);
//    let b12= Bottle::new([ 'F', 'K', 'C', 'O']);
//
//    let b13= Bottle::new([ 'F', 'I', 'I', 'D']);
//    let b14= Bottle::new([ 'E', 'E', 'B', 'C']);
//    let b15= Bottle::new([ 'N', 'N', 'I', 'J']);
//    let b16= Bottle::new([ ' ', ' ', ' ', ' ']);
//    let b17= Bottle::new([ ' ', ' ', ' ', ' ']);
//    let b18= Bottle::new([ ' ', ' ', ' ', ' ']);
//    let position= Position::new(vec![b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16, b17, b18], 0);
//    let b1= Bottle::new([ 'A', 'B', 'C', 'B']);
//    let b2= Bottle::new([ 'D', 'E', 'F', 'E']);
//    let b3= Bottle::new([ 'G', 'G', 'H', 'A']);
//    let b4= Bottle::new([ 'H', 'F', 'C', 'D']);
//
//    let b5= Bottle::new([ 'I', 'G', 'A', 'C']);
//    let b6= Bottle::new([ 'F', 'H', 'I', 'E']);
//    let b7= Bottle::new([ 'E', 'A', 'H', 'G']);
//    let b8= Bottle::new([ 'C', 'I', 'I', 'D']);
//
//    let b9= Bottle::new([ 'B', 'B', 'F', 'D']);
//    let b17= Bottle::new([ ' ', ' ', ' ', ' ']);
//    let b18= Bottle::new([ ' ', ' ', ' ', ' ']);
//    let position= Position::new(vec![b1, b2, b3, b4, b5, b6, b7, b8, b9, b17, b18], 0);
//    let mut bfs= BFS::new(position);
//    let solution= bfs.solve();
//    println!("{:?}", solution);
    handleInputData().unwrap_or_else(|()| process::exit(1));
//      bottle1.fillFrom(&mut bottle2);
//      println!("{:?}", bottle1);
//      println!("{:?}", bottle2);
}

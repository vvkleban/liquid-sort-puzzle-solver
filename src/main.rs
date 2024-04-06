#![allow(non_snake_case)]
//#[derive(Debug, Clone)]
//struct Student {
//    name: String,
//    grade: u8,
//}

mod bottle;
mod position;

use bottle::*;
use position::*;

struct Move {
    positions: Vec<Position>
}

struct BFS {
    moves: Vec<Move>
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
      let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
      let bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
      println!("{:?}", bottle1);
      println!("{:?}", bottle2);
      let (newBottle1, newBottle2)= bottle1.fillFrom(bottle2).unwrap();
      println!("{:?}", newBottle1);
      println!("{:?}", newBottle2);
}

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::Write;
use std::fmt;
use crate::bottle::*;

#[derive(Debug, Clone)]
pub struct Position {
    bottles: Vec<Bottle>,
    pub previous: usize,
    pub identity: Vec<char>
}

impl Position {
    pub fn new(bottles: Vec<Bottle>, previous: usize) -> Self {
        let mut cloned_bottles = bottles.clone();
        cloned_bottles.sort();
        let identity: Vec<char> = cloned_bottles
            .iter()
            .flat_map(|bottle| bottle.content.iter().copied())
            .collect();
        Self { bottles, previous, identity }
    }

    pub fn isSolved(&self) -> bool {
        self.bottles.iter().all(|bottle| bottle.isSolved())
    }

    pub fn isValid(&self) -> Result<(), String> {
        let mut char_count = HashMap::new();
        // Iterate over each array and then each character in the array
        for bottle in &self.bottles {
            for &ch in bottle.content.iter() {
                if ch != ' ' {
                    *char_count.entry(ch).or_insert(0) += 1;
                }
            }
        }

        if char_count.values().all(|&count| count == 4) {
            return Ok(());
        }
        let mut error= String::new();
        error.push_str("Error, the position is invalid! Please count the characters: \n");
        for (&character, &count) in char_count.iter() {
            if count != 4 {
                write!(error, "Character: '{}', Count: {}\n", character, count).unwrap();
            }
        }
        Err(error)
    }

    pub fn getNextPossiblePositions(&self, myIndex: usize) -> Vec<Position> {
        let mut result: Vec<Position>= Vec::new();
        let mut newBottles= self.bottles.clone();
        let bottleNum= self.bottles.len();
        for i in 0..bottleNum {
            for j in 0..bottleNum {
                if i < j {
                    let (left, right) = newBottles.split_at_mut(j);
                    if left[i].fillFrom(&mut right[0]) {
                        result.push(Position::new(newBottles, myIndex));
                        newBottles= self.bottles.clone();
                    }
                }
                else if i > j {
                    let (left, right) = newBottles.split_at_mut(i);
                    if right[0].fillFrom(&mut left[j]) {
                        result.push(Position::new(newBottles, myIndex));
                        newBottles= self.bottles.clone();
                    }
                }
            }
        }
        result
    }

    pub fn toString(&self, possiblePrevious: Option<&Position>) -> Result<String, String> {
        let mut out= String::new();
        let length= self.bottles.len();
        if let Some(previous) = possiblePrevious {
            if previous.bottles.len() != length {
                return Err("Two positions have different length!".to_string());
            }
        }
        for i in (0..4).rev() {
            for j in 0..length {
                if j > 0 && j % ((length + 2) / 3) == 0 {
                    out.push_str("  ");
                }
                let ourBottle= &self.bottles[j];
                // if previous exists and our bottles aren't equal, make our bottle bold
                if let Some(_) = possiblePrevious.filter(|p| ourBottle != &p.bottles[j]) {
                    write!(out, "❚{}❚", ourBottle.content[i]).unwrap();
                } else {
                    write!(out, "|{}|", ourBottle.content[i]).unwrap();
                }
                
            }
            writeln!(out).unwrap();
        }
        writeln!(out, "------------------------------------------------").unwrap();
        Ok(out)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash of our identity is all that matters
        self.identity.hash(state);
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..4).rev() {
            for bottle in &self.bottles {
                write!(f, "|{}|", bottle.content[i])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "------------------------------------------------")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn checkNextPositions() {
        let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle5= Bottle::new([ 'B', 'B', 'B', ' ']);
        let pos1= Position::new(vec![bottle1, bottle2, bottle4, bottle5], 0);
        let newPositions= pos1.getNextPossiblePositions(0);
        assert_eq!(newPositions.len(), 4);
        let expectedIdentities: HashSet<Vec<char>> = HashSet::from_iter([
            vec!['A', 'A', 'A', ' ', 'B', 'B', ' ', ' ', 'B', 'B', ' ', ' ', 'B', 'B', 'B', ' '],
            vec!['A', ' ', ' ', ' ', 'B', 'B', ' ', ' ', 'B', 'B', 'A', 'A', 'B', 'B', 'B', ' '],
            vec!['A', 'A', ' ', ' ', 'B', ' ', ' ', ' ', 'B', 'B', 'A', ' ', 'B', 'B', 'B', 'B']
        ].iter().cloned());
//        println!("Checker:");
//        for identity in &expectedIdentities {
//            println!("{:?}", identity);
//        }
//        println!("Result:");
        newPositions.iter().for_each(|position| { /* println!("{:?}", position.identity); */ assert_eq!(expectedIdentities.contains(&position.identity), true) });
    }

    #[test]
    fn checkNumNextPositions() {
        let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', 'A', ' ']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle5= Bottle::new([ 'B', 'B', 'B', ' ']);
        let pos1= Position::new(vec![bottle1, bottle2, bottle3, bottle4, bottle5], 0);
        let newPositions= pos1.getNextPossiblePositions(0);
        assert_eq!(newPositions.len(), 8);
//        for pos in &newPositions {
//            println!("{:?}", pos);
//        }
    }
}

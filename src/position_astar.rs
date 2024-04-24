use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt::Write;
use std::fmt;
use std::rc::Rc;
use crate::bottle::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a specific arrangement or position of bottles.
pub struct PositionAstar {
    bottles: Vec<Bottle>, // Holds the current arrangement of bottles.
    pub previous: Option<Rc<PositionAstar>>,  // Link to the previous position in the game
    // Unique identifier based on the sorted contents of the bottles.
    // Allows to identify identical positions with all bottle permutations
    pub currentCost: u32,
    pub totalProjectedCost: u32
}

impl PositionAstar {

    pub fn new(bottles: Vec<Bottle>) -> PositionAstar {
        PositionAstar::newChild(bottles, None, 0, 0)
    }

    pub fn newChild(
        bottles: Vec<Bottle>,
        previous: Option<Rc<PositionAstar>>,
        currentCost: u32,
        projectedCost: u32) -> Self
    {
        Self { bottles, previous, currentCost, totalProjectedCost: currentCost + projectedCost }
    }

    pub fn getIdentity(&self) -> Vec<u8> {
        let mut cloned_bottles = self.bottles.clone();
        cloned_bottles.sort();
        cloned_bottles
            .into_iter()
            .flat_map(|bottle| bottle.content.into_iter())
            .collect()
    }

    #[inline]
    /// # Returns
    /// `true` if all bottles are solved, otherwise `false`.
    pub fn isSolved(&self) -> bool {
        self.bottles.iter().all(|bottle| bottle.isSolved())
    }

    pub fn getNumColors(&self) -> usize {
        let mut char_count = HashSet::new();
        for bottle in &self.bottles {
            for &ch in bottle.content.iter() {
                if ch != ' ' as u8 {
                    char_count.insert(ch);
                }
            }
        }
        char_count.len()
    }

    /// Validates that all characters (colors) in the bottles appear exactly 4 times,
    /// ensuring the position meets game rules.
    ///
    /// # Returns
    /// `Ok(())` if the validation passes, otherwise `Err` with a message detailing inconsistencies.
    pub fn isValid(&self) -> Result<(), String> {
        let mut char_count = HashMap::new();
        // Iterate over each array and then each character in the array
        for bottle in &self.bottles {
            for &ch in bottle.content.iter() {
                if ch != ' ' as u8 {
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

    /// Calculates the syntropy value for a given identity of a position.
    /// Syntropy measures the orderliness based on consecutive identical slots in each bottle.
    ///
    /// # Arguments
    /// * `identity` - A reference to the identity vector of a position.
    ///
    /// # Returns
    /// An usize representing the syntropy value.
    fn getSyntropy(bottles: &Vec<Bottle>) -> u32 {
        let mut syntropy= 0;
        for b in bottles {
            for i in 0..3 {
                if b.content[i] != (' ' as u8) && b.content[i] == b.content[i+1] {
                    syntropy += 1;
                }
            }
        }
        syntropy 
    }

    /// Generates all valid next positions reachable in one move by attempting to transfer contents
    /// between each pair of bottles.
    ///
    /// # Arguments
    /// * `targetSyntropy` - The syntropy value of a puzzle solution (predictable and the same for
    /// all solutions
    ///
    /// # Returns
    /// A vector of `Position` instances representing all possible next states.
    pub fn getNextPossiblePositions(parent: &Rc<PositionAstar>, targetSyntropy: u32) -> Vec<Rc<PositionAstar>> {
        let mut result= Vec::new();
        let mut newBottles= parent.bottles.clone();
        let bottleNum= newBottles.len();
        for i in 0..bottleNum {
            for j in 0..bottleNum {
                if i < j {
                    let (left, right) = newBottles.split_at_mut(j);
                    if left[i].fillFrom(&mut right[0]) {
                        let newSyntropy= PositionAstar::getSyntropy(&newBottles);
                        result.push(Rc::new(PositionAstar::newChild(newBottles, Some(parent.clone()), parent.currentCost + 1, targetSyntropy - newSyntropy)));
                        newBottles= parent.bottles.clone();
                    }
                }
                else if i > j {
                    let (left, right) = newBottles.split_at_mut(i);
                    if right[0].fillFrom(&mut left[j]) {
                        let newSyntropy= PositionAstar::getSyntropy(&newBottles);
                        result.push(Rc::new(PositionAstar::newChild(newBottles, Some(parent.clone()), parent.currentCost + 1, targetSyntropy - newSyntropy)));
                        newBottles= parent.bottles.clone();
                    }
                }
            }
        }
        result
    }

    /// Creates a string representation of the position,
    /// optionally highlighting differences from a previous position.
    ///
    /// # Arguments
    /// * `possiblePrevious` - An optional reference to a previous position for comparison.
    ///
    /// # Returns
    /// `Ok(String)` containing the formatted position or an `Err(String)` if there's a length mismatch.
    pub fn toString(&self, possiblePrevious: Option<&PositionAstar>) -> Result<String, String> {
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
                    write!(out, "❚{}❚", ourBottle.content[i] as char).unwrap();
                } else {
                    write!(out, "|{}|", ourBottle.content[i] as char).unwrap();
                }
                
            }
            writeln!(out).unwrap();
        }
        writeln!(out, "------------------------------------------------").unwrap();
        Ok(out)
    }
}

impl fmt::Display for PositionAstar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..4).rev() {
            for bottle in &self.bottles {
                write!(f, "|{}|", bottle.content[i] as char)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "------------------------------------------------")?;
        Ok(())
    }
}

impl PartialOrd for PositionAstar {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PositionAstar {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if other.totalProjectedCost == self.totalProjectedCost {
            // Depth-first - prioritize higher current cost
            return self.currentCost.cmp(&other.currentCost);
        }
        // Prioritize lower projected total cost
        other.totalProjectedCost.cmp(&self.totalProjectedCost)
    }
} 

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn checkNextPositions() {
        let bottle1= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::newChars([ 'B', 'B', 'A', ' ']);
        let bottle4= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let bottle5= Bottle::newChars([ 'B', 'B', 'B', ' ']);
        let pos1= Rc::new(PositionAstar::new(vec![bottle1, bottle2, bottle4, bottle5]));
        let newPositions= PositionAstar::getNextPossiblePositions(&pos1, 100);
        assert_eq!(newPositions.len(), 4);
        let expectedIdentities: HashSet<Vec<u8>> = Vec::from_iter([
            vec!['A', 'A', 'A', ' ', 'B', 'B', ' ', ' ', 'B', 'B', ' ', ' ', 'B', 'B', 'B', ' '],
            vec!['A', ' ', ' ', ' ', 'B', 'B', ' ', ' ', 'B', 'B', 'A', 'A', 'B', 'B', 'B', ' '],
            vec!['A', 'A', ' ', ' ', 'B', ' ', ' ', ' ', 'B', 'B', 'A', ' ', 'B', 'B', 'B', 'B']
        ]).into_iter()
        .map(|inner_vec| 
            {
                inner_vec.into_iter()
                .map(|c| c as u8) // Safe cast, as all chars are guaranteed to be ASCII
                .collect::<Vec<u8>>() // Collect bytes into Vec<u8>
            }).collect();
//        println!("Checker:");
//        for identity in &expectedIdentities {
//            println!("{:?}", identity);
//        }
//        println!("Result:");
        newPositions.iter().for_each(|position| { /* println!("{:?}", position.identity); */ assert_eq!(expectedIdentities.contains(&position.getIdentity()), true) });
    }

    #[test]
    fn checkNumNextPositions() {
        let bottle1= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::newChars([ 'B', 'B', 'A', ' ']);
        let bottle3= Bottle::newChars([ 'A', 'A', 'A', ' ']);
        let bottle4= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let bottle5= Bottle::newChars([ 'B', 'B', 'B', ' ']);
        let pos1= Rc::new(PositionAstar::new(vec![bottle1, bottle2, bottle3, bottle4, bottle5]));
        let newPositions= PositionAstar::getNextPossiblePositions(&pos1, 100);
        assert_eq!(newPositions.len(), 8);
//        for pos in &newPositions {
//            println!("{:?}", pos);
//        }
    }
}

use std::hash::{Hash, Hasher};
use crate::bottle::*;

#[derive(Debug, Clone)]
pub struct Position {
    bottles: Vec<Bottle>,
    previous: usize,
    identity: Vec<char>
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn checkNextPositions() {
        let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', 'A', ' ']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle5= Bottle::new([ 'B', 'B', 'B', ' ']);
        let pos1= Position::new(vec![bottle1, bottle2, bottle3, bottle4, bottle5], 0);
        let newPositions= pos1.getNextPossiblePositions(0);
        assert_eq!(newPositions.len(), 8);
        for pos in &newPositions {
            println!("{:?}", pos);
        }
    }
}

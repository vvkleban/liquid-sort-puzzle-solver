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

    pub fn generateNew(&self, myIndex: usize) -> Vec<Bottle> {
        for i in 0..self.bottles.len() {
            for j in 0..self.bottles.len() {
                if i != j {
                    if let Some(copy1, copy2) =self.bottles[i].fillFrom(bottles[j]
                    {
                    }
                }
            }
        }
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

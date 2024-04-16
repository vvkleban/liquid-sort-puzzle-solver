use std::collections::HashSet;
use crate::position::*;

struct Move {
    positions: Vec<Position>
}

pub struct BFS {
    moves: Vec<Move>,
    uniquePositions: HashSet<Vec<char>>,
    syntropy: u32
}

impl Move {
    pub fn new(positions: Vec<Position>) -> Self {
        Self { positions }
    }

    pub fn choices(&self) -> usize {
        self.positions.len()
    }
}

impl BFS {
    pub fn new(initialPosition: Position) -> Self {
        Self { moves: vec![Move::new(vec![initialPosition.clone()]) ],
               uniquePositions: HashSet::from_iter(std::iter::once(initialPosition.identity)),
               syntropy: u32::MAX}
    }

    pub fn solve(&mut self) -> Option<Vec<Position>> {
        loop {
            let aMove= &self.moves[self.moves.len() - 1];
            // If we ran out of move choices, there is no solution
            if aMove.choices() < 1 {
                return None
            }
            // Look for a solution in a given move
            for positionIndex in 0..aMove.positions.len() {
                if aMove.positions[positionIndex].isSolved() {
                    return Some(self.buildSolutionVector(self.moves.len() - 1, positionIndex));
                }
            }
            // No solution was found, so make the next move
            self.generateNewMoveChoices();
        }
    }

    fn compactBFS(&mut self, index: usize) -> usize {
        if index < 1 { return 0; }
        let (previousMoves, newMove)= self.moves.split_at_mut(index);
        let currentPositions= &mut newMove[0].positions;
        let oldPositions= &previousMoves[index - 1].positions;
        let mut compactedOldPositions= Vec::new();
        let mut deleted= 0;
        let mut lastOldIndex: isize= -1;
        for newPositionIndex in 0..currentPositions.len() {
            let currentPosition= &mut currentPositions[newPositionIndex];
            let previous= currentPosition.previous;
            if previous as isize > lastOldIndex {
                compactedOldPositions.push(oldPositions[previous].clone());
                deleted += previous - (lastOldIndex as usize) + 1;
                lastOldIndex= previous as isize;
            }
            currentPosition.previous= compactedOldPositions.len() - 1;
        }
        previousMoves[index - 1].positions= compactedOldPositions;
        deleted += self.compactBFS(index - 1);
        deleted
    }

    fn generateNewMoveChoices(&mut self) {
        let mut newMove= Move::new(Vec::new());
        let currentPositions= &self.moves[self.moves.len() - 1].positions;
        let mut candidates= 0;
        for positionIndex in 0..currentPositions.len() {
            newMove.positions.extend(currentPositions[positionIndex]
                .getNextPossiblePositions(positionIndex)
                .into_iter()
                .filter(|position| { candidates += 1; self.uniquePositions.insert(position.identity.clone()) } ));
        }
        println!("Iteration: {}, Candidates: {}, Moves: {}", self.moves.len() + 1, candidates, newMove.positions.len());
        println!("Pruned {} dead positions", self.compactBFS(self.moves.len() - 1));
        self.moves.push(newMove);
    }

    fn buildSolutionVector(&self, moveIndex: usize, positionIndex: usize) -> Vec<Position> {
        let position= self.moves[moveIndex].positions[positionIndex].clone();
        if moveIndex == 0 {
            return vec![position];
        } 
        let mut previousMoves = self.buildSolutionVector(moveIndex - 1, position.previous);
        previousMoves.push(position);
        previousMoves
    }
}

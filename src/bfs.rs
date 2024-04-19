use std::collections::HashSet;
use crate::position::*;

struct Move {
    positions: Vec<Position>
}

pub struct BFS {
    moves: Vec<Move>,
    uniquePositions: HashSet<Vec<u8>>,
    syntropy: usize
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
               syntropy: 0}
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
        let mut newSyntropy= 0;
        for positionIndex in 0..currentPositions.len() {
            newMove.positions.extend(currentPositions[positionIndex]
                .getNextPossiblePositions(positionIndex)
                .into_iter()
                .filter(|position| { candidates += 1; self.uniquePositions.insert(position.identity.clone()) } )
                .inspect(|position| { let syntropy= BFS::getSyntropy(&position.identity);
                                      if syntropy > newSyntropy {
                                          newSyntropy= syntropy;
                                      } }));
        }
        println!("Iteration: {}, Candidates: {}, Moves: {}", self.moves.len() + 1, candidates, newMove.positions.len());
        println!("Pruned {} dead positions", self.compactBFS(self.moves.len() - 1));
        /*if newSyntropy > self.syntropy {
            println!("New syntropy is {}", newSyntropy);
            self.syntropy= newSyntropy;
            let oldHashSize= self.uniquePositions.len();
            self.uniquePositions.retain(|identity| BFS::getSyntropy(identity) >= newSyntropy);
            println!("Compacting unique positions hash from {} to {}", oldHashSize, self.uniquePositions.len());
        } */
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

    fn getSyntropy(identity: &Vec<u8>) -> usize {
        let mut syntropy= 0;
        for i in 0..(identity.len() / 4) {
            for j in 0..3 {
                if identity[i*4 + j] != (' ' as u8) && identity[i*4 + j] == identity[i*4 + j + 1] {
                    syntropy += 1;
                }
            }
        }
        syntropy
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn checkSyntropy() {
        let identity= vec!['A', 'A', ' ', ' ', 'B', 'B','C', 'C']
            .into_iter().map(|c| c as u8).collect();
        assert_eq!(BFS::getSyntropy(&identity), 3);
        let identity= vec!['A', 'A', ' ', ' ', 'B', 'B','C', 'C', 'C', 'C', 'A', 'A']
            .into_iter().map(|c| c as u8).collect();
        assert_eq!(BFS::getSyntropy(&identity), 5);
    }
}

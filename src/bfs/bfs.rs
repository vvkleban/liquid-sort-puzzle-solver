use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;
use crate::bfs::position_bfs::*;
use crate::traits::position::*;

/// Represents a move in the BFS algorithm which consists of a list of positions.
struct Move {
    positions: Vec<PositionBFS> // All unique (barring permutations) positions reachable by this move
}

/// Handles the BFS (Breadth-First Search) process for solving a puzzle.
pub struct BFS {
    // All reachable positions arranged per move starting from the initial position
    moves: Vec<Move>, 
    // Collision sets to make sure all positions recorded are truly unique (barring bottle
    // permutations). The sets are arranged per syntropy, where each syntropy value maps to a set
    // of positions with this syntropy value.
    // The syntropy value is a measure of the orderliness of a position: how many consecutive slots
    // in every bottle are filled with the same color. E.g. a Bottle "BAAA" will have a syntropy
    // value of 2, because two 'A' are stacked on another 'A'. A syntropy can never decrease
    // between the moves. It can only increase or stay the same (in case we transfer some color to
    // an empty bottle)
    uniquePositions: HashMap<usize,HashSet<Vec<u8>>>,
    // Minimum syntropy of the positions in the last move
    syntropy: usize
}

impl Move {
    pub fn new(positions: Vec<PositionBFS>) -> Self {
        Self { positions }
    }

    #[inline]
    /// Returns the number of choices available in this move, equating to the number of positions.
    pub fn choices(&self) -> usize {
        self.positions.len()
    }
}

impl BFS {
    /// Constructs a new BFS instance with an initial position, initializes the `uniquePositions` map,
    /// and calculates the initial syntropy based on the identity of the initial position.
    ///
    /// # Arguments
    /// * `initialPosition` - The starting point of the BFS.
    pub fn new(initialPosition: PositionBFS) -> Self {
        let syntropy= BFS::getSyntropy(&initialPosition.getIdentity());
        let firstHS= HashSet::from_iter(std::iter::once(initialPosition.getIdentity()));
        let mv= Move::new(vec![initialPosition]);
        let mut HM= HashMap::new();
        HM.insert(syntropy, firstHS);
        Self { moves: vec![ mv ],
               uniquePositions: HM,
               syntropy}
    }

    /// Executes the BFS algorithm to find a solution.
    ///
    /// # Returns
    /// `Option<Vec<Position>>` representing the sequence of moves to solve the puzzle if a solution is found.
    /// `None` if no solution is possible.
    pub fn solve(&mut self) -> Option<Vec<Rc<dyn Position>>> {
        loop {
            let aMove= &self.moves[self.moves.len() - 1];
            // If we ran out of move choices, there is no solution
            if aMove.choices() < 1 {
                return None
            }
            // Look for a solution in a given move
            for positionIndex in 0..aMove.choices() {
                if aMove.positions[positionIndex].isSolved() {
                    return Some(self.buildSolutionVector(self.moves.len() - 1, positionIndex));
                }
            }
            // No solution was found, so make the next move
            self.generateNewMoveChoices();
        }
    }

    /// Recursively compacts the BFS tree from a specified index to remove redundant positions and save space.
    ///
    /// # Arguments
    /// * `index` - The index from where to start the compaction process.
    ///
    /// # Returns
    /// The number of positions that were removed during compaction.
    fn compactBFS(&mut self, index: usize) -> usize {
        if index < 1 { return 0; }
        let (previousMoves, newMove)= self.moves.split_at_mut(index);
        let currentPositions= &mut newMove[0].positions;
        let oldPositions= &previousMoves[index - 1].positions;
        let mut compactedOldPositions= Vec::new();
        let mut lastOldIndex: isize= -1;
        for newPositionIndex in 0..currentPositions.len() {
            let currentPosition= &mut currentPositions[newPositionIndex];
            let previous= currentPosition.previous;
            if previous as isize > lastOldIndex {
                compactedOldPositions.push(oldPositions[previous].clone());
                lastOldIndex= previous as isize;
            }
            currentPosition.previous= compactedOldPositions.len() - 1;
        }
        let deleted= oldPositions.len() - compactedOldPositions.len();
        previousMoves[index - 1].positions= compactedOldPositions;
        deleted + self.compactBFS(index - 1)
    }

    /// Generates a new move vector consisting of all possible new positions from all reachable positions 
    /// in the latest move and updates the BFS structure accordingly.
    fn generateNewMoveChoices(&mut self) {
        let mut newMove= Move::new(Vec::new());
        let currentPositions= &self.moves[self.moves.len() - 1].positions;
        let mut candidates= 0;
        let mut newMinSyntropy= usize::MAX;
        for positionIndex in 0..currentPositions.len() {
            for candidate in currentPositions[positionIndex]
                  .getNextPossiblePositions(positionIndex) 
            {
                candidates += 1;
                let candidateIdentity= candidate.getIdentity();
                let syntropy= BFS::getSyntropy(&candidateIdentity);
                if !self.uniquePositions
                    .entry(syntropy)
                    .or_insert_with(HashSet::new)
                    .insert(candidateIdentity)
                {
                    continue;
                }
                if syntropy < newMinSyntropy {
                    newMinSyntropy= syntropy;
                }
                newMove.positions.push(candidate);
            }
        }
        println!("Iteration: {}, Candidates: {}, Moves: {}", self.moves.len(), candidates, newMove.positions.len());
        if self.moves.len() % 5 == 0 {
            println!("Pruned {} dead positions", self.compactBFS(self.moves.len() - 1));
        }
        if newMinSyntropy > self.syntropy {
            println!("New syntropy is {}", newMinSyntropy);
            self.syntropy= newMinSyntropy;
            let oldHashSize= self.uniquePositions.len();
            self.uniquePositions.retain(|&syn, _| syn >= newMinSyntropy);
            println!("Compacting unique positions hash from {} to {}", oldHashSize, self.uniquePositions.len());
        } 
        self.moves.push(newMove);
    }

    /// Constructs a vector of `Position` objects representing the path from the initial position to
    /// the given position in the solution sequence. It is used to provide a solution to the
    /// puzzle. In fact - the shortest possible solution as per BFS algorithm
    ///
    /// # Arguments
    /// * `moveIndex` - Index of the move in `moves`.
    /// * `positionIndex` - Index of the position in the move specified by `moveIndex`.
    ///
    /// # Returns
    /// A vector of `Position` instances tracing the path from the start to the specified position.
    fn buildSolutionVector(&self, moveIndex: usize, positionIndex: usize) -> Vec<Rc<dyn Position>> {
        let position= self.moves[moveIndex].positions[positionIndex].clone();
        if moveIndex == 0 {
            return vec![Rc::new(position)];
        } 
        let mut previousMoves = self.buildSolutionVector(moveIndex - 1, position.previous);
        previousMoves.push(Rc::new(position));
        previousMoves
    }

    /// Calculates the syntropy value for a given identity of a position.
    /// Syntropy measures the orderliness based on consecutive identical slots in each bottle.
    ///
    /// # Arguments
    /// * `identity` - A reference to the identity vector of a position.
    ///
    /// # Returns
    /// An usize representing the syntropy value.
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

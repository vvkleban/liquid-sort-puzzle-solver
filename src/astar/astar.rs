use std::rc::Rc;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use crate::astar::position_astar::*;
use crate::traits::position::*;

pub struct Astar {
    heap: BinaryHeap<Rc<PositionAstar>>,
    // Collision sets to make sure all positions recorded are truly unique (barring bottle
    // permutations). 
    uniquePositions: HashSet<Vec<u8>>,
}

impl Astar {
    /// Constructs a new Astar instance with an initial position, initializes the `uniquePositions` map,
    /// and calculates the initial syntropy based on the identity of the initial position.
    ///
    /// # Arguments
    /// * `initialPosition` - The starting point of the BFS.
    pub fn new(initialPosition: PositionAstar) -> Self {
        let uniquePositions= HashSet::from_iter(std::iter::once(initialPosition.getIdentity()));
        let heap= BinaryHeap::from_iter(std::iter::once(Rc::new(initialPosition)));
        Self { heap,
               uniquePositions }
    }

    /// Executes the A* algorithm to find a solution.
    ///
    /// # Returns
    /// `Option<Vec<Position>>` representing the sequence of moves to solve the puzzle if a solution is found.
    /// `None` if no solution is possible.
    pub fn solve(&mut self) -> Option<Vec<Rc<dyn Position>>> {
        while let Some(candidate)= self.heap.pop() {
            if candidate.isSolved() {
                return Some(Astar::buildSolutionVector(candidate));
            }
            for position in PositionAstar::getNextPossiblePositions(&candidate) {
                let candidateIdentity= position.getIdentity();
                if !self.uniquePositions.insert(candidateIdentity) {
                    continue;
                } 
                self.heap.push(position);
            }
        }
        None
    }

    /// Constructs a vector of `Position` objects representing the path from the initial position to
    /// the given position in the solution sequence. It is used to provide a solution to the
    /// puzzle. In fact - the shortest possible solution as per BFS algorithm
    ///
    /// # Arguments
    /// * `candidate` - The final resolved state of the puzzle
    ///
    /// # Returns
    /// A vector of `Position` instances tracing the path from the start to the specified position.
    fn buildSolutionVector(candidate: Rc<PositionAstar>) -> Vec<Rc<dyn Position>> {
        let mut solution=vec![ Rc::clone(&candidate) as Rc<dyn Position> ]; 
        let mut optionalPosition: &Option<Rc<PositionAstar>>= &candidate.previous;
        while let Some(position)= optionalPosition {
            solution.insert(0, (*position).clone());
            optionalPosition= &position.previous;
        }
        solution
    }

}

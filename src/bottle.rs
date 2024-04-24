/// Representation of a bottle in a liquid sort game
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bottle {
    // ascii letters represent colors. ' ' (space) represents empty space
    pub content: [u8; 4]
}

impl Bottle {
    pub fn new(content: [u8; 4]) -> Self {
        Self { content }
    }

    #[inline]
    pub fn isEmpty(&self) -> bool {
        self.content[0] == ' ' as u8
    }

    #[inline]
    /// Checks if the bottle is in a "solved" state.
    /// A bottle is considered solved if all its content slots contain the same non-space character, 
    /// or if the first slot is a space, implying the bottle is empty or correctly arranged.
    pub fn isSolved(&self) -> bool {
        if self.isEmpty() {
            return true;
        }
        let color= self.content[0];
        self.content[1] == color && self.content[2] == color && self.content[3] == color
    }

    #[inline]
    /// Return the top empty slot index
    pub fn getTopIndex(&self) -> usize {
        let mut i: usize= 3;
        while self.content[i] == (' ' as u8) && i > 0 {
            i-= 1;
        }
        i
    }

    /// Return how many towers of the same color are stacked in this bottle
    pub fn getColorTowers(&self) -> usize {
        if self.isEmpty() {
            return 0;
        }
        let mut towers= 1;
        for i in 1..4 {
            if self.content[i] != (' ' as u8) && self.content[i] != self.content[i-1] {
                towers += 1;
            }
        }
        towers
    }

    /// Transfers content from another `Bottle` (`other`) into this bottle.
    /// Filling starts from the topmost non-empty slot of the other bottle and ends either
    /// when this bottle is full or the other bottle has no more content to transfer.
    ///
    /// # Parameters
    /// * `other` - A mutable reference to another `Bottle` from which content will be transferred.
    ///
    /// # Returns
    /// `true` if any content was transferred, `false` otherwise.
    pub fn fillFrom(&mut self, other: &mut Bottle) -> bool {
        let mut otherTopIndex: isize= other.getTopIndex() as isize;
        if other.content[otherTopIndex as usize] == ' ' as u8 {
            return false;
        }
        let mut ourTopIndex= self.getTopIndex();
        let mut mutated= false;
        let ourTopColor= if self.content[0] == ' ' as u8 {
            self.content[0]= other.content[otherTopIndex as usize];
            other.content[otherTopIndex as usize]= ' ' as u8;
            otherTopIndex-= 1;
            mutated= true;
            self.content[0]
        } else { 
            self.content[ourTopIndex] 
        };
        while ourTopIndex < 3 
            && otherTopIndex >= 0
            && other.content[otherTopIndex as usize] == ourTopColor
        {
            ourTopIndex+= 1;
            self.content[ourTopIndex]= other.content[otherTopIndex as usize];
            other.content[otherTopIndex as usize]= ' ' as u8;
            otherTopIndex-= 1;
            mutated= true;
        }
        mutated
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    impl Bottle {
        pub fn newChars(content: [char; 4]) -> Self {
            let byteArray= [ content[0] as u8, content[1] as u8, content[2] as u8, content[3] as u8 ];
            Self { content: byteArray }
        }
    }

    #[test]
    fn fillCopySingleColor() {
        let mut bottle1= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let mut bottle2= Bottle::newChars([ 'B', 'B', 'A', ' ']);
        let bottle3= Bottle::newChars([ 'A', 'A', 'A', ' ']);
        let bottle4= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == true);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillOtherEmpty() {
        let mut bottle1= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let mut bottle2= Bottle::newChars([ ' ', ' ', ' ', ' ']);
        let bottle3= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let bottle4= Bottle::newChars([ ' ', ' ', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == false);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillIncompatibleColors() {
        let mut bottle1= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let mut bottle2= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::newChars([ 'A', 'A', ' ', ' ']);
        let bottle4= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == false);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillEmptyDestination() {
        let mut bottle1= Bottle::newChars([ ' ', ' ', ' ', ' ']);
        let mut bottle2= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let bottle4= Bottle::newChars([ ' ', ' ', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == true);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillAlreadyFull() {
        let mut bottle1= Bottle::newChars([ 'A', 'A', 'B', 'B']);
        let mut bottle2= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::newChars([ 'A', 'A', 'B', 'B']);
        let bottle4= Bottle::newChars([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == false);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn checkTowers() {
        assert_eq!(Bottle::getColorTowers(&Bottle::newChars([ 'A', 'A', ' ', ' '])), 1);
        assert_eq!(Bottle::getColorTowers(&Bottle::newChars([ 'B', 'B', 'A', ' '])), 2);
        assert_eq!(Bottle::getColorTowers(&Bottle::newChars([ 'A', 'B', 'D', 'C'])), 4);
        assert_eq!(Bottle::getColorTowers(&Bottle::newChars([ ' ', ' ', ' ', ' '])), 0);
    }
}

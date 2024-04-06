#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bottle {
   pub content: [char; 4]
}

impl Bottle {
    pub fn new(content: [char; 4]) -> Self {
        Self { content }
    }

    #[inline]
    pub fn isSolved(&self) -> bool {
        if self.content[0] == ' ' {
            return true;
        }
        let color= self.content[0];
        self.content[1] == color && self.content[2] == color && self.content[3] == color
    }

    #[inline]
    pub fn getTopIndex(&self) -> usize {
        let mut i: usize= 3;
        while self.content[i] == ' ' && i > 0 {
            i-= 1;
        }
        i
    }

    // If our bottle is fillable with the contents of the other, return the indices of where to
    // start
//    fn getTransferIndices(self, other: Bottle) -> Option<(usize, usize)> {
//    }

    pub fn fillFrom(self, other: Bottle) -> Option<(Bottle, Bottle)> {
        let mut otherTopIndex: isize= other.getTopIndex() as isize;
        if other.content[otherTopIndex as usize] == ' ' {
            return None;
        }
        let mut ourTopIndex= self.getTopIndex();
        let mut mutated= None;
        let ourTopColor= if self.content[0] == ' ' {
            mutated= Some((self.clone(), other.clone()));
            let (ref mut myCopy, ref mut otherCopy)= mutated.as_mut().unwrap();
            myCopy.content[0]= otherCopy.content[otherTopIndex as usize];
            otherCopy.content[otherTopIndex as usize]= ' ';
            otherTopIndex-= 1;
            myCopy.content[0]
        } else { 
            self.content[ourTopIndex] 
        };
        while ourTopIndex < 3 
            && otherTopIndex >= 0
            && other.content[otherTopIndex as usize] == ourTopColor
        {
            if mutated == None {
                mutated= Some((self.clone(), other.clone()));
            } 
            let (ref mut myCopy, ref mut otherCopy)= mutated.as_mut().unwrap();
            ourTopIndex+= 1;
            myCopy.content[ourTopIndex]= otherCopy.content[otherTopIndex as usize];
            otherCopy.content[otherTopIndex as usize]= ' ';
            otherTopIndex-= 1;
        }
        mutated
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fillCopySingleColor() {
        let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', 'A', ' ']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(bottle2);
        assert!(mutated != None);
        let (newBottle1, newBottle2)= mutated.unwrap();
        assert_eq!(newBottle1, bottle3);
        assert_eq!(newBottle2, bottle4);
    }

    #[test]
    fn fillOtherEmpty() {
        let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::new([ ' ', ' ', ' ', ' ']);
        let mutated= bottle1.fillFrom(bottle2);
        assert!(mutated == None);
    }

    #[test]
    fn fillIncompatibleColors() {
        let bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle2= Bottle::new([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(bottle2);
        assert!(mutated == None);
    }

    #[test]
    fn fillEmptyDestination() {
        let bottle1= Bottle::new([ ' ', ' ', ' ', ' ']);
        let bottle2= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle4= Bottle::new([ ' ', ' ', ' ', ' ']);
        let mutated= bottle1.fillFrom(bottle2);
        assert!(mutated != None);
        let (newBottle1, newBottle2)= mutated.unwrap();
        assert_eq!(newBottle1, bottle3);
        assert_eq!(newBottle2, bottle4);
    }

    #[test]
    fn fillAlreadyFull() {
        let bottle1= Bottle::new([ 'A', 'A', 'B', 'B']);
        let bottle2= Bottle::new([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(bottle2);
        assert!(mutated == None);
    }
}

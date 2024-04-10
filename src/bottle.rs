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

    pub fn fillFrom(&mut self, other: &mut Bottle) -> bool {
        let mut otherTopIndex: isize= other.getTopIndex() as isize;
        if other.content[otherTopIndex as usize] == ' ' {
            return false;
        }
        let mut ourTopIndex= self.getTopIndex();
        let mut mutated= false;
        let ourTopColor= if self.content[0] == ' ' {
            self.content[0]= other.content[otherTopIndex as usize];
            other.content[otherTopIndex as usize]= ' ';
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
            other.content[otherTopIndex as usize]= ' ';
            otherTopIndex-= 1;
            mutated= true;
        }
        mutated
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fillCopySingleColor() {
        let mut bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let mut bottle2= Bottle::new([ 'B', 'B', 'A', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', 'A', ' ']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == true);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillOtherEmpty() {
        let mut bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let mut bottle2= Bottle::new([ ' ', ' ', ' ', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle4= Bottle::new([ ' ', ' ', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == false);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillIncompatibleColors() {
        let mut bottle1= Bottle::new([ 'A', 'A', ' ', ' ']);
        let mut bottle2= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', ' ', ' ']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == false);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillEmptyDestination() {
        let mut bottle1= Bottle::new([ ' ', ' ', ' ', ' ']);
        let mut bottle2= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle4= Bottle::new([ ' ', ' ', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == true);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }

    #[test]
    fn fillAlreadyFull() {
        let mut bottle1= Bottle::new([ 'A', 'A', 'B', 'B']);
        let mut bottle2= Bottle::new([ 'B', 'B', ' ', ' ']);
        let bottle3= Bottle::new([ 'A', 'A', 'B', 'B']);
        let bottle4= Bottle::new([ 'B', 'B', ' ', ' ']);
        let mutated= bottle1.fillFrom(&mut bottle2);
        assert!(mutated == false);
        assert_eq!(bottle1, bottle3);
        assert_eq!(bottle2, bottle4);
    }
}

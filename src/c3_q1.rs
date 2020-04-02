//Three in One: Describe how you could use a single array to implement three stacks.

use crate::vec_stack::VecStack;

#[derive(Debug)]
struct MemRegion {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct TripleStack<T>
{
    triple: Vec<T>,
    regions: Vec<MemRegion>,
}

impl<T> TripleStack<T> {

    pub fn new(size: usize) -> Self{
        TripleStack {
            triple: vec![],
            regions: vec![
                MemRegion {
                    start: 0,
                    end: size / 3
                },
                MemRegion {
                    start: (size / 3) + 1,
                    end: (size / 3) * 2
                },
                MemRegion {
                    start: ((size / 3) * 2) + 1,
                    end: size
                }
            ]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let mut t_stack: TripleStack<usize> = TripleStack::new(100);
        println!("{:?}", t_stack);
    }
}
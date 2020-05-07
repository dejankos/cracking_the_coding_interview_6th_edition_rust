//Three in One: Describe how you could use a single array to implement three stacks.




#[derive(Debug)]
struct MemRegion {
    start: usize,
    end: usize,
    next_index: usize,
}

#[derive(Debug)]
struct TripleStack<T> {
    triple: Vec<Option<T>>,
    regions: Vec<MemRegion>,
}

impl<T> TripleStack<T> {
    pub fn new(size: usize) -> Self {
        let mut triple = Vec::with_capacity(size);
        triple.resize_with(size, || None::<T>);

        TripleStack {
            triple,
            regions: vec![
                MemRegion {
                    start: 0,
                    end: size / 3,
                    next_index: 0,
                },
                MemRegion {
                    start: (size / 3) + 1,
                    end: (size / 3) * 2,
                    next_index: (size / 3) + 1,
                },
                MemRegion {
                    start: ((size / 3) * 2) + 1,
                    end: size,
                    next_index: ((size / 3) * 2) + 1,
                },
            ],
        }
    }

    pub fn push(&mut self, stack: usize, e: T) {
        let mut mem_reg = &mut self.regions[stack];
        let idx = mem_reg.next_index;
        if idx > mem_reg.end {
            panic!("Out of memory!")
        }

        self.triple[idx] = Some(e);
        mem_reg.next_index = idx + 1;
    }

    pub fn peek(&self, stack: usize) -> Option<&T> {
        let mem_reg = &self.regions[stack];

        self.triple[mem_reg.next_index - 1].as_ref()
    }

    pub fn pop(&mut self, stack: usize) -> Option<T> {
        let mem_reg = &mut self.regions[stack];
        let idx = mem_reg.next_index - 1;

        let e = self.triple[idx].take();
        mem_reg.next_index = idx;
        e
    }

    pub fn size(&self, stack: usize) -> usize {
        let mem_reg = &self.regions[stack];

        mem_reg.next_index - mem_reg.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let mut t_stack: TripleStack<usize> = TripleStack::new(100);
        t_stack.push(0, 1);

        println!("{:?}", t_stack);
        println!("{:?}", t_stack.peek(0));
        println!("{:?}", t_stack.pop(0));
        println!("{:?}", t_stack);

        t_stack.push(2, 30);
        println!("{:?}", t_stack);
    }

    #[test]
    fn should_peep_and_pop_triplet() {
        let mut t_stack: TripleStack<usize> = TripleStack::new(100);
        t_stack.push(0, 1);
        t_stack.push(0, 2);
        t_stack.push(0, 3);

        t_stack.push(1, 11);
        t_stack.push(1, 12);
        t_stack.push(1, 13);

        t_stack.push(2, 21);
        t_stack.push(2, 22);
        t_stack.push(2, 23);

        assert_eq!(3, t_stack.size(0));
        assert_eq!(3, t_stack.size(1));
        assert_eq!(3, t_stack.size(2));

        assert_eq!(&3, t_stack.peek(0).unwrap());
        assert_eq!(&13, t_stack.peek(1).unwrap());
        assert_eq!(&23, t_stack.peek(2).unwrap());

        assert_eq!(3, t_stack.pop(0).unwrap());
        assert_eq!(13, t_stack.pop(1).unwrap());
        assert_eq!(23, t_stack.pop(2).unwrap());

        assert_eq!(&2, t_stack.peek(0).unwrap());
        assert_eq!(&12, t_stack.peek(1).unwrap());
        assert_eq!(&22, t_stack.peek(2).unwrap());

        assert_eq!(2, t_stack.size(0));
        assert_eq!(2, t_stack.size(1));
        assert_eq!(2, t_stack.size(2));
    }
}

// Circular Array: Implement a CircularArray class that supports an array-like data structure which
// can be efficiently rotated. If possible, the class should use a generic type (also called a template), and
// should support iteration via the standard for (Obj o : circularArray) notation.

#[derive(Debug, Clone)]
struct CircularArray<T> {
    array: Vec<Option<T>>,
    size: usize,
}

struct IntoIter<T> {
    c_arr: CircularArray<T>,
    next: usize,
}

impl<T> CircularArray<T> {
    fn new(size: usize) -> Self {
        let mut array = Vec::with_capacity(size);
        array.resize_with(size, || None::<T>);

        CircularArray { array, size }
    }

    fn add(&mut self, index: usize, e: T) {
        self.array[(index) % self.size] = Some(e)
    }

    fn get(&self, index: usize) -> &Option<T> {
        &self.array[(index) % self.size]
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            c_arr: self,
            next: 0,
        }
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.c_arr.size {
            let next = self.c_arr.get(self.next);
            self.next += 1;

            *next
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fill_array() {
        let mut c_arr = CircularArray::new(5);
        c_arr.add(0, 1);
        c_arr.add(1, 2);
        c_arr.add(2, 3);
        c_arr.add(3, 4);
        c_arr.add(4, 5);
        c_arr.add(5, 6);
        c_arr.add(6, 7);

        let expected = vec![Some(6), Some(7), Some(3), Some(4), Some(5)];
        for (i, v) in c_arr.into_iter().enumerate() {
            assert_eq!(expected[i].unwrap(), v);
        }
    }
}

// Animal Shelter: An animal shelter, which holds only dogs and cats, operates on a strictly"first in, first
// out" basis. People must adopt either the "oldest" (based on arrival time) of all animals at the shelter,
// or they can select whether they would prefer a dog or a cat (and will receive the oldest animal of
// that type). They cannot select which specific animal they would like. Create the data structures to
// maintain this system and implement operations such as enqueue, dequeueAny, dequeueDog,
// and dequeueCat.
// You may use the built-in Linkedlist data structure.

// The book is with Java examples that's why (I guess) they have LL in mind, but I'll use VecDeque since it has a lot of API's in common with Java List Interface.

use std::collections::VecDeque;

#[derive(PartialEq, Copy, Clone)]
enum AnimalType {
    DOG,
    CAT,
}

#[derive(Copy, Clone)]
struct Animal {
    a_type: AnimalType,
    name: &'static str,
}

struct AnimalShelter {
    animals: VecDeque<Animal>,
}

impl Animal {
    fn new_dog(name: &'static str) -> Animal {
        Animal {
            a_type: AnimalType::DOG,
            name,
        }
    }

    fn new_cat(name: &'static str) -> Animal {
        Animal {
            a_type: AnimalType::CAT,
            name,
        }
    }
}

impl AnimalShelter {
    fn new() -> Self {
        AnimalShelter {
            animals: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, a: Animal) {
        self.animals.push_front(a)
    }

    fn dequeue_any(&mut self) -> Option<Animal> {
        self.animals.pop_back()
    }

    fn dequeue_dog(&mut self) -> Option<Animal> {
        self.dequeue_last(AnimalType::DOG)
    }

    fn dequeue_cat(&mut self) -> Option<Animal> {
        self.dequeue_last(AnimalType::CAT)
    }

    fn dequeue_last(&mut self, t: AnimalType) -> Option<Animal> {
        if let Some(idx) = self.find_last_idx(t) {
            let last = self.animals[idx];
            self.animals.remove(idx);
            return Some(last);
        }

        None
    }

    fn find_last_idx(&self, t: AnimalType) -> Option<usize> {
        self.animals.iter().rposition(|a| a.a_type == t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_dequeue_any() {
        let mut shelter = AnimalShelter::new();
        shelter.enqueue(Animal::new_cat("Fluffy"));
        shelter.enqueue(Animal::new_dog("Rex"));
        shelter.enqueue(Animal::new_cat("Grumpy"));

        assert_eq!("Fluffy", shelter.dequeue_any().unwrap().name);
        assert_eq!("Rex", shelter.dequeue_any().unwrap().name);
        assert_eq!("Grumpy", shelter.dequeue_any().unwrap().name);
    }

    #[test]
    fn should_dequeue_cat() {
        let mut shelter = AnimalShelter::new();
        shelter.enqueue(Animal::new_cat("Fluffy"));
        shelter.enqueue(Animal::new_dog("Rex"));
        shelter.enqueue(Animal::new_cat("Grumpy"));

        assert_eq!("Fluffy", shelter.dequeue_cat().unwrap().name);
        assert_eq!("Grumpy", shelter.dequeue_cat().unwrap().name);

        // we'll dequeue the dog as well, they all should have a home
        assert_eq!("Rex", shelter.dequeue_dog().unwrap().name);
    }

    #[test]
    fn should_dequeue_dog() {
        let mut shelter = AnimalShelter::new();
        shelter.enqueue(Animal::new_dog("Rex"));
        shelter.enqueue(Animal::new_cat("Grumpy"));
        shelter.enqueue(Animal::new_dog("Laika"));

        assert_eq!("Rex", shelter.dequeue_dog().unwrap().name);
        assert_eq!("Laika", shelter.dequeue_dog().unwrap().name);

        // same as above
        assert_eq!("Grumpy", shelter.dequeue_cat().unwrap().name);
    }
}

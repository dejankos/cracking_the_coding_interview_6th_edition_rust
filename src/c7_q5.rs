// Online Book Reader: Design the data structures for an online book reader system.

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Book {
    name: String,
    author: String,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct User {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct OnlineLibrary {
    books: Vec<Book>,
    users: HashMap<User, HashSet<Book>>,
}

impl OnlineLibrary {
    fn new() -> Self {
        OnlineLibrary {
            books: vec![
                Book {
                    name: String::from("A brief history of time"),
                    author: String::from("Stephen Hawking"),
                },
                Book {
                    name: String::from("Mr Tompkins"),
                    author: String::from("George Gamow"),
                },
                Book {
                    name: String::from("Watchmen"),
                    author: String::from("Alan Moore"),
                },
            ],
            users: HashMap::new(),
        }
    }

    fn reg_user(&mut self, user: User) {
        self.users.insert(user, HashSet::new());
    }

    fn list_books(&self) -> Vec<Book> {
        self.books.clone()
    }

    fn borrow_book(&mut self, user: &User, book: &Book) {
        if let Some(h_set) = self.users.get_mut(user) {
            h_set.insert(book.clone());
        }
    }

    fn currently_reading(&self, user: &User) -> HashSet<Book> {
        self.users[user].clone()
    }

    fn return_book(&mut self, user: &User, book: &Book) {
        if let Some(h_set) = self.users.get_mut(user) {
            h_set.remove(book);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_buy_and_read() {
        let user = User {
            id: 1,
            name: String::from("the one"),
        };

        let mut lib = OnlineLibrary::new();
        lib.reg_user(user.clone());
        let book = &lib.list_books()[0];
        lib.borrow_book(&user, book);
        let current = lib.currently_reading(&user);

        assert!(lib.currently_reading(&user).contains(book));
        assert_eq!(lib.currently_reading(&user).len(), 1);

        lib.return_book(&user, book);

        assert_eq!(lib.currently_reading(&user).len(), 0);
    }
}

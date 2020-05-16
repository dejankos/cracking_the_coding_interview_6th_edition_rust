// Deck of Cards: Design the data structures for a generic deck of cards. Explain how you would
// subclass the data structures to implement blackjack.





use rand::seq::SliceRandom;
use rand::thread_rng;

trait Deck {
    fn new() -> Self
    where
        Self: Sized;
    fn shuffle(&mut self);
    fn deal_next(&mut self) -> Box<dyn Card>;
    fn remaining(&self) -> usize;
}

trait Card {
    fn sign(&self) -> &str;
    fn num(&self) -> u8;
}

struct PokerCard {
    card_type: PokerCardType,
    num: u8,
}

struct PokerDeck {
    cards: Vec<PokerCard>,
}

impl Card for PokerCard {
    fn sign(&self) -> &str {
        stringify!(self.card_type)
    }

    fn num(&self) -> u8 {
        self.num
    }
}

impl Deck for PokerDeck {
    fn new() -> Self {
        let mut cards = vec![];
        for i in 1..14 {
            let num = i as u8;
            cards.push(PokerCard {
                card_type: PokerCardType::Hearth,
                num,
            });
            cards.push(PokerCard {
                card_type: PokerCardType::Club,
                num,
            });
            cards.push(PokerCard {
                card_type: PokerCardType::Diamond,
                num,
            });
            cards.push(PokerCard {
                card_type: PokerCardType::Spade,
                num,
            });
        }

        PokerDeck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    fn deal_next(&mut self) -> Box<dyn Card> {
        if let Some(card) = self.cards.pop() {
            Box::new(card)
        } else {
            panic!("No cards left!")
        }
    }

    fn remaining(&self) -> usize {
        self.cards.len()
    }
}

#[derive(Debug)]
enum PokerCardType {
    Hearth,
    Diamond,
    Club,
    Spade,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_cards() {
        let mut deck = PokerDeck::new();
        deck.shuffle();
        let mut cards = vec![];
        while deck.remaining() > 0 {
            cards.push(deck.deal_next())
        }

        assert_eq!(cards.len(), 52);
        assert_eq!(deck.remaining(), 0);
    }
}

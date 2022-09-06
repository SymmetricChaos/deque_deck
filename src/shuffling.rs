use rand::seq::SliceRandom;
use rand_distr::{Bernoulli, Distribution};

use crate::deck::Deck;

fn bern(p: f64) -> bool {
    let bern = Bernoulli::new(p).unwrap();
    bern.sample(&mut rand::thread_rng())
}

impl<T> Deck<T> {
    /// Consumes two Decks and returns the result of riffling them together using the Gilbert-Shannon-Reeds algorithm. This is a slow and statistically poor quality shuffle that simulates a single riffle shuffle. For good mixing several riffles are needed.
    pub fn from_riffle(mut left: Deck<T>, mut right: Deck<T>) -> Deck<T> {
        let n = left.len() + right.len();
        let mut new: Deck<T> = Deck::with_capacity(n);
        for _ in 0..n {
            // We're going to hope decks are restricted to billions of cards here
            let l = left.len() as f64;
            let r = right.len() as f64;
            if bern(r / (l + r)) {
                match left.draw_top() {
                    Some(card) => new.place_bottom(card),
                    None => {
                        new.extend(right);
                        break;
                    }
                }
            } else {
                match right.draw_top() {
                    Some(card) => new.place_bottom(card),
                    None => {
                        new.extend(left);
                        break;
                    }
                }
            }
        }
        new
    }

    /// Combine the Deck with another deck, in place, via a riffle shuffle.
    pub fn riffle_with(&mut self, mut right: Deck<T>) {
        if self.len() == 0 {
            self.extend(right);
            return;
        }

        let mut l_len = self.len();
        let mut r_len = right.len();
        let mut cursor = 0;

        loop {
            if r_len == 0 {
                break;
            }
            let l = l_len as f64;
            let r = r_len as f64;

            // If the right branch is chosen, place the card at the cursor position
            // and reduce the length of the right side
            if bern(r / (l + r)) {
                match right.draw_top() {
                    Some(card) => {
                        self.place_nth(cursor, card);
                        r_len -= 1;
                    }
                    None => {
                        self.extend(right);
                        break;
                    }
                }
            // If the left branch is chosen just reduce the length of the left size.
            } else {
                l_len -= 1;
            }
            cursor += 1;
        }
    }

    /// Perform a Fisher-Yates shuffle on the deck. This is a mathematically correct shuffle that gives every card an equal chance of ending up at any postion.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }

    /// Perform a single riffle shuffle of the deck. See .riffle() for details.
    pub fn shuffle_riffle(&mut self) {
        let right = self.split_off_binom();
        self.riffle_with(right);
    }

    /// Perform a Gilbreath shuffle on the deck that uses n cards. This is not a fair shuffle.
    pub fn shuffle_gilbreath(&mut self, n: usize) -> Result<(), &'static str> {
        if n > self.len() {
            return Err("n must be less than the number of cards in the deck");
        }
        let mut new: Deck<T> = Deck::with_capacity(n);
        for _ in 0..n {
            new.place_top(self.draw_top().unwrap())
        }
        self.riffle_with(new);

        Ok(())
    }
}

impl<T: Clone> Deck<T> {
    /// Perform a perfect riffle shuffle (aka a faro shuffle). The deck must contain an even number of cards. This is not a fair shuffle.
    pub fn shuffle_faro(&mut self, out: bool) -> Result<(), &'static str> {
        let n = self.cards.len();
        if n % 2 == 0 {
            return Err("a faro shuffle requires an even number of cards");
        } else {
            let mut new: Deck<T> = Deck::with_capacity(n);
            let cards = Deck::from(self.cards.clone());
            let (mut left, mut right) = cards.split_nth(n / 2);
            if out {
                for _ in 0..(n / 2) {
                    new.place_top(left.draw_top().unwrap());
                    new.place_top(right.draw_top().unwrap());
                }
            } else {
                for _ in 0..(n / 2) {
                    new.place_top(right.draw_top().unwrap());
                    new.place_top(left.draw_top().unwrap());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_deck {
    use super::*;
    // #[test]
    // fn riffle() {
    //     for _ in 0..10 {
    //         let left = Deck::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    //         let right = Deck::from([10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    //         println!("{:?}", Deck::riffle(left, right))
    //     }
    // }

    #[test]
    fn riffle_in_place() {
        for _ in 0..10 {
            let mut left = Deck::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let right = Deck::from([10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
            left.riffle_with(right);
            println!("{:?}", left)
        }
    }
}

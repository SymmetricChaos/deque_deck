use rand::{seq::SliceRandom, thread_rng};
use rand_distr::{Bernoulli, Distribution};

use crate::deck::Deck;

fn bern(p: f64) -> bool {
    let bern = Bernoulli::new(p).unwrap();
    bern.sample(&mut rand::thread_rng())
}

impl<T> Deck<T> {
    /// Perform a Fisher-Yates shuffle on the deck. This is a mathematically correct shuffle that gives every card an equal chance of ending up at any postion.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }

    /// Extends the Deck with another and then shuffle the result.
    pub fn shuffle_with(&mut self, right: Deck<T>) {
        self.extend(right);
        self.shuffle();
    }

    /// Perform a single riffle shuffle of the deck using the Gilbert-Shannon-Reeds algorithm. Poor randomization.
    pub fn riffle(&mut self) {
        let right = self.split_off_binom();
        self.riffle_with(right);
    }

    /// Riffle shuffle another Deck into this one, consuming the other Deck.
    pub fn riffle_with(&mut self, mut right: Deck<T>) {
        if self.len() == 0 {
            self.extend(right);
            return;
        }

        let mut l_len = self.len();
        let mut r_len = right.len();
        let mut cursor = 0;

        self.cards.reserve_exact(r_len);

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

    /// Perform a Gilbreath shuffle on the deck that uses n cards. Poor randomization.
    pub fn gilbreath(&mut self, n: usize) -> Result<(), &'static str> {
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

    /// Perform a faro shuffle (a perfect riffle shuffle). An out shuffle places the first card
    /// on top. An in shuffle places the first card second. This is not a true shuffle as it is
    /// entirely deterministic.
    pub fn faro(&mut self, out: bool) {
        let len = self.len();

        let right = self.split_off_nth(len / 2);

        let mut cursor = match out {
            true => 1,
            false => 0,
        };

        for card in right {
            self.place_nth(cursor, card);
            cursor += 2;
        }
    }
}

impl<T: Clone> Deck<T> {
    /// Perform a pile shuffle using n piles. Very poor randomization.
    pub fn pile_shuffle(&mut self, n: usize) {
        let mut decks = vec![Deck::empty(); n];
        let mut ctr = 0;
        for card in self.clone().into_iter() {
            decks[ctr].place_top(card);
            ctr = (ctr + 1) % n;
        }
        decks.shuffle(&mut thread_rng());
        *self = decks.iter().cloned().collect();
    }
}

#[cfg(test)]
mod test_deck {
    use super::*;

    // #[test]
    // fn riffle_in_place() {
    //     for _ in 0..10 {
    //         let mut left = Deck::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    //         let right = Deck::from([10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    //         left.riffle_with(right);
    //         println!("{:?}", left)
    //     }
    // }

    #[test]
    fn faro_in_place() {
        // Out shuffle places the first card on top
        let mut deck = Deck::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        deck.faro(true);
        assert_eq!(deck, Deck::from([0, 5, 1, 6, 2, 7, 3, 8, 4, 9]));

        // In shuffle places the first card in the second position
        let mut deck = Deck::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        deck.faro(false);
        assert_eq!(deck, Deck::from([5, 0, 6, 1, 7, 2, 8, 3, 9, 4]));

        // Check an odd number of cards
        let mut deck = Deck::from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        deck.faro(false);
        assert_eq!(deck, Deck::from([4, 0, 5, 1, 6, 2, 7, 3, 8]));
    }
}

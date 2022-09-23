use rand::{seq::SliceRandom, Rng};

use crate::deck::Deck;

impl<T> Deck<T> {
    /// Perform a Fisher-Yates shuffle on the deck. This is a mathematically correct shuffle that gives every card
    /// an equal chance of ending up at any postion. It should be perferred whenever thorough shuffling is needed.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }

    /// Extends the Deck with another and then shuffles the result.
    pub fn shuffle_with(&mut self, right: Deck<T>) {
        self.extend(right);
        self.shuffle();
    }

    /// Perform a single riffle shuffle of the deck using the Gilbert-Shannon-Reeds algorithm. Poor randomization.
    pub fn riffle(&mut self) {
        let right = self.split_off_binom();
        self.riffle_with(right);
    }

    /// Perform a single riffle shuffle of the deck using the Gilbert-Shannon-Reeds algorithm. Poor randomization.
    pub fn riffle_at_nth(&mut self, n: usize) {
        let right = self.split_off_nth(n);
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
            if self.rng.gen_bool(r / (l + r)) {
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

    /// Performs the inverse of a riffle shuffle. This is equivalent to taking cards at random from the deck (in order) to make a new
    /// deck then placing the remains of the original on top. However this is done without creating an additional deck.
    pub fn inverse_riffle(&mut self) {
        let mut cursor = 0;
        let mut ctr = 0;
        loop {
            if ctr == self.len() {
                break;
            }
            if self.rng.gen_bool(0.5) {
                let card = self
                    .draw_nth(cursor)
                    .expect("cursor should not be out of bounds");
                self.place_bottom(card);
            } else {
                cursor += 1
            }
            ctr += 1
        }
    }

    /// Perform a Gilbreath shuffle on the deck that uses n cards. Poor randomization.
    pub fn gilbreath(&mut self, n: usize) -> Result<(), &'static str> {
        if n > self.len() {
            return Err("n must be less than the number of cards in the deck");
        }
        let new = self.split_off_nth(n);

        self.reverse();
        self.riffle_with(new);

        Ok(())
    }

    /// Perform an overhand shuffle using Pemantle's algorithm
    pub fn overhand(&mut self, p: f64) {
        let len = self.len();
        let temp = self.cards.make_contiguous();
        let mut lo = 0;
        for i in 0..len {
            if self.rng.gen_bool(p) {
                temp[lo..i].reverse();
                lo = i
            }
        }
        self.cards.make_contiguous().reverse()
    }

    /// Premantle's original algorithm. This has similar statistical properties to an overhand shuffle
    /// but does not recreate the shuffle itself. Runs about twice as fast.
    pub fn premantle(&mut self, p: f64) {
        let len = self.len();
        let temp = self.cards.make_contiguous();
        let mut lo = 0;
        for i in 0..len {
            if self.rng.gen_bool(p) {
                temp[lo..i].reverse();
                lo = i
            }
        }
    }

    /// Perform a faro shuffle (a perfect riffle shuffle). An "out shuffle" places the first card
    /// on top. An "in shuffle" places the first card second. This is not a true shuffle as it is
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
        // If n is greather than or equal to the size of the deck
        // it is equivalent to a Fisher-Yates shuffle
        if n >= self.len() {
            self.shuffle();
            return;
        }
        let mut decks = vec![Deck::empty(); n];
        let mut ctr = 0;
        for _ in 0..self.len() {
            decks[ctr].place_top(self.draw_top().unwrap());
            ctr = (ctr + 1) % n;
        }
        decks.shuffle(&mut self.rng);
        *self = Deck::from(decks);
    }
}

#[cfg(test)]
mod test_deck {
    use super::*;

    #[test]
    fn riffle() {
        let mut deck = Deck::<i32>::from_iter(0..10);
        deck.set_seed_u64(314159);
        deck.riffle();
        assert_eq!(deck.cards, [5, 0, 1, 6, 2, 3, 7, 8, 4, 9]);
    }

    #[test]
    fn riffle_with() {
        let mut left = Deck::from_iter(0..10_i32);
        left.set_seed_u64(314159);
        let right = Deck::from([10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
        left.riffle_with(right);
        assert_eq!(
            left.cards,
            [10, 11, 0, 1, 12, 2, 3, 13, 4, 5, 14, 15, 16, 17, 18, 6, 7, 8, 19, 9]
        );
    }

    #[test]
    fn gilbreath() {
        let mut deck = Deck::from_iter(0..10_i32);
        deck.set_seed_u64(314159);
        deck.gilbreath(5).unwrap();
        assert_eq!(deck.cards, [5, 6, 4, 3, 7, 2, 1, 8, 0, 9]);
    }

    #[test]
    fn overhand() {
        let mut deck = Deck::from_iter(0..10_i32);
        deck.set_seed_u64(314159);
        deck.overhand(0.3);
        assert_eq!(deck.cards, [9, 8, 7, 4, 5, 6, 1, 2, 3, 0]);
    }

    #[test]
    fn premantle() {
        let mut deck = Deck::from_iter(0..10_i32);
        deck.set_seed_u64(314159);
        deck.premantle(0.3);
        assert_eq!(deck.cards, [0, 3, 2, 1, 6, 5, 4, 7, 8, 9]);
    }

    #[test]
    fn inv_riffle() {
        let mut deck = Deck::from_iter(0..10_i32);
        deck.set_seed_u64(314159);
        deck.inverse_riffle();
        assert_eq!(deck.cards, [2, 3, 5, 6, 8, 9, 0, 1, 4, 7]);
    }

    #[test]
    fn faro_in_place() {
        // Out shuffle places the first card on top
        let mut deck = Deck::from_iter(0..10_i32);
        deck.faro(true);
        assert_eq!(deck, Deck::from([0, 5, 1, 6, 2, 7, 3, 8, 4, 9]));

        // In shuffle places the first card in the second position
        let mut deck = Deck::from_iter(0..10_i32);
        deck.faro(false);
        assert_eq!(deck, Deck::from([5, 0, 6, 1, 7, 2, 8, 3, 9, 4]));

        // Check an odd number of cards
        let mut deck = Deck::from_iter(0..10_i32);
        deck.faro(false);
        assert_eq!(deck, Deck::from([4, 0, 5, 1, 6, 2, 7, 3, 8]));
    }

    #[test]
    fn pile_shuffle() {
        let mut deck = Deck::from_iter(0..10_i32);
        deck.set_seed_u64(314159);
        deck.pile_shuffle(3);
        assert_eq!(deck.cards, [8, 5, 2, 7, 4, 1, 9, 6, 3, 0]);
    }
}

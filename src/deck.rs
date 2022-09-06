use std::{collections::VecDeque, fmt::Debug};

use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Bernoulli, Binomial, Distribution};

#[derive(Clone, Debug)]
pub struct Deck<T> {
    cards: VecDeque<T>,
}

impl<PlayingCard> Deck<PlayingCard> {
    /// Create a deck of PlayingCard in canonical order.
    pub fn new() -> Deck<PlayingCard> {
        todo!("Create a deck of PlayingCard in canonical order.")
    }
}

impl<T: Debug> Deck<T> {
    fn bern(p: f64) -> bool {
        let bern = Bernoulli::new(p).unwrap();
        bern.sample(&mut rand::thread_rng())
    }

    fn binom(&self) -> usize {
        let bin = Binomial::new(self.cards.len().try_into().unwrap(), 0.5).unwrap();
        usize::try_from(bin.sample(&mut rand::thread_rng())).unwrap()
    }

    fn uniform(&self) -> usize {
        rand::thread_rng().gen_range(0..self.cards.len())
    }

    pub fn empty() -> Deck<T> {
        Deck::from(VecDeque::new())
    }

    pub fn with_capacity(n: usize) -> Deck<T> {
        Deck::from(VecDeque::with_capacity(n))
    }

    /// Consumes two Decks and returns the result of riffling them together using the Gilbert-Shannon-Reeds algorithm. This is a slow and statistically poor quality shuffle that simulates a single riffle shuffle. For good mixing several riffles are needed.
    pub fn from_riffle(mut left: Deck<T>, mut right: Deck<T>) -> Deck<T> {
        let n = left.len() + right.len();
        let mut new: Deck<T> = Deck::with_capacity(n);
        for _ in 0..n {
            // We're going to hope decks are restricted to billions of cards here
            let l = left.len() as f64;
            let r = right.len() as f64;
            if Self::bern(r / (l + r)) {
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
            if Self::bern(r / (l + r)) {
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

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn extend(&mut self, other: Deck<T>) {
        self.cards.extend(other.cards)
    }

    /// Get a reference to the nth card.
    pub fn get(&self, n: usize) -> Option<&T> {
        self.cards.get(n)
    }

    /// Get a mutable reference to the nth card.
    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        self.cards.get_mut(n)
    }

    /// Draw the top card of the deck.
    pub fn draw_top(&mut self) -> Option<T> {
        self.cards.pop_front()
    }

    /// Draw the bottom card of the deck.
    pub fn draw_bottom(&mut self) -> Option<T> {
        self.cards.pop_back()
    }

    /// Draw the nth card of the top. 0 draws the top card.
    pub fn draw_nth(&mut self, n: usize) -> Option<T> {
        self.cards.remove(n)
    }

    /// Draw a uniformly random card from the deck.
    pub fn draw_random(&mut self) -> Option<T> {
        self.draw_nth(self.uniform())
    }

    /// Draw a card from the deck following a binomial distribution.
    pub fn draw_binom(&mut self) -> Option<T> {
        self.draw_nth(self.binom())
    }

    /// Place the card on top of the deck.
    pub fn place_top(&mut self, card: T) {
        self.cards.push_front(card)
    }

    /// Place the card on the bottom of the deck.
    pub fn place_bottom(&mut self, card: T) {
        self.cards.push_back(card)
    }

    /// Place the card in the nth position in the deck. 0 places it on the top.
    pub fn place_nth(&mut self, n: usize, card: T) {
        self.cards.insert(n, card);
    }

    /// Place the card at a random position in the deck.
    pub fn place_random(&mut self, card: T) {
        self.place_nth(self.uniform(), card)
    }

    /// Place a card into the deck following a binomial distribution.
    pub fn place_binom(&mut self, card: T) {
        self.place_nth(self.binom(), card)
    }

    /// Cut the deck at nth position.
    pub fn cut_nth(&mut self, n: usize) {
        self.cards.rotate_left(n)
    }

    /// Cut the deck at a random position.
    pub fn cut_random(&mut self) {
        self.cards.rotate_left(self.uniform())
    }

    /// Cut the deck following a binomial distribution.
    pub fn cut_binom(&mut self) {
        self.cards.rotate_left(self.binom())
    }

    /// Split the deck at the nth position, retaining the top part.
    pub fn split_off_nth(&mut self, n: usize) -> Deck<T> {
        Deck::from(self.cards.split_off(n))
    }

    /// Split the deck at a random position, retaining the top part.
    pub fn split_off_random(&mut self) -> Deck<T> {
        self.split_off_nth(self.uniform())
    }

    /// Split the deck following a binomial distribution, retaining the top part.
    pub fn split_off_binom(&mut self) -> Deck<T> {
        self.split_off_nth(self.binom())
    }

    /// Split the deck at the nth position, consuming it.
    pub fn split_nth(mut self, n: usize) -> (Deck<T>, Deck<T>) {
        let cards = self.cards.split_off(n);
        (Deck::from(self.cards), Deck::from(cards))
    }

    /// Split the deck at a random position, consuming it.
    pub fn split_random(self) -> (Deck<T>, Deck<T>) {
        let n = self.uniform();
        self.split_nth(n)
    }

    /// Split the deck following a binomial distribution, consuming it.
    pub fn split_binom(self) -> (Deck<T>, Deck<T>) {
        let n = self.binom();
        self.split_nth(n)
    }

    /// Perform a Fisher-Yates shuffle on the deck. This is a mathematically correct shuffle that gives every card an equal chance of ending up at any postion.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }
}

impl<T: Clone + Debug> Deck<T> {
    /// Perform a single riffle shuffle of the deck. See .riffle() for details.
    pub fn shuffle_riffle(&mut self) {
        let right = self.split_off_binom();
        self.riffle_with(right);
    }

    /// Perform a perfect riffle shuffle (aka a faro shuffle). The deck must contain an even number of cards. This is not a fair shuffle.
    pub fn shuffle_faro(&mut self, out: bool) -> Result<(), &'static str> {
        let n = self.cards.len();
        if n % 2 == 0 {
            return Err("a faro shuffle requires an even number of cards");
        } else {
            let mut new: Deck<T> = Deck::with_capacity(n);
            let cards = Deck::from(self.cards.clone());
            let (mut left, mut right) = cards.split_binom();
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

impl<T: Ord> Deck<T> {
    pub fn sort(&mut self) {
        self.cards.make_contiguous().sort()
    }
}

impl<T, const N: usize> From<[T; N]> for Deck<T> {
    fn from(arr: [T; N]) -> Self {
        Deck {
            cards: VecDeque::from(arr),
        }
    }
}

impl<T> From<Vec<T>> for Deck<T> {
    fn from(vec: Vec<T>) -> Self {
        Deck {
            cards: VecDeque::from(vec),
        }
    }
}

impl<T> From<VecDeque<T>> for Deck<T> {
    fn from(vec: VecDeque<T>) -> Self {
        Deck {
            cards: VecDeque::from(vec),
        }
    }
}

impl<T> FromIterator<T> for Deck<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();
        let mut cards = VecDeque::with_capacity(lower);
        cards.extend(iterator);
        Deck { cards }
    }
}

impl<T> IntoIterator for Deck<T> {
    type Item = T;

    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

#[cfg(test)]
mod test_deck {
    use super::*;

    #[test]
    fn draw_top() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        assert_eq!(deck.draw_top().unwrap(), 1)
    }

    #[test]
    fn draw_bottom() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        assert_eq!(deck.draw_bottom().unwrap(), 6)
    }

    #[test]
    fn draw_nth() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        assert_eq!(deck.draw_nth(3).unwrap(), 4)
    }

    #[test]
    fn place_top() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        deck.place_top(100);
        assert_eq!(deck.get(0).unwrap(), &100);
    }

    #[test]
    fn place_bottom() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        deck.place_bottom(100);
        assert_eq!(deck.get(6).unwrap(), &100);
    }

    #[test]
    fn place_nth() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        deck.place_nth(3, 100);
        assert_eq!(deck.get(3).unwrap(), &100);
    }

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

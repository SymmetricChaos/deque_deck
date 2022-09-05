use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Binomial, Distribution};

pub struct Deck<T> {
    cards: VecDeque<T>,
}

impl<PlayingCard> Deck<PlayingCard> {
    /// Creates a deck of PlayingCard in canonical order.
    pub fn new() -> Deck<PlayingCard> {
        todo!("Creates a deck of PlayingCard in canonical order.")
    }
}

impl<T> Deck<T> {
    fn binom(&self) -> usize {
        let bin = Binomial::new(self.cards.len().try_into().unwrap(), 0.5).unwrap();
        usize::try_from(bin.sample(&mut rand::thread_rng())).unwrap()
    }

    fn uniform(&self) -> usize {
        rand::thread_rng().gen_range(0..self.cards.len())
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

    /// Draw a card from the deck following a binomial distribution, simulating human selection.
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

    /// Place a card into the deck following a binomial distribution, simulating human selection.
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

    /// Cut the deck following a binomial distribution, simulating human selection.
    pub fn cut_binom(&mut self) {
        self.cards.rotate_left(self.binom())
    }

    /// Split the deck at the nth position. Retains the top of the split and returns the bottom.
    pub fn split_nth(&mut self, n: usize) -> Deck<T> {
        Deck {
            cards: self.cards.split_off(n),
        }
    }

    /// Split the deck at a random position. Retains the top of the split and returns the bottom.
    pub fn split_random(&mut self) -> Deck<T> {
        self.split_nth(self.uniform())
    }

    /// Split the deck following a binomial distribution, simulating human selection. Retains the top of the split and returns the bottom.
    pub fn split_binom(&mut self) -> Deck<T> {
        self.split_nth(self.binom())
    }

    /// Perform a Fisher-Yates shuffle on the deck. This is a mathematically correct shuffle that gives every card an equal chance of ending up at any postion. For a simulated human shuffle see riffle.
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }

    /// Perform a single Bayer-Diaconis riffle of the deck. This is a mathematically poor shuffle that simulates a single riffle shuffle. For good mixing several riffles are needed.
    pub fn riffle(&mut self) {
        let n = self.split_binom();
        todo!()
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
}

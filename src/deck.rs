use rand::{Rng, SeedableRng};
use rand_distr::{Binomial, Distribution};
use rand_xoshiro::Xoroshiro128PlusPlus;
use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, PartialEq)]
/// A deque_deck Deck is simply a VecDeque and an RNG
pub struct Deck<T> {
    pub cards: VecDeque<T>,
    pub(crate) rng: Xoroshiro128PlusPlus,
}

impl<T> Deck<T> {
    // This is used to simulate a human making a selection of where to cut a deck. Strictly speaking humans
    // are much more biased than a binomial distribution. However for a 52 card deck there is a 99.98%
    // probability of selecting in the middle half and a 93% chance of selecting in the middle quarter of
    // the deck.
    pub(crate) fn binom(&mut self) -> usize {
        let bin = Binomial::new(self.cards.len().try_into().unwrap(), 0.5).unwrap();
        usize::try_from(bin.sample(&mut self.rng)).unwrap()
    }

    pub(crate) fn uniform(&mut self) -> usize {
        self.rng.gen_range(0..self.cards.len())
    }

    pub(crate) fn bern(&mut self, p: f64) -> bool {
        self.rng.gen_bool(p)
    }

    /// Create an empty deck.
    pub fn empty() -> Deck<T> {
        Deck::from(VecDeque::new())
    }

    /// Create an empty deck with a specific amount of capacity allocated.
    pub fn with_capacity(n: usize) -> Deck<T> {
        Deck::from(VecDeque::with_capacity(n))
    }

    /// Supply 128 bits of state for the RNG
    pub fn set_seed(&mut self, seed: [u8; 16]) {
        self.rng = Xoroshiro128PlusPlus::from_seed(seed)
    }

    /// Seed the internal RNG from a u64.
    pub fn set_seed_u64(&mut self, seed: u64) {
        self.rng = Xoroshiro128PlusPlus::seed_from_u64(seed)
    }

    // /// Jump the internal RNG forward by 2^64 steps.
    // pub fn jump(&mut self) {
    //     self.rng.jump()
    // }

    /// Number of cards in the Deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Append the Deck with the cards of another Deck, consuming the other. The other deck is placed below this one.
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

    /// A front to back iterator of references.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.cards.iter()
    }

    /// A front to back iterator of mutable references.
    pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<T> {
        self.cards.iter_mut()
    }

    /// Swap cards i and j, returns an error if either is out of bounds.
    pub fn swap(&mut self, i: usize, j: usize) -> Result<(), &'static str> {
        if i >= self.len() || j >= self.len() {
            return Err("index out of bounds");
        }
        self.cards.swap(i, j);
        Ok(())
    }

    /// Reverse the order of the entire deck.
    pub fn reverse(&mut self) {
        self.cards.make_contiguous().reverse()
    }

    /// A reference to the top card.
    pub fn top(&self) -> Option<&T> {
        self.cards.front()
    }

    /// A mutable reference to the top card.
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.cards.front_mut()
    }

    /// A reference to the bottom card.
    pub fn bottom(&self) -> Option<&T> {
        self.cards.back()
    }

    /// A mutable reference to the bottom card.
    pub fn bottom_mut(&mut self) -> Option<&mut T> {
        self.cards.back_mut()
    }

    /// Draw the top card of the deck. Alias for .draw_top() as this is a common operation.
    pub fn draw(&mut self) -> Option<T> {
        self.cards.pop_front()
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
        let n = self.uniform();
        self.draw_nth(n)
    }

    /// Draw a card from the deck following a binomial distribution.
    pub fn draw_binom(&mut self) -> Option<T> {
        let n = self.binom();
        self.draw_nth(n)
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
        let n = self.uniform();
        self.place_nth(n, card)
    }

    /// Place a card into the deck following a binomial distribution.
    pub fn place_binom(&mut self, card: T) {
        let n = self.binom();
        self.place_nth(n, card)
    }

    /// Cut the deck at nth position.
    pub fn cut_nth(&mut self, n: usize) {
        self.cards.rotate_left(n)
    }

    /// Cut the deck at a random position.
    pub fn cut_random(&mut self) {
        let n = self.uniform();
        self.cards.rotate_left(n)
    }

    /// Cut the deck following a binomial distribution.
    pub fn cut_binom(&mut self) {
        let n = self.binom();
        self.cards.rotate_left(n)
    }

    /// Split the deck at the nth position, retaining the top part.
    pub fn split_off_nth(&mut self, n: usize) -> Deck<T> {
        Deck::from(self.cards.split_off(n))
    }

    /// Split the deck at a random position, retaining the top part.
    pub fn split_off_random(&mut self) -> Deck<T> {
        let n = self.uniform();
        self.split_off_nth(n)
    }

    /// Split the deck following a binomial distribution, retaining the top part.
    pub fn split_off_binom(&mut self) -> Deck<T> {
        let n = self.binom();
        self.split_off_nth(n)
    }

    /// Split the deck at the nth position, consuming it.
    pub fn split_nth(mut self, n: usize) -> (Deck<T>, Deck<T>) {
        let cards = self.cards.split_off(n);
        (Deck::from(self.cards), Deck::from(cards))
    }

    /// Split the deck at a random position, consuming it.
    pub fn split_random(mut self) -> (Deck<T>, Deck<T>) {
        let n = self.uniform();
        self.split_nth(n)
    }

    /// Split the deck following a binomial distribution, consuming it.
    pub fn split_binom(mut self) -> (Deck<T>, Deck<T>) {
        let n = self.binom();
        self.split_nth(n)
    }

    /// Perform a cycle permutation on the cards of the Deck. If any index
    /// is invalid an error is returned before any swaps are made.
    pub fn cycle(&mut self, cycle: &[usize]) -> Result<(), &'static str> {
        for pos in cycle {
            if *pos >= self.len() {
                return Err("index out of bounds");
            }
        }
        for pair in cycle.windows(2) {
            self.swap(pair[0], pair[1]).unwrap()
        }
        Ok(())
    }
}

impl<T: Display> Display for Deck<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = format!("[{}", self.cards[0]);
        for card in self.cards.iter().skip(1) {
            list.push_str(", ");
            list.push_str(&card.to_string());
        }
        write!(f, "{}]", list)
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
            rng: Xoroshiro128PlusPlus::from_entropy(),
        }
    }
}

impl<T> From<Vec<T>> for Deck<T> {
    fn from(vec: Vec<T>) -> Self {
        Deck {
            cards: VecDeque::from(vec),
            rng: Xoroshiro128PlusPlus::from_entropy(),
        }
    }
}

impl<T> From<VecDeque<T>> for Deck<T> {
    fn from(vec: VecDeque<T>) -> Self {
        Deck {
            cards: VecDeque::from(vec),
            rng: Xoroshiro128PlusPlus::from_entropy(),
        }
    }
}

impl<T> FromIterator<T> for Deck<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();
        let mut cards = VecDeque::with_capacity(lower);
        cards.extend(iterator);
        Deck {
            cards,
            rng: Xoroshiro128PlusPlus::from_entropy(),
        }
    }
}

/// Gather a Vec of several Decks into a single deck.
impl<T> From<Vec<Deck<T>>> for Deck<T> {
    fn from(vec: Vec<Deck<T>>) -> Self {
        let mut out = Deck::with_capacity(vec.iter().map(|d| d.len()).sum());
        for deck in vec {
            out.extend(deck)
        }
        out
    }
}

/// Gather an Iterator of several Decks into a single deck.
impl<T> FromIterator<Deck<T>> for Deck<T> {
    fn from_iter<I: IntoIterator<Item = Deck<T>>>(iter: I) -> Self {
        let mut out = Deck::empty();
        for deck in iter.into_iter() {
            out.extend(deck)
        }
        out
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
        let mut deck = Deck::from_iter(1..=6);
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

    #[test]
    fn cut_nth() {
        let mut deck = Deck::from([1, 2, 3, 4, 5, 6]);
        deck.cut_nth(3);
        assert_eq!(deck, Deck::from([4, 5, 6, 1, 2, 3]));
    }

    #[test]
    fn cycle() {
        let mut deck = Deck::from([0, 1, 2]);
        deck.cycle(&[0, 1, 2]).unwrap();
        assert_eq!(deck, Deck::from([1, 2, 0]));
    }

    #[test]
    fn reverse() {
        let mut deck = Deck::from_iter(0..=9);
        deck.reverse();
        assert_eq!(deck, Deck::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
    }
}

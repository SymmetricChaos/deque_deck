use crate::deck::Deck;

use self::playing_card::PlayingCard;

pub mod playing_card;
pub mod rank;
pub mod suit;

impl<T> Deck<T> {
    /// Create a deck of PlayingCard in the canonical "new deck" order.
    pub fn new() -> Deck<PlayingCard> {
        let mut deck = Deck::with_capacity(52);
        let rank_chars = "A23456789TJQK";

        for suit in ['H', 'C'] {
            for r in rank_chars.chars() {
                let card = PlayingCard::try_from((r, suit)).unwrap();
                deck.place_top(card)
            }
        }

        for suit in ['D', 'S'] {
            for r in rank_chars.chars().rev() {
                let card = PlayingCard::try_from((r, suit)).unwrap();
                deck.place_top(card)
            }
        }

        deck
    }
}

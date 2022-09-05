use std::fmt::Display;

use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, Copy, Clone)]
pub struct PlayingCard {
    rank: Rank,
    suit: Suit,
}

impl PlayingCard {
    /// The full English name of the card.
    pub fn name(&self) -> String {
        format!("{} of {}", self.rank.name(), self.suit.name())
    }

    /// The standard character pair that represents the rank and suit.
    fn pair(&self) -> (char, char) {
        (self.rank.symbol(), self.suit.symbol())
    }

    /// The character pair that represents the rank and suit, but using the Unicode black suit symbol.
    fn pair_unicode(&self) -> (char, char) {
        (self.rank.symbol(), self.suit.unicode())
    }

    /// Tthe Unicode character for this card.
    fn unicode(&self) -> char {
        self.suit.cards()[self.rank as usize]
    }
}

impl TryFrom<(char, char)> for PlayingCard {
    type Error = &'static str;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        let rank = Rank::try_from(value.0)?;
        let suit = Suit::try_from(value.1)?;
        Ok(PlayingCard { rank, suit })
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.pair().0, self.pair().1)
    }
}

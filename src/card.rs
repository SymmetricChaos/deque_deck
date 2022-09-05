use rank::Rank;
use suit::Suit;

#[derive(Debug, Copy, Clone)]
pub struct PlayingCard {
    rank: Rank,
    suit: Suit,
}

impl PlayingCard {
    /// The full English name of the card.
    fn name(&self) -> &'static str {
        match self.rank {
            Rank::Ace => match self.suit {
                Suit::Clubs => "Ace of Clubs",
                Suit::Hearts => "Ace of Hearts",
                Suit::Spades => "Ace of Spades",
                Suit::Diamonds => "Ace of Diamonds",
            },
            Rank::One => match self.suit {
                Suit::Clubs => "One of Clubs",
                Suit::Hearts => "One of Hearts",
                Suit::Spades => "One of Spades",
                Suit::Diamonds => "One of Diamonds",
            },
            Rank::Two => match self.suit {
                Suit::Clubs => "Two of Clubs",
                Suit::Hearts => "Two of Hearts",
                Suit::Spades => "Two of Spades",
                Suit::Diamonds => "Two of Diamonds",
            },
            Rank::Three => match self.suit {
                Suit::Clubs => "Three of Clubs",
                Suit::Hearts => "Three of Hearts",
                Suit::Spades => "Three of Spades",
                Suit::Diamonds => "Three of Diamonds",
            },
            Rank::Four => match self.suit {
                Suit::Clubs => "Four of Clubs",
                Suit::Hearts => "Four of Hearts",
                Suit::Spades => "Four of Spades",
                Suit::Diamonds => "Four of Diamonds",
            },
            Rank::Five => match self.suit {
                Suit::Clubs => "Five of Clubs",
                Suit::Hearts => "Five of Hearts",
                Suit::Spades => "Five of Spades",
                Suit::Diamonds => "Five of Diamonds",
            },
            Rank::Six => match self.suit {
                Suit::Clubs => "Six of Clubs",
                Suit::Hearts => "Six of Hearts",
                Suit::Spades => "Six of Spades",
                Suit::Diamonds => "Six of Diamonds",
            },
            Rank::Seven => match self.suit {
                Suit::Clubs => "Seven of Clubs",
                Suit::Hearts => "Seven of Hearts",
                Suit::Spades => "Seven of Spades",
                Suit::Diamonds => "Seven of Diamonds",
            },
            Rank::Eight => match self.suit {
                Suit::Clubs => "Eight of Clubs",
                Suit::Hearts => "Eight of Hearts",
                Suit::Spades => "Eight of Spades",
                Suit::Diamonds => "Eight of Diamonds",
            },
            Rank::Nine => match self.suit {
                Suit::Clubs => "Nine of Clubs",
                Suit::Hearts => "Nine of Hearts",
                Suit::Spades => "Nine of Spades",
                Suit::Diamonds => "Nine of Diamonds",
            },
            Rank::Ten => match self.suit {
                Suit::Clubs => "Ten of Clubs",
                Suit::Hearts => "Ten of Hearts",
                Suit::Spades => "Ten of Spades",
                Suit::Diamonds => "Ten of Diamonds",
            },
            Rank::Jack => match self.suit {
                Suit::Clubs => "Jack of Clubs",
                Suit::Hearts => "Jack of Hearts",
                Suit::Spades => "Jack of Spades",
                Suit::Diamonds => "Jack of Diamonds",
            },
            Rank::Queen => match self.suit {
                Suit::Clubs => "Queen of Clubs",
                Suit::Hearts => "Queen of Hearts",
                Suit::Spades => "Queen of Spades",
                Suit::Diamonds => "Queen of Diamonds",
            },
            Rank::King => match self.suit {
                Suit::Clubs => "King of Clubs",
                Suit::Hearts => "King of Hearts",
                Suit::Spades => "King of Spades",
                Suit::Diamonds => "King of Diamonds",
            },
        }
    }

    /// Return the character pair that represents the rank and suit.
    fn pair(&self) -> (char, char) {
        (self.rank.symbol(), self.suit.symbol())
    }

    /// Return the Unicode character for this card.
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

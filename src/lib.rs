use std::collections::VecDeque;

use rand::{Rng, random};


#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

impl Suit {

    pub const CLUBS: [char; 14] =    ['🂡', '🂢', '🂣', '🂤', '🂥', '🂦', '🂧', '🂨', '🂩', '🂪', '🂫', '🂬', '🂭', '🂮'];
    pub const HEARTS: [char; 14] =   ['🂱', '🂲', '🂳', '🂴', '🂵', '🂶', '🂷', '🂸', '🂹', '🂺', '🂻', '🂼', '🂽', '🂾'];
    pub const DIAMONDS: [char; 14] = ['🃁', '🃂', '🃃', '🃄', '🃅', '🃆', '🃇', '🃈', '🃉', '🃊', '🃋', '🃌', '🃍', '🃎'];
    pub const SPADES: [char; 14] =   ['🃑', '🃒', '🃓', '🃔', '🃕', '🃖', '🃗', '🃘', '🃙', '🃚', '🃛', '🃜', '🃝', '🃞'];

    pub fn symbol(&self) -> char {
        match self {
            Suit::Clubs => 'C',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
            Suit::Diamonds => 'D',
        }
    }

    pub fn cards(&self) -> &'static [char; 14] {
        match self {
            Suit::Clubs => &Suit::CLUBS,
            Suit::Hearts => &Suit::HEARTS,
            Suit::Diamonds => &Suit::DIAMONDS,
            Suit::Spades => &Suit::SPADES,
        }
        
    }
}

impl TryFrom<char> for Suit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'C' => Suit::Clubs,
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            'S' => Suit::Spades,
            _ => return Err("invalid char for suit")
        })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl Rank {
    pub fn symbol(&self) -> char {
        match self {
            Rank::Ace => 'A',
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Rank::Ace,
            '1' => Rank::One,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            _ => return Err("invalid char for rank")
        })
    }
}


#[derive(Debug, Copy, Clone)]
pub struct PlayingCard{
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
    fn pair(&self) -> (char,char) {
        (self.rank.symbol(), self.suit.symbol())
    }

    /// Return the Unicode character for this card.
    fn unicode(&self) -> char {
        self.suit.cards()[self.rank as usize]
    }
}

impl TryFrom<(char,char)> for PlayingCard {
    type Error = &'static str;

    fn try_from(value: (char,char)) -> Result<Self, Self::Error> {
        let rank = Rank::try_from(value.0)?;
        let suit = Suit::try_from(value.1)?;
        Ok(PlayingCard{rank,suit})
    }
}



pub struct Deck {
    cards: VecDeque<PlayingCard>
}

impl Deck {

    pub fn new() -> Deck {
        Deck { 
            cards: VecDeque::from(
                []
            )
        }
    }

    /// Draw the top card of the deck.
    pub fn draw_top(&mut self) -> Option<PlayingCard> {
        self.cards.pop_front()
    }

    /// Draw the bottom card of the deck.
    pub fn draw_bottom(&mut self) -> Option<PlayingCard> {
        self.cards.pop_back()
    }

    /// Draw the nth card of the top. 0 draws the top card.
    pub fn draw_nth(&mut self, n: usize) -> Option<PlayingCard> {
        self.cards.remove(n)
    }


    /// Draw a random card from the deck.
    pub fn draw_random(&mut self) -> Option<PlayingCard> {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..self.cards.len());
        self.draw_nth(n)
    }



    /// Place the card on top of the deck.
    pub fn place_top(&mut self, card: PlayingCard) {
        self.cards.push_front(card)
    }

    /// Place the card on the bottom of the deck.
    pub fn place_bottom(&mut self, card: PlayingCard) {
        self.cards.push_back(card)
    }
    
    /// Place the card in the nth position in the deck. 0 places it on the top.
    pub fn place_nth(&mut self, n: usize, card: PlayingCard) {
        self.cards.insert(n, card);
    }

    /// Place the card at a random position in the deck.
    pub fn place_random(&mut self, card: PlayingCard) {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..self.cards.len());
        self.place_nth(n, card)
    }



    /// Shuffle the deck.
    pub fn shuffle(&mut self) {
        self.shuffle()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

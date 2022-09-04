
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


#[derive(Debug, Copy, Clone)]
pub struct PlayingCard{
    rank: Rank,
    suit: Suit,
}

impl PlayingCard {

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

    fn pair(&self) -> (char,char) {
        (self.rank.symbol(),self.suit.symbol())
    }

    fn unicode(&self) -> char {
        self.suit.cards()[self.rank as usize]
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

/// The suit of a card.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

impl Suit {
    pub const CLUBS: [char; 14] = [
        '🃑', '🃒', '🃓', '🃔', '🃕', '🃖', '🃗', '🃘', '🃙', '🃚', '🃛', '🃜', '🃝', '🃞',
    ];
    pub const HEARTS: [char; 14] = [
        '🂱', '🂲', '🂳', '🂴', '🂵', '🂶', '🂷', '🂸', '🂹', '🂺', '🂻', '🂼', '🂽', '🂾',
    ];
    pub const DIAMONDS: [char; 14] = [
        '🃁', '🃂', '🃃', '🃄', '🃅', '🃆', '🃇', '🃈', '🃉', '🃊', '🃋', '🃌', '🃍', '🃎',
    ];
    pub const SPADES: [char; 14] = [
        '🂡', '🂢', '🂣', '🂤', '🂥', '🂦', '🂧', '🂨', '🂩', '🂪', '🂫', '🂬', '🂭', '🂮',
    ];

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
            _ => return Err("invalid char for suit"),
        })
    }
}

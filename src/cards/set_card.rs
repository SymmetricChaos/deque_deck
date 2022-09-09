use std::fmt::Display;

use crate::deck::Deck;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetColor {
    Red,
    Green,
    Purple,
    Custom(&'static str),
}

impl Display for SetColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetColor::Red => write!(f, "Red"),
            SetColor::Green => write!(f, "Green"),
            SetColor::Purple => write!(f, "Purple"),
            SetColor::Custom(color) => write!(f, "{}", color),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetShading {
    Solid,
    Striped,
    Open,
    Custom(&'static str),
}

impl Display for SetShading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetShading::Solid => write!(f, "Solid"),
            SetShading::Striped => write!(f, "Striped"),
            SetShading::Open => write!(f, "Open"),
            SetShading::Custom(shading) => write!(f, "{}", shading),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetShape {
    Diamond,
    Squiggle,
    Oval,
    Custom(&'static str),
}

impl Display for SetShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetShape::Diamond => write!(f, "Diamond"),
            SetShape::Squiggle => write!(f, "Squiggle"),
            SetShape::Oval => write!(f, "Oval"),
            SetShape::Custom(shape) => write!(f, "{}", shape),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetNumber {
    One,
    Two,
    Three,
    Custom(&'static str),
}

impl Display for SetNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetNumber::One => write!(f, "One"),
            SetNumber::Two => write!(f, "Two"),
            SetNumber::Three => write!(f, "Three"),
            SetNumber::Custom(number) => write!(f, "{}", number),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetCard {
    color: SetColor,
    shading: SetShading,
    shape: SetShape,
    number: SetNumber,
}

impl Display for SetCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.color, self.shading, self.shape, self.number
        )
    }
}

impl Deck<SetCard> {
    /// Create a deck of standard Set cards.
    pub fn new() -> Deck<SetCard> {
        let mut deck = Deck::with_capacity(81);

        for color in [SetColor::Red, SetColor::Green, SetColor::Purple] {
            for shading in [SetShading::Solid, SetShading::Striped, SetShading::Open] {
                for shape in [SetShape::Diamond, SetShape::Squiggle, SetShape::Oval] {
                    for number in [SetNumber::One, SetNumber::Two, SetNumber::Three] {
                        deck.place_top(SetCard {
                            color,
                            shading,
                            shape,
                            number,
                        })
                    }
                }
            }
        }

        deck
    }
}

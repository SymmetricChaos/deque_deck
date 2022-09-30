#[cfg(test)]
mod test_deck {
    use crate::deck::Deck;

    const NUM_CARDS: usize = 52;
    const NUM_TRIALS: usize = 50_000;

    use std::{fs::File, io::Write};

    #[test]
    fn shuffle_speed() {
        let mut f = File::create("speed.txt").unwrap();

        writeln!(
            f,
            "Shuffle a deck of {} cards {} times.",
            NUM_CARDS, NUM_TRIALS
        )
        .unwrap();

        let mut deck = Deck::from_iter(0..NUM_CARDS);
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.shuffle()
        }

        writeln!(f, "Fisher-Yates: {:?}", time.elapsed().unwrap()).unwrap();

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.riffle()
        }
        writeln!(f, "Riffle: {:?}", time.elapsed().unwrap()).unwrap();

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.overhand(0.3)
        }
        writeln!(f, "Overhand: {:?}", time.elapsed().unwrap()).unwrap();

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.premantle(0.3)
        }
        writeln!(f, "Premantle: {:?}", time.elapsed().unwrap()).unwrap();

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.pile_shuffle(5)
        }
        writeln!(f, "Pile Shuffle: {:?}", time.elapsed().unwrap()).unwrap();
    }
}

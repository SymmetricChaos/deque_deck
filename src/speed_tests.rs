#[cfg(test)]
mod test_deck {
    use crate::deck::Deck;

    const NUM_CARDS: usize = 52;
    const NUM_TRIALS: usize = 50_000;

    use std::{fs::File, io::Write};

    #[test]
    fn shuffle_speed() {
        // Speed tests write to file so we can see result with release mode
        // The outputs.txt file is used to save information and ensure nothing is optimized away by the compiler
        let mut f = File::create("speed.txt").unwrap();
        let mut o = File::create("outputs.txt").unwrap();

        writeln!(
            f,
            "Shuffle a deck of {} cards {} times.",
            NUM_CARDS, NUM_TRIALS
        )
        .unwrap();

        let mut deck = Deck::from_iter(0..NUM_CARDS);
        let mut firsts = Vec::with_capacity(NUM_TRIALS);
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.shuffle();
            firsts.push(deck.top().unwrap().clone())
        }
        let elapsed = time.elapsed().unwrap();
        writeln!(f, "Fisher-Yates: {:?}", elapsed).unwrap();
        writeln!(o, "{:?}", firsts).unwrap();

        firsts.clear();
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.riffle();
            firsts.push(deck.top().unwrap().clone())
        }
        let elapsed = time.elapsed().unwrap();
        writeln!(f, "Riffle: {:?}", elapsed).unwrap();
        writeln!(o, "{:?}", firsts).unwrap();

        firsts.clear();
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.overhand(0.3);
            firsts.push(deck.top().unwrap().clone())
        }
        let elapsed = time.elapsed().unwrap();
        writeln!(f, "Overhand: {:?}", elapsed).unwrap();
        writeln!(o, "{:?}", firsts).unwrap();

        firsts.clear();
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.premantle(0.3);
            firsts.push(deck.top().unwrap().clone())
        }
        let elapsed = time.elapsed().unwrap();
        writeln!(f, "Premantle: {:?}", elapsed).unwrap();
        writeln!(o, "{:?}", firsts).unwrap();

        firsts.clear();
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.pile_shuffle(5);
            firsts.push(deck.top().unwrap().clone())
        }
        let elapsed = time.elapsed().unwrap();
        writeln!(f, "Pile: {:?}", elapsed).unwrap();
        writeln!(o, "{:?}", firsts).unwrap();
    }
}

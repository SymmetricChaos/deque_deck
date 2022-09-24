#[cfg(test)]
mod test_deck {
    use crate::deck::Deck;

    const NUM_CARDS: usize = 52;
    const NUM_TRIALS: usize = 50_000;

    #[test]
    fn shuffle_speed() {
        println!("Suffle a deck of {} cards {} times.", NUM_CARDS, NUM_TRIALS);

        let mut deck = Deck::from_iter(0..NUM_CARDS);
        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.shuffle()
        }
        println!("Fisher-Yates: {:?}", time.elapsed().unwrap());

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.riffle()
        }
        println!("Riffle: {:?}", time.elapsed().unwrap());

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.overhand(0.3)
        }
        println!("Overhand: {:?}", time.elapsed().unwrap());

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.premantle(0.3)
        }
        println!("Premantle: {:?}", time.elapsed().unwrap());

        let time = std::time::SystemTime::now();
        for _ in 0..NUM_TRIALS {
            deck.pile_shuffle(5)
        }
        println!("Pile Shuffle: {:?}", time.elapsed().unwrap());
    }
}

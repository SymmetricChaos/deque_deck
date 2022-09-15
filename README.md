# deque_deck

This crate provides a simple interface for creating and manipulating a fully generic deck of cards.

## Usage

Add this to your `Cargo.toml`:
 
```toml
[dependencies]
deque_deck = "0.1.0"
```

Standard manipulations of a deque data structure are available but for convenience have been renamed to reflect the language used for decks of cards. So "draw" is used in place of "pop" and "cut" is used in place of "left_rotate" and so on. 

## Shuffling
Various shuffling algorithms are supported. If quality of randomization is needed only the shuffle() methods should be used. All other methods exist either to simulate physical methods of shuffling or are just of interest mathematically.

### Standard (Fisher-Yates)
The default shuffling method uses the Fisher-Yates algorithm which gives every card an equal probabilty of ending up at any position.

```rust
let mut deck = Deck::from([0,1,2,3,4,5,6,7,8,9]);
deck.shuffle();
// Deck{ [1, 9, 7, 6, 2, 8, 4, 5, 0, 3] }
```

### Riffle (Gibert-Shannon-Reeds)
The riffle method uses the Gibert-Shannon-Reeds algorithm to pick a point near the middle of the deck (using a binomial distribution), then split the deck and riffle the two halves together.

 ```rust
let mut deck = Deck::from([0,1,2,3,4,5,6,7,8,9]);
deck.riffle();
// Deck{ [5, 6, 0, 7, 1, 8, 2, 3, 4, 9] }
```

### Overhand (Pemantle)
The overhand method uses Pemantle’s algorithm to take packets of cards from the top and place them successively on the bottom.

 ```rust
let mut deck = Deck::from([0,1,2,3,4,5,6,7,8,9]);
deck.overhand();
// Deck{ [9, 7, 8, 6, 3, 4, 5, 1, 2, 0] }
```

### Other Shuffles
The pile shuffle and Gilbreath shuffle are available to simulate those less common methods. The inverse riffle shuffle and the original Premantle shuffle are available for statistical puroposes but do not represent any real-world shuffling technique. The faro is also provided as, althrough it is not a true shuffle, it is closely related to the riffle shuffle.
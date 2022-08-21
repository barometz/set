//! The rules of the game. Provides cards, a self-shuffling deck, and set
//! validation.

use rand::seq::SliceRandom;
use std::collections::HashSet;
pub mod card;

pub struct Deck(Vec<card::Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut rng = rand::thread_rng();
        let mut cards: Vec<card::Card> = all_cards().into_iter().collect();
        cards.shuffle(&mut rng);
        Deck(cards)
    }

    pub fn deal(&mut self) -> Option<[card::Card; 3]> {
        if self.empty() {
            None
        } else {
            Some([
                self.0.pop().unwrap(),
                self.0.pop().unwrap(),
                self.0.pop().unwrap(),
            ])
        }
    }

    pub fn empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}

fn all_cards() -> HashSet<card::Card> {
    let mut cards = HashSet::<card::Card>::new();

    for color in [card::Color::Green, card::Color::Purple, card::Color::Red] {
        for count in [card::Count::One, card::Count::Two, card::Count::Three] {
            for shape in [card::Shape::Diamond, card::Shape::Oval, card::Shape::Wave] {
                for fill in [card::Fill::Dashed, card::Fill::Full, card::Fill::Open] {
                    cards.insert(card::Card {
                        color,
                        count,
                        shape,
                        fill,
                    });
                }
            }
        }
    }
    cards
}

pub fn check(set: [card::Card; 3]) -> bool {
    let colors: HashSet<card::Color> = set.map(|c| c.color).into();
    let counts: HashSet<card::Count> = set.map(|c| c.count).into();
    let shapes: HashSet<card::Shape> = set.map(|c| c.shape).into();
    let fills: HashSet<card::Fill> = set.map(|c| c.fill).into();

    let valid = |len| len == 1 || len == 3;
    valid(colors.len()) && valid(counts.len()) && valid(shapes.len()) && valid(fills.len())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn card_counter() {
        let cards = super::all_cards();
        assert_eq!(cards.len(), 81);
    }

    #[test]
    fn valid() {
        use super::card::*;
        // All the same. This doesn't exist, but check anyway.
        assert!(super::check([
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
        ]));
        // All different.
        assert!(super::check([
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Purple, Count::Two, Shape::Diamond, Fill::Dashed),
            Card::new(Color::Red, Count::Three, Shape::Wave, Fill::Full),
        ]));
        // Some all different, some all the same.
        assert!(super::check([
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Purple, Count::One, Shape::Diamond, Fill::Open),
            Card::new(Color::Red, Count::One, Shape::Wave, Fill::Open),
        ]));
    }

    #[test]
    fn invalid() {
        use super::card::*;
        assert!(!super::check([
            Card::new(Color::Red, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::Two, Shape::Oval, Fill::Open),
        ]));
        assert!(!super::check([
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::Two, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
        ]));
        assert!(!super::check([
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Wave, Fill::Open),
        ]));
        assert!(!super::check([
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Open),
            Card::new(Color::Green, Count::One, Shape::Oval, Fill::Dashed),
            Card::new(Color::Green, Count::Two, Shape::Oval, Fill::Open),
        ]));
    }

    #[test]
    fn deck() {
        let mut deals = HashSet::<[super::card::Card; 3]>::new();
        let mut deck = super::Deck::new();
        for _ in 0..27 {
            deals.insert(deck.deal().unwrap());
        }
        // A deck can deal three cards 27 times.
        assert!(deck.deal().is_none());
        // And all the dealt sets of three are unique.
        assert_eq!(deals.len(), 27);
    }
}

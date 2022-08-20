use std::collections::HashSet;

mod card;

pub fn all_cards() -> HashSet<card::Card> {
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
}

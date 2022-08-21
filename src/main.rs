use std::collections::HashSet;
use std::hash::Hash;

use set::card::Card;

fn print_table(table: &HashSet<Card>) {
    for card in table.iter() {
        print!("{}, ", card);
    }
    println!();
}

// Try all combinations of three cards.
fn find_set_bruteforce(table: &HashSet<Card>) -> Option<[Card; 3]> {
    for (position, card1) in table.iter().enumerate() {
        for card2 in table.iter().skip(position + 1) {
            for card3 in table.iter().skip(position + 2) {
                if set::check([*card1, *card2, *card3]) {
                    return Some([*card1, *card2, *card3]);
                }
            }
        }
    }
    None
}

fn next_property<Property: Eq + Hash + Copy>(
    property1: Property,
    property2: Property,
    all_properties: &HashSet<Property>,
) -> Property {
    if property1 == property2 {
        property1
    } else {
        *all_properties
            .difference(&HashSet::from([property1, property2]))
            .next()
            .expect("Not enough properties")
    }
}

/// Find a set by taking any two cards, determining what third card you need,
/// and finding that.
fn find_set_third_card(table: &HashSet<Card>) -> Option<[Card; 3]> {
    // this feels like a lazy-static shaped problem
    let colors: HashSet<set::card::Color> = set::card::COLORS.into();
    let counts: HashSet<set::card::Count> = set::card::COUNTS.into();
    let shapes: HashSet<set::card::Shape> = set::card::SHAPES.into();
    let fills: HashSet<set::card::Fill> = set::card::FILLS.into();

    for (position, card1) in table.iter().enumerate() {
        for card2 in table.iter().skip(position + 1) {
            let color3 = next_property(card1.color, card2.color, &colors);
            let count3 = next_property(card1.count, card2.count, &counts);
            let shape3 = next_property(card1.shape, card2.shape, &shapes);
            let fill3 = next_property(card1.fill, card2.fill, &fills);
            let card3 = Card::new(color3, count3, shape3, fill3);
            if table.contains(&card3) {
                return Some([*card1, *card2, card3]);
            }
        }
    }
    None
}

struct Game {
    pub deck: set::Deck,
    pub table: HashSet<Card>,
    pub sets: HashSet<[Card; 3]>,
}

/// Playing the game. This sort of mixes game rules and player actions, but it's
/// fine.
impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            deck: set::Deck::new(),
            table: HashSet::new(),
            sets: HashSet::new(),
        };

        while game.table.len() < 12 {
            game.table
                .extend(game.deck.deal().expect("This deck is missing a few cards."));
        }

        game
    }

    /// Start-of-turn setup - make sure there are at least 12 cards on the table.
    pub fn upkeep(&mut self) {
        if self.table.len() < 12 {
            self.add_cards();
        }
    }

    /// Find a set.
    pub fn find_set(&mut self) -> bool {
        let found_set = find_set_third_card(&self.table);
        if let Some(cards) = found_set {
            println!("Found: {}, {}, {}.", cards[0], cards[1], cards[2]);
            self.sets.insert(cards);
            for card in cards {
                self.table.remove(&card);
            }
            true
        } else {
            false
        }
    }

    /// Deal three cards from the deck, if possible.
    pub fn add_cards(&mut self) -> bool {
        if let Some(cards) = self.deck.deal() {
            println!(
                "Dealing new cards: {}, {}, {}.",
                cards[0], cards[1], cards[2]
            );
            self.table.extend(cards);
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut game = Game::new();

    println!("Initial table:");
    print_table(&game.table);

    loop {
        game.upkeep();
        if !game.find_set() {
            // No set found, try to add some more cards.
            if !game.add_cards() {
                // no sets, out of cards, so the game ends.
                break;
            }
        }
    }

    println!("Final table:");
    print_table(&game.table);
}

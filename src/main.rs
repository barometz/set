use std::collections::HashSet;
use std::hash::Hash;

use set::card::Card;

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
    mut all_properties: HashSet<Property>,
) -> Property {
    if property1 == property2 {
        property1
    } else {
        all_properties.remove(&property1);
        all_properties.remove(&property2);
        *all_properties.iter().next().expect("Not enough properties")
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
            let color3 = next_property(card1.color, card2.color, colors.clone());
            let count3 = next_property(card1.count, card2.count, counts.clone());
            let shape3 = next_property(card1.shape, card2.shape, shapes.clone());
            let fill3 = next_property(card1.fill, card2.fill, fills.clone());
            let card3 = Card::new(color3, count3, shape3, fill3);
            if table.contains(&card3) {
                return Some([*card1, *card2, card3]);
            }
        }
    }
    None

}

fn main() {
    let mut deck = set::Deck::new();

    let mut table = HashSet::<set::card::Card>::new();

    // prepare the table
    while table.len() < 12 {
        table.extend(deck.deal().expect("This deck is missing a few cards."));
    }

    println!("Initial table:");
    for card in &table {
        print!("{}, ", card);
    }
    println!();

    while !table.is_empty() {
        println!("{} cards on the table.", table.len());
        let found_set = find_set_third_card(&table);

        match found_set {
            Some(cards) => {
                println!("Found: {}, {}, {}.", cards[0], cards[1], cards[2]);
                for card in &cards {
                    table.remove(card);
                }

                if table.len() < 12 {
                    if let Some(new_cards) = deck.deal() {
                        println!(
                            "Dealing new cards: {}, {}, {}.",
                            new_cards[0], new_cards[1], new_cards[2]
                        );

                        table.extend(new_cards);
                    }
                }
            }
            None => {
                println!("No sets found.");
                if let Some(new_cards) = deck.deal() {
                    println!(
                        "Dealing new cards: {}, {}, {}.",
                        new_cards[0], new_cards[1], new_cards[2]
                    );
                    table.extend(new_cards);
                } else {
                    break;
                }
            }
        }
    }
}

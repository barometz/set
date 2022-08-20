use std::collections::HashSet;

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
        let found_set = find_set_bruteforce(&table);

        match found_set {
            Some(cards) => {
                println!("Found: {}, {}, {}.", cards[0], cards[1], cards[2]);
                for card in &cards {
                    table.remove(card);
                }

                if table.len() < 12 {
                    if let Some(new_cards) = deck.deal() {
                    println!("Dealing new cards: {}, {}, {}.", new_cards[0], new_cards[1], new_cards[2]);

                        table.extend(new_cards);
                    }
                }
            }
            None => {
                println!("No sets found.");
                if let Some(new_cards) = deck.deal() {
                    println!("Dealing new cards: {}, {}, {}.", new_cards[0], new_cards[1], new_cards[2]);
                    table.extend(new_cards);
                } else {
                    break;
                }
            }
        }
    }
}

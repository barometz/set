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

    println!("Initial table: {:?}.", table);

    while !table.is_empty() {
        println!("{} cards on the table.", table.len());
        let found_set = find_set_bruteforce(&table);

        match found_set {
            Some(cards) => {
                println!("Found: {:?}", cards);
                for card in &cards {
                    table.remove(card);
                }

                if table.len() < 12 {
                    if let Some(new_cards) = deck.deal() {
                    println!("Dealing new cards: {:?}.", new_cards);

                        table.extend(new_cards);
                    }
                }
            }
            None => {
                if let Some(new_cards) = deck.deal() {
                    println!("No sets found.");
                    println!("Dealing new cards: {:?}.", new_cards);
                    table.extend(new_cards);
                } else {
                    break;
                }
            }
        }
    }
}

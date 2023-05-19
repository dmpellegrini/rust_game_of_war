use crate::{
    card::{
    Card,
    Suit,
    Value,
    get_all_cards,
    },
};

#[derive(Debug)]
pub struct Deck {
    pub cards: [Card; 52],
}

impl Deck {
    pub fn new_blank() -> Deck {
        let cards = [Card::new(Value::Blank, Suit::Blank); 52];

        Deck{
            cards,
        }
    }

    pub fn new_full() -> Deck {
        let cards = get_all_cards(false);

        Deck{
            cards,
        }
    }
}

pub fn new_player_decks(shuffled: bool) -> (Deck, Deck) {
    let cards = get_all_cards(shuffled);

    let mut deck_player1 = Deck::new_blank();
    let mut deck_player2 = Deck::new_blank();
    let mut index = 0;

    for card in &cards {
        if index < 26 {
            deck_player1.cards[index % 26] = card.clone();
        } else if index > 25 {
            deck_player2.cards[index % 26] = card.clone();
        }
        index += 1;
    }

    (deck_player1, deck_player2)
}

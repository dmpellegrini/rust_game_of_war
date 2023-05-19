use crate::deck::Deck;
use crate::card::{Card, Value};
use rand::Rng;

pub fn play_game(mut deck_1: Deck, mut deck_2: Deck) -> (Deck, Deck){

    println!("Before 1 {0:?}, \n\n Before 2 {1:?}", deck_1, deck_2);
    for index in 1..=1000 {
        // let (mut deck_1, mut deck_2, drawn_cards) = draw_cards(&mut deck_1, &mut deck_2);
        let drawn_cards = draw_cards(&mut deck_1, &mut deck_2);
        println!("Player1 drew {0:?}, Player2 drew {1:?}, {2:?}", drawn_cards[0], drawn_cards[1], index);

        if drawn_cards[0].card_value() == 0 {
            println!("Player2 Wins The Game");
            break
        } else if drawn_cards[1].card_value() == 0 {
            println!("Player1 Wins The Game");
            break
        } else if drawn_cards[0].card_value() == drawn_cards[1].card_value() {
            declare_war(&mut deck_1, &mut deck_2, drawn_cards);
        } else if drawn_cards[0].card_value() > drawn_cards[1].card_value() {
            println!("Player1 Wins");
            award_cards(&mut deck_1, drawn_cards);
        }else if drawn_cards[1].card_value() > drawn_cards[0].card_value(){
            println!("Player2 Wins");
            award_cards(&mut deck_2, drawn_cards);
        }
    }
    println!("After 1 {0:?}, \n\n After 2 {1:?}", deck_1, deck_2);

    (deck_1, deck_2)
}

fn award_cards(deck: &mut Deck, drawn_cards: [Card; 2]) -> () {
    let shuffled_drawn_cards = shuffle_draw_cards(drawn_cards);
    for drawn_card in shuffled_drawn_cards {
        for (j, deck_card) in deck.cards.iter().enumerate() {
            match deck_card.value {
                Value::Blank => {
                    deck.cards[j] = drawn_card.clone();
                    break
                },
                _ => continue,
            }
        }
    }
}

fn award_war_pile(deck: &mut Deck, drawn_cards: Vec<Card>) -> () {
    // let shuffled_war_pile = shuffle_war_pile(drawn_cards);
    // for drawn_card in shuffled_war_pile {
    for drawn_card in drawn_cards {
        for (j, deck_card) in deck.cards.iter().enumerate() {
            match deck_card.value {
                Value::Blank => {
                    deck.cards[j] = drawn_card.clone();
                    break
                },
                _ => continue,
            }
        }
    }
}

fn draw_cards<'a>(deck_1: &'a mut Deck, mut deck_2: &'a mut Deck) -> [Card; 2] {

    let drawn_cards = [deck_1.cards[0].clone(), deck_2.cards[0].clone()];

    for index in 0..(deck_1.cards.len() - 1){
        deck_1.cards[index] = deck_1.cards[index + 1].clone();
        deck_2.cards[index] = deck_2.cards[index + 1].clone();
    }
    drawn_cards
}

fn shuffle_draw_cards(mut cards: [Card; 2]) -> [Card; 2]{
    for num in 0..cards.len() {
        let rand_index = rand::thread_rng().gen_range(0..cards.len());
        let temp = cards[rand_index].clone();
        cards[rand_index] = cards[num].clone();
        cards[num] = temp;
    }
    cards
}

fn shuffle_war_pile(mut cards: Vec<Card>) -> Vec<Card>{
    for num in 0..cards.len() {
        let rand_index = rand::thread_rng().gen_range(0..cards.len());
        let temp = cards[rand_index].clone();
        cards[rand_index] = cards[num].clone();
        cards[num] = temp;
    }
    cards
}

fn declare_war<'a>(mut deck_1: &'a mut Deck, mut deck_2: &'a mut Deck, mut drawn_cards: [Card; 2]) -> (Vec<Card>, bool) {

    let mut winner = true;
    let mut war_pile = vec![
        drawn_cards[0].clone(),
        drawn_cards[1].clone(),
        deck_1.cards[0].clone(),
        deck_2.cards[0].clone(),
        deck_1.cards[1].clone(),
        deck_2.cards[1].clone(),
        deck_1.cards[2].clone(),
        deck_2.cards[2].clone(),
        deck_1.cards[3].clone(),
        deck_2.cards[3].clone(),
    ];

    for _ in 0..4 {
        for index in 0..(deck_1.cards.len() - 1){
            deck_1.cards[index] = deck_1.cards[index + 1].clone();
            deck_2.cards[index] = deck_2.cards[index + 1].clone();
        }
    }
    
    println!("{:#?}", war_pile);
    println!("Player1 drew {0:?}, Player2 drew {1:?}", war_pile[8], war_pile[9]);
    if war_pile[8].card_value() == 0 && war_pile[9].card_value() == 0 {
        println!("This is rare... both players ran out of cards, looks like a draw");
    } else if war_pile[8].card_value() == 0 {
        winner = false;
        println!("Player2 Wins The Game");
    } else if war_pile[9].card_value() == 0 {
        winner = true;
        println!("Player1 Wins The Game");
    } else if war_pile[8].card_value() > war_pile[9].card_value() {
        winner = true;
        println!("Player1 Wins");
        award_war_pile(deck_1, war_pile.clone());
    } else if war_pile[9].card_value() > war_pile[8].card_value() {
        winner = false;
        println!("Player2 Wins");
        award_war_pile(deck_2, war_pile.clone());
    } else if war_pile[8].card_value() == war_pile[9].card_value() {
        drawn_cards[1] = war_pile.pop().unwrap();
        drawn_cards[0] = war_pile.pop().unwrap();
        let (mut award_pile, winner) = declare_war(&mut deck_1, &mut deck_2, drawn_cards);
        if winner {
            award_war_pile(deck_1, war_pile.clone());
        } else if !winner {
            award_war_pile(deck_2, war_pile.clone());
        }

        
    }
    (war_pile, winner)
}

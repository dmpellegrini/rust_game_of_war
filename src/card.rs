use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
    Blank,
}

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
    Blank,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Card {
        Card {
            value,
            suit,
        }
    }

    pub fn card_value(self) -> u8 {
       match self.value {
           Value::Ace => 14,
           Value::King => 13,
           Value::Queen => 12,
           Value::Jack => 11,
           Value::Number(num) => {
               if num > 1 || num < 11 {
                   num
               }else{
                   panic!("Invalid number");
               }
           },
           Value::Blank => 0
       } 
    }
}

pub fn get_all_cards(shuffled: bool) -> [Card; 52] {

    let all_values: [Value; 13] = [
        Value::Ace,
        Value::King,
        Value::Queen,
        Value::Jack,
        Value::Number(10),
        Value::Number(9),
        Value::Number(8),
        Value::Number(7),
        Value::Number(6),
        Value::Number(5),
        Value::Number(4),
        Value::Number(3),
        Value::Number(2),
    ];

    let all_suits: [Suit; 4] = [
        Suit::Spades,
        Suit::Clubs,
        Suit::Diamonds,
        Suit::Hearts
    ];

    let mut index = 0;
    let mut all_cards = [Card::new(Value::Blank, Suit::Blank); 52]; 

    for suit in &all_suits {
        for value in &all_values {
            all_cards[index] = Card::new(value.clone(), suit.clone());
            index += 1;
        }
    }

    if shuffled {
        return shuffle_cards(all_cards)
    }
    all_cards
}

pub fn shuffle_cards(mut cards: [Card; 52]) -> [Card; 52]{
    for num in 0..cards.len() {
        let rand_index = rand::thread_rng().gen_range(0..cards.len());
        let temp = cards[rand_index].clone();
        cards[rand_index] = cards[num].clone();
        cards[num] = temp;
    }
    cards
}




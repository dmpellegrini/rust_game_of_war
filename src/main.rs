use rust_game_of_war::deck::new_player_decks;
use rust_game_of_war::game::play_game;

fn main() {
    let (deck_player1, deck_player2) = new_player_decks(true);
    play_game(deck_player1, deck_player2);
}


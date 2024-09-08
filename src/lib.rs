#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// written by Steven Hert, omo this code needs to be formated after the competition
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
extern crate alloc;
use stylus_sdk::{ alloy_primitives::U256, prelude::*, stylus_proc::entrypoint };

use stylus_sdk::{ console, msg };

sol_storage! {
    #[entrypoint]
    pub struct Game {
        string word;
        string hint;
        string guessed;
        Player player1;
        Player player2;
        Player player3;
        uint256 turn;
        bool game_active;
    
    }

    pub struct Player {
        string name;
        string name_id;
        uint256 score;
        uint256 turn;
    }
}

#[external]
impl Game {
    pub fn new(&mut self) -> String {
        let word = "antidisestablishmentarianism".to_string();
        let hint =
            "opposition to withdrawing state support from an established institution,
            historical,
            religious context,
            political term,
            long word".to_string();

        // omo this is self explanatry
        self.word.set_str(word);
        self.hint.set_str(hint);
        self.guessed.set_str("");
        self.turn.set(U256::from(1));
        self.game_active.set(false);
        self.player1.name.set_str("");
        self.player1.name_id.set_str("");
        self.player1.score.set(U256::from(0));
        self.player1.turn.set(U256::from(1));
        self.player2.name.set_str("");
        self.player2.name_id.set_str("");
        self.player2.score.set(U256::from(0));
        self.player3.name.set_str("");
        self.player3.name_id.set_str("");
        self.player3.score.set(U256::from(0));
        return "Machine new state".to_string();
    }

    pub fn add_player(&mut self, name: String) -> String {
        if self.game_active.get() {
            return String::from("Game has started");
        }
        if self.player1.name.get_string() == String::from("") {
            self.player1.name.set_str(name);
            let user_id = format!("{}", msg::sender());
            self.player1.name_id.set_str(user_id);
            return String::from("Player1 set");
        } else if self.player1.name.get_string() == String::from("") {
            self.player2.name.set_str(name);
            let user_id = format!("{}", msg::sender());
            self.player2.name_id.set_str(user_id);
            return String::from("Player2 set");
        } else if self.player1.name.get_string() == String::from("") {
            self.player3.name.set_str(name);
            let user_id = format!("{}", msg::sender());
            self.player3.name_id.set_str(user_id);
            self.game_active.set(true);
            return String::from("Player3 set");
        } else {
            return String::from("No more player space");
        }
    }

    // Guess a letter (called by frontend for each player’s turn)
    pub fn guess_letter(&mut self, letter: String) -> String {
        if !self.game_active.get() {
            return String::from("Game over. Start a new game.");
        }

        if self.my_turn() == 0 {
            return String::from("It's not your turn.");
        }

        // Check if letter is in the word
        let mut score_increase = 0;
        for c in self.word.get_string().chars() {
            if c.to_string() == letter {
                score_increase += 1;
            }
        }

        // Update player score
        match self.my_turn() {
            1 => {
                let state_x = self.player1.score.get();
                let state_x2 = state_x + U256::from(score_increase);
                self.player1.score.set(state_x2);
            }
            2 => {
                let state_x = self.player2.score.get();
                let state_x2 = state_x + U256::from(score_increase);
                self.player2.score.set(state_x2);
            }
            3 => {
                let state_x = self.player3.score.get();
                let state_x2 = state_x + U256::from(score_increase);
                self.player3.score.set(state_x2);
            }
            _ => (),
        }

        let character = letter.chars().next();

        match character {
            Some(c) => self.guessed.get_string().push(c),
            None => (),
        }

        if self.is_word_complete() {
            self.game_active.set(false);
            let t = self.get_guessed_word();
            return format!(
                "Game complete! {} is the winner! word is {}",
                self.get_winner_name(),
                t
            );
        }
        let turner;
        if self.turn.get() == U256::from(3) {
            turner = U256::from(1);
        } else {
            turner = self.turn.get() + U256::from(1);
        }
        // Change turn to next player
        self.turn.set(turner);

        // Return the guessed state to the frontend
        self.get_guessed_word()
    }

    // Check if the word is fully guessed
    fn is_word_complete(&self) -> bool {
        self.word
            .get_string()
            .chars()
            .all(|c| self.guessed.get_string().contains(c))
    }

    // Get the current state of the guessed word (e.g., *ddr***)
    pub fn get_guessed_word(&self) -> String {
        self.word
            .get_string()
            .chars()
            .map(|c| if self.guessed.get_string().contains(c) { c } else { '*' })
            .collect()
    }

    // Get the name of the player with the highest score
    pub fn get_winner_name(&self) -> String {
        let mut winner = self.player1.name.get_string();
        if self.player2.score.get() > self.player1.score.get() {
            winner = self.player2.name.get_string();
        }
        if self.player3.score.get() > self.player1.score.get() {
            winner = self.player3.name.get_string();
        }
        winner
    }

    pub fn get_scores(&self) -> String {
        let player1_score = self.player1.score.get();
        let player2_score = self.player2.score.get();
        let player3_score = self.player3.score.get();
        let player1_name = self.player1.name.get_string();
        let player2_name = self.player2.name.get_string();
        let player3_name = self.player3.name.get_string();
        return format!(
            r#"{{{}:{}, {}:{}, {}:{}}}"#,
            player1_name,
            player1_score,
            player2_name,
            player2_score,
            player3_name,
            player3_score
        );
    }
}

impl Game {
    pub fn my_turn(&self) -> u8 {
        // return my turn number if it's my turn
        // return 0 if it's not my turn
        let user_id = format!("{}", msg::sender());
        if self.player1.name_id.get_string() == user_id {
            if self.player1.turn.get() == self.turn.get() {
                return 1;
            }
        } else if self.player2.name_id.get_string() == user_id {
            if self.player2.turn.get() == self.turn.get() {
                return 2;
            }
        } else if self.player3.name_id.get_string() == user_id {
            if self.player3.turn.get() == self.turn.get() {
                return 3;
            }
        }
        return 0;
    }
}

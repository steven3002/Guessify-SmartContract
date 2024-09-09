#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// written by Steven Hert, omo this code needs to be formated after the competition
// #[global_allocator]
// static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
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

#[public]
impl Game {
    pub fn new(&mut self) {
        let word = "antidisestablishmentarianism".to_string();
        let hint =
            "opposition to withdrawing state support from an established institution,
            historical,
            religious context,
            political term,
            long word".to_string();
        let default_x = "".to_string();
        // omo this is self explanatry
        self.word.set_str(word);
        self.hint.set_str(hint);
        self.guessed.set_str(default_x.clone());
        self.turn.set(U256::from(1));
        self.game_active.set(false);

        self.player1.name.set_str(default_x.clone());
        self.player1.name_id.set_str(default_x.clone());
        self.player1.score.set(U256::from(0));
        self.player1.turn.set(U256::from(1));

        self.player2.name.set_str(default_x.clone());
        self.player2.name_id.set_str(default_x.clone());
        self.player2.score.set(U256::from(0));
        self.player2.turn.set(U256::from(2));

        self.player3.name.set_str(default_x.clone());
        self.player3.name_id.set_str(default_x.clone());
        self.player3.score.set(U256::from(0));
        self.player3.turn.set(U256::from(3));
    }

    pub fn add_player(&mut self, name: String) {
        if self.game_active.get() {
            return;
        }
        if self.player1.name.get_string().is_empty() {
            self.player1.name.set_str(name);
            let user_id = format!("{}", msg::sender());
            self.player1.name_id.set_str(user_id);
            return;
        } else if self.player2.name.get_string().is_empty() {
            self.player2.name.set_str(name);
            let user_id = format!("{}", msg::sender());
            self.player2.name_id.set_str(user_id);
            return;
        } else if self.player3.name.get_string().is_empty() {
            self.player3.name.set_str(name);
            let user_id = format!("{}", msg::sender());
            self.player3.name_id.set_str(user_id);
            self.game_active.set(true);
            return;
        } else {
            return;
        }
    }

    // Guess a letter (called by frontend for each player’s turn)
    pub fn guess_letter(&mut self, letter: String) {
        if !self.game_active.get() {
            return;
        }

        if self.my_turn() == 0 {
            return;
        }

        if self.guessed.get_string().contains(letter.as_str()) {
            return;
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

        let mut guessed = self.guessed.get_string();
        if let Some(c) = letter.chars().next() {
            guessed.push(c);
        }
        self.guessed.set_str(&guessed);

        if self.is_word_complete() {
            self.game_active.set(false);

            return;
        }
        let turner;
        if self.turn.get() == U256::from(3) {
            turner = U256::from(1);
        } else {
            turner = self.turn.get() + U256::from(1);
        }
        // Change turn to next player
        self.turn.set(turner);
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
    pub fn get_hints(&self) -> String {
        self.hint.get_string()
    }
    pub fn get_turn(&self) -> String {
        let turn_m = self.turn.get();
        format!("{}", turn_m)
    }
}

impl Game {
    pub fn my_turn(&self) -> u8 {
        // return  if it's my turn
        // return  if it's not my turn

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

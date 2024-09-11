#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// written by Steven Hert, omo this code needs to be formated after the competition

extern crate alloc;
use stylus_sdk::{ alloy_primitives::U8, prelude::*, stylus_proc::entrypoint };
use stylus_sdk::{ console, msg };

sol_storage! {
    #[entrypoint]
    pub struct Game {
        string word; //holds the current state word
        string hint;//holds the current state hint
        string guessed;//holds the guessed letters
        Player player1; //holds info on the first player
        Player player2;// holds info on the secound player
        Player player3;//holds info on the third player
        uint8 turn; //holds the  turn of the current player
        bool game_active; // this is used to control the state of the game, but due to space reason it is not implemented a lot
        SWorod[] meta_data; //  this holds the current data on all the hints and games
    }

    pub struct Player {
        // this is the data of the state of a given player 
        string name; // holds the name of the player(the fronend guy is to send me the name)
        address name_id; // holds the wallet address of the player
        uint8 score; // holds the score of the player
        uint8 turn; // holds the turn id of the player\
    }

    pub struct SWorod{
        //before i wanted to put all the data in a Vec<[String; 2]> list
        string word; // holds a instance word
        string hint;  //holds the hint of that instance word 
    }
}

#[public]
impl Game {
    pub fn admin_set(&mut self, data: [String; 2]) {
        // ths is to add more data to the stored hints and words
        let mut state_increment = self.meta_data.grow();
        state_increment.word.set_str(data[0].clone());
        state_increment.hint.set_str(data[1].clone());
    }

    pub fn new(&mut self, index: u32) {
        // this is to refresh the instance of the machine
        let default_x = "".to_string();
        // omo this is self explanatry
        if let Some(state_word) = self.meta_data.get(index as usize) {
            let word = state_word.word.get_string();
            let hint = state_word.hint.get_string();
            self.word.set_str(word);
            self.hint.set_str(hint);
        }

        self.guessed.set_str(default_x.clone());
        self.turn.set(U8::from(1));
        self.game_active.set(false);

        self.player1.name.set_str(default_x.clone());

        self.player1.score.set(U8::from(0));
        self.player1.turn.set(U8::from(1));

        self.player2.name.set_str(default_x.clone());
        self.player2.score.set(U8::from(0));
        self.player2.turn.set(U8::from(2));

        self.player3.name.set_str(default_x.clone());
        self.player3.score.set(U8::from(0));
        self.player3.turn.set(U8::from(3));
    }

    pub fn add_player(&mut self, name: String) {
        if self.game_active.get() {
            return;
        }
        if self.player1.name.get_string().is_empty() {
            self.player1.name.set_str(name);

            self.player1.name_id.set(msg::sender());
            return;
        } else if self.player2.name.get_string().is_empty() {
            self.player2.name.set_str(name);

            self.player2.name_id.set(msg::sender());
            return;
        } else if self.player3.name.get_string().is_empty() {
            self.player3.name.set_str(name);

            self.player3.name_id.set(msg::sender());
            self.game_active.set(true);
            return;
        } else {
            return;
        }
    }

    // Guess a letter (called by frontend for each player’s turn)
    pub fn guess_letter(&mut self, letter: String) {
        let turn_x = self.my_turn();
        if !self.game_active.get() || turn_x == 0 {
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
        match turn_x {
            1 => {
                let state_x = self.player1.score.get();
                let state_x2 = state_x + U8::from(score_increase);
                self.player1.score.set(state_x2);
            }
            2 => {
                let state_x = self.player2.score.get();
                let state_x2 = state_x + U8::from(score_increase);
                self.player2.score.set(state_x2);
            }
            3 => {
                let state_x = self.player3.score.get();
                let state_x2 = state_x + U8::from(score_increase);
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
        if self.turn.get() == U8::from(3) {
            turner = U8::from(1);
        } else {
            turner = self.turn.get() + U8::from(1);
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

        let user_id = msg::sender();
        if self.player1.name_id.get() == user_id {
            if self.player1.turn.get() == self.turn.get() {
                return 1;
            }
        } else if self.player2.name_id.get() == user_id {
            if self.player2.turn.get() == self.turn.get() {
                return 2;
            }
        } else if self.player3.name_id.get() == user_id {
            if self.player3.turn.get() == self.turn.get() {
                return 3;
            }
        }
        return 0;
    }
}

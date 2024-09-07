#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// written by Steven Hert, omo this code needs to be formated after the competition

// #[global_allocator]
// static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
extern crate alloc;

use stylus_sdk::{ alloy_primitives::U256, prelude::*, stylus_proc::entrypoint };

use stylus_sdk::{ console, msg };

use alloy_sol_types::sol;

sol_storage! {
    #[entrypoint]
    pub struct Room {
        address[] player;
        mapping(address => string ) name;
        mapping(address => uint256) score;
        State state;
        bool started;
        bool gameEnded;
    }

    pub struct State{
        uint256 turn; // turns = 0 too 2, +1 until == 2 then equals to zero
        string current;
        string[] hints;
        uint256 wordsLen;
        string[] word;
        uint256 total_guess;

        
    }
}

#[public]
impl Room {
    pub fn init(&mut self) {
        self.started.set(false);
        self.gameEnded.set(false);
        self.state.turn.set(U256::from(0));
        self.state.total_guess.set(U256::from(0));

        let naxz = [
            "kaleidoscope",
            "antidisestablishmentarianism",
            "uncharacteristically",
            "disproportionateness",
            "interchangeableness",
            "misunderstandingly",
            "indistinguishability",
            "counterintuitiveness",
            "unconstitutionality",
            "incompatibilities",
            "unintelligibilities",
        ];

        let hintx = [
            [
                "Optical device",
                "Produces changing patterns",
                "Uses mirrors and colored glass",
                "Creates beautiful and shifting designs",
                "Often used for visual art",
            ],
            [
                "opposition to withdrawing state support from an established institution",
                "historical",
                "religious context",
                "political term",
                "long word",
            ],
            [
                "acting in a way that is not typical",
                "uncommon behavior",
                "out of character",
                "unusual actions",
                "rare",
            ],
            [
                "lack of proportion",
                "uneven distribution",
                "imbalance",
                "size difference",
                "disproportion",
            ],
            [
                "capable of being exchanged",
                "mutual replacement",
                "swapping things",
                "substitution",
                "mutual",
            ],
            [
                "misinterpretation",
                "failure to understand correctly",
                "confusion",
                "wrong idea",
                "misconception",
            ],
            [
                "not able to be distinguished",
                "difficult to tell apart",
                "similar in appearance",
                "identity confusion",
                "likeness",
            ],
            [
                "goes against common sense",
                "counter to expectations",
                "surprising logic",
                "paradoxical",
                "unconventional reasoning",
            ],
            [
                "in violation of a constitutional law",
                "illegal by constitution",
                "law-related",
                "government rules",
                "invalid",
            ],
            [
                "inability to exist or work together",
                "conflicting traits",
                "differences",
                "unsuitable pairing",
                "non-cohesive",
            ],
            [
                "unable to be understood",
                "incomprehensible",
                "unclear communication",
                "confusing language",
                "vague",
            ],
        ];

        let max = naxz.len();
        let number = 1;

        let word_new = naxz[number as usize].to_string();
        let word_len = word_new.len();
        self.state.wordsLen.set(U256::from(word_len));
        for _xi in 0..word_len {
            let mut star = self.state.word.grow();
            let charx = "*".to_string();
            star.set_str(charx);
        }
        self.state.current.set_str(word_new);
        let datr = hintx[number as usize];

        for d in datr {
            let mut appl = self.state.hints.grow();
            let string_data = d.to_string();
            appl.set_str(string_data);
        }
        self.state.wordsLen.set(U256::from(12));
    }

    pub fn add_player(&mut self, name: String) -> Result<String, String> {
        if self.player.len() == 3 {
            for index in 0..self.player.len() {
                if let Some(storage_guard) = self.player.getter(index) {
                    if format!("{}", *storage_guard.get()) == format!("{}", msg::sender()) {
                        return Ok(String::from("Game Started"));
                    }
                }
            }
            return Err("Game is full".to_string());
        }
        self.player.push(msg::sender());

        let mut name_accessor = self.name.setter(msg::sender());
        name_accessor.set_str(name);
        self.score.insert(msg::sender(), U256::from(0));
        if self.game_start() {
            return Ok(String::from("Start"));
        }
        return Ok("wait....".to_string());
    }

    pub fn play(&mut self, letter: String) -> Result<String, String> {
        if self.gameEnded.get() {
            return Err("Game is ended".to_string());
        }
        if !self.in_room() {
            return Err("you are not in room".to_string());
        }
        if !self.started.get() {
            return Err("game in lobby".to_string());
        }
        let playerIn = self.player_index();
        if self.state.turn.get() == U256::from(playerIn) {
            let current_string = self.state.current.get_string();
            let word = String::from(current_string);
            let count_ = self.count_and_update(word, letter);
            let mut current_score = self.score.setter(msg::sender());

            let new_score = current_score.get() + U256::from(count_);
            current_score.set(new_score);
            let mut _m = vec![];
            for inx in 0..self.state.word.len() {
                let items = self.state.word.getter(inx);
                if let Some(_e) = items {
                    _m.push(_e.get_string());
                }
            }
            self.change_turn();
            return Ok(format!(r#"{{"result":{},"state":{:?}}}"#, count_, _m));
        } else {
            return Err("not your turn".to_string());
        }
    }

    pub fn get_score(&self) -> String {
        let mut names = vec![];
        let mut scores = vec![];

        for index in 0..3 {
            if let Some(storage_guard) = self.player.getter(index) {
                let name = self.name.getter(storage_guard.get());

                names.push(name.get_string());

                let score = self.score.getter(storage_guard.get());

                scores.push(score.get());
            }
        }
        return format!(r#"{{"names":{:?},"score":{:?}}}"#, names, scores);
    }

    pub fn get_word_len(&self) {
        self.state.wordsLen.get();
    }

    pub fn get_hint(&self) -> Result<String, String> {
        let current_guess_state = self.state.total_guess.get();
        if current_guess_state < U256::from(4) {
            return Err("No Hints".to_string());
        } else {
            let mut hint_ = vec![];
            let hint_len = current_guess_state / U256::from(4);
            if hint_len >= U256::from(5) {
                for inx in 0..5 {
                    let items = self.state.hints.getter(inx);
                    if let Some(_e) = items {
                        hint_.push(_e.get_string());
                    }
                }
                return Ok(format!(r#"{{"hints":{:?}}}"#, hint_));
            } else {
                let test = [1, 2, 3, 4];
                let mut control = 0;
                for tst in test {
                    if U256::from(tst.clone()) == hint_len {
                        control = tst;
                        break;
                    }
                }
                for inx in 0..control {
                    let items = self.state.hints.getter(inx);
                    if let Some(_e) = items {
                        hint_.push(_e.get_string());
                    }
                }
                return Ok(format!(r#"{{"hints":{:?}}}"#, hint_));
            }
        }
    }
}

impl Room {
    pub fn in_room(&self) -> bool {
        for index in 0..self.player.len() {
            if let Some(storage_guard) = self.player.getter(index) {
                if format!("{}", *storage_guard.get()) == format!("{}", msg::sender()) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn player_index(&self) -> u8 {
        for index in 0..self.player.len() {
            if let Some(storage_guard) = self.player.getter(index) {
                if format!("{}", *storage_guard.get()) == format!("{}", msg::sender()) {
                    return index as u8;
                }
            }
        }
        return 0;
    }

    pub fn count_and_update(&mut self, word: String, letter: String) -> usize {
        if letter.len() != 1 {
            return 0;
        }

        let letter_char = letter.chars().next().unwrap();
        let mut count = 0;

        for (i, c) in word.chars().enumerate() {
            if c == letter_char {
                if let Some(mut x) = self.state.word.setter(i) {
                    x.set_str(c.to_string());
                }
                count += 1;
            }
        }

        count
    }
    pub fn change_turn(&mut self) {
        let current_guess = self.state.total_guess.get() + U256::from(1);
        self.state.total_guess.set(current_guess);

        if self.state.turn.get() == U256::from(2) {
            self.state.turn.set(U256::from(0));
        } else {
            let state_ = U256::from(1) + self.state.turn.get();
            self.state.turn.set(state_);
        }
    }

    pub fn game_start(&mut self) -> bool {
        if self.player.len() == 3 {
            self.started.set(true);
            return true;
        }
        return false;
    }

    // still having the compersion issue and only this random dependencies is over 210kb
    // pub fn random_number(&self, max: u32) -> u32 {
    //     let mut rng = rand::thread_rng();
    //     rng.gen_range(0..max)
    // }
}

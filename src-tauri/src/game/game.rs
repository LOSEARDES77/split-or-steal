pub mod game{
    use std::collections::HashMap;

    pub struct Game{
        pub player_1: Player,
        pub player_2: Player,
        pub rounds: u32,
        pub score_board: Vec<[bool; 2]>,
        score_map: [u8; 4], // 0: both cooperatye  1: both defect  2: defect win   3: defect loose
        pub current_round: u32
    }
    #[derive(Clone)]
    pub struct Player{
        pub name: String,
        score: u32,
        pub history: Vec<u8>,  // 0: cooperate  1: defect 2: unplayed
        play_fn: fn(&Vec<u8>, &Vec<u8>, u32) -> bool // first arg: player_1 history, second arg: player_2 history third arg: current round   return: true for cooperate, false for defect
    }


    impl Game {
        
        fn allways_cooperate(_: &Vec<u8>, _: &Vec<u8>, _:u32) -> bool{
            true
        }

        fn allways_defect(_: &Vec<u8>, _: &Vec<u8>, _:u32) -> bool{
            false
        }

        fn copy_opponent(_: &Vec<u8>, opponent_history: &Vec<u8>, _:u32) -> bool{
            if opponent_history.len() == 0{
                return true;
            }

            opponent_history[opponent_history.len()-1] == 0
        }

        fn random_choice(_: &Vec<u8>, _: &Vec<u8>, _:u32) -> bool {
            rand::random()
        }

        fn once_defected(_: &Vec<u8>, oponent_history: &Vec<u8>, _: u32) -> bool{
            for i in oponent_history{
                if *i == 1{
                    return false;
                }
            }
            true
        }

        fn yes_no(self_history: &Vec<u8>, _: &Vec<u8>, _:u32) -> bool{
            if self_history.len() == 0{
                return true;
            }
            if *self_history.last().unwrap() == 1{
                return true;
            }
            false
            
        }

        fn def_if_twice_defected(_: &Vec<u8>, oponent_history: &Vec<u8>, _:u32) -> bool{
            if oponent_history.len() < 2{
                return true;
            }
            if oponent_history[oponent_history.len()-1] == 1 && oponent_history[oponent_history.len()-2] == 1{
                return false;
            }
            true
        }

        fn round_based(_: &Vec<u8>, _: &Vec<u8>, round: u32) -> bool{
            if round % 4 == 0{
                return false;
            }
            true
        }

        fn tester(self_history: &Vec<u8>, oponent_history: &Vec<u8>,round: u32,) -> bool{
            if round == 0{
                return false;
            }
            if round == 1{
                return true;
            }
            if oponent_history[1] == 0{
                if *self_history.last().unwrap() == 1{
                    return true;
                }
                return false;
            }
            true
        }


        

        pub fn new(player_1_name: String, player_2_name: String, rounds: u32, score_map: [u8; 4]) -> Game{

            let mut players: HashMap<String, Player> = HashMap::new();

            players.insert("Allways Cooperate".to_owned(), Player{
            name: "Allways Cooperate".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::allways_cooperate
            });

            players.insert("Allways Defect".to_owned(), Player{
            name: "Allways Defect".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::allways_defect
            });

            players.insert("Copy Opponent".to_owned(), Player{
            name: "Copy Opponent".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::copy_opponent
            });

            players.insert("Random Choice".to_owned(), Player{
            name: "Random Choice".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::random_choice
            });

            players.insert("Once Defected".to_owned(), Player{
            name: "Once Defected".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::once_defected
            });

            players.insert("Yes No".to_owned(), Player{
            name: "Yes No".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::yes_no
            });

            players.insert("Simple".to_owned(), Player{
            name: "Simple".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::def_if_twice_defected
            });

            players.insert("Round Based".to_owned(), Player{
            name: "Round Based".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::round_based
            });

            players.insert("Tester".to_owned(), Player{
            name: "Tester".to_owned(),
            score: 0,
            history: Vec::new(),
            play_fn: Game::tester
            });

            let p1 = players.get(&player_1_name).unwrap();
            let p2 = players.get(&player_2_name).unwrap();

            Game{
            player_1: p1.clone(),
            player_2: p2.clone(),
            rounds: rounds,
            score_board: Vec::new(),
            score_map: score_map,
            current_round: 0,
            }
        }

        fn fetch_result(&self, p1: bool, p2:bool) -> u8{
            if p1 && p2{
                return self.score_map[0];   
            }if !p1 && !p2{
                return self.score_map[1];
            }if !p1 && p2{
                return self.score_map[2];
            }
            self.score_map[3]
        }

        pub fn next_round(&mut self) -> [bool; 2]{
            let player_1 = &self.player_1;
            let player_2 = &self.player_2;

            let player_1_choice = (player_1.play_fn)(&player_1.history, &player_2.history, self.current_round);
            let player_2_choice = (player_2.play_fn)(&player_2.history, &player_1.history, self.current_round);
            
            let player_1_score_gained = self.fetch_result(player_1_choice, player_2_choice);
            let player_2_score_gained = self.fetch_result(player_2_choice, player_1_choice);

            let mut player_1 = self.player_1.clone();
            player_1.history.push(if player_1_choice {0} else {1});
            player_1.score += player_1_score_gained as u32;
            
            let mut player_2 = self.player_2.clone();
            player_2.history.push(if player_2_choice {0} else {1});
            player_2.score += player_2_score_gained as u32;


            self.score_board.push([player_1_choice, player_2_choice]);

            self.current_round += 1;

            [player_1_choice , player_2_choice]
        }

    }

}
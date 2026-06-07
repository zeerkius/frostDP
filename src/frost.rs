#![allow(
    unused_variables,
    dead_code,
    non_snake_case,
    unused_parens,
    unused_variables,
    non_snake_case,
    unreachable_code,
    unused_imports,
    unused_assignments,
    unused_mut
)]

use std::*;
use std::collections::{HashSet,HashMap};
use rand::seq::SliceRandom;
use rand::{random_iter, SeedableRng};
use rand::rngs::StdRng;
use rand::random_range;
use std::*;


pub struct CoinChange{
    pub coins : Vec<usize>,
    pub T : i32
}


impl CoinChange {
    pub fn new(coins: Vec<usize>, T: i32) -> Self {
        Self {
            coins,
            T
        }
    }


    pub fn CoinChangeTotalWays(self) -> i32 {
        if (self.T == 0) && (self.coins.is_empty()) {
            return 1
        }
        if (self.T > 0) && (self.coins.is_empty()) {
            return 0
        }

        // statically make dp table , dim(dp) = m x n

        let n: usize = self.coins.len() + 1;
        let m: usize = (self.T + 1) as usize;
        let coins: Vec<usize> = self.coins;


        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for j in 0..n {
            dp[0][j] = 1; // essentially expanding dp[0][0] = 1 across all j
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i][j - 1];

                if i >= coins[j - 1] {
                    dp[i][j] += dp[i - coins[j - 1]][j];
                }
            }
        }


        dp[m - 1][n - 1]
    }


    pub fn maxContribution(self) -> HashMap<i32, i32> {
        // tree of choices
        let mut sequences: Vec<Vec<i32>> = vec![];
        let mut path: Vec<i32> = vec![];
        let coins = self.coins.iter().map(|x| *x as i32).collect();

        let mut frequency_map: HashMap<i32, i32> = HashMap::new();



        fn dfs(s: i32, T: i32, sequences: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, coins: Vec<i32>) -> () {
            if s == T {
                sequences.push(path.clone());
                return
            }
            if s > T {
                return
            }
            for c in coins.iter() {
                path.push(c.clone()); // otherwise it will consume by value and cause moving errors
                let denomination: i32 = c.clone();
                let sum: i32 = s + denomination;
                dfs(sum, T, sequences, path, coins.clone())
            }
        }
        dfs(0, self.T, &mut sequences, &mut path, coins);

        // we will then flatten sequences and make a HashMap that Will essentially show the frequency of all denominations 
        // using all sequences that sum to T

        let sequences = sequences.iter().flatten();

        for d in sequences {
            *frequency_map.entry(*d).or_insert(0) += 1;
        }

        frequency_map
    }

    pub fn k_resources(self, k: i32) -> Vec<Vec<i32>> {  // we have k of each coin instead of inf of each coin

        let mut sequences: Vec<Vec<i32>> = vec![];
        let mut path: Vec<i32> = vec![];
        let coins = self.coins.iter().map(|x| *x as i32).collect();
        let k_ref = k;


        fn dfs(s: i32, T: i32, sequences: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, coins: Vec<i32>, k: i32) -> () {
            if s == T && k > 0 {
                sequences.push(path.clone());
                return
            }
            if s > T || k <= 0{
                return
            }
            for c in coins.iter() {
                path.push(c.clone()); // otherwise it will consume by value and cause moving errors
                let denomination: i32 = c.clone();
                let sum: i32 = s + denomination;
                let k_minus = k - 1;
                dfs(sum, T, sequences, path, coins.clone(), k_minus)
            }
        }
        dfs(0, self.T, &mut sequences, &mut path, coins, k_ref);

        // we will then flatten sequences and make a HashMap that Will essentially show the frequency of all denominations 
        // using all sequences that sum to T

        sequences
    }

    pub fn end(self, inst: CoinChange) -> () {
        drop(inst);
    }
}


pub struct STB{
    n_die : usize,
    m_trials : usize
}

    
    



impl STB {
    pub fn new(n_die: usize, m_trials: usize) -> Result<Self, String> {
        if n_die < 2 || m_trials < 2 {
            return Err(" Either m or n is less than 2".to_string());
        }
        Ok(Self { n_die, m_trials })
    }

    fn get_expectation(&self) -> Vec<f32> {
        let m: f32 = self.m_trials as f32;

        let a: i32 = 1;
        let b: i32 = self.n_die as i32;
        let mean: f32 = ((a + b) as f32 / 2.0);
        let sum_mean: f32 = (m * mean);

        println!(" Single Die Expected Value  {:?}", mean);
        println!(" Sum Die Expected Value {:?}", sum_mean);
        vec![mean, sum_mean]
    }

    fn make_cards(&self) -> Vec<i32> {
        let mut cards: Vec<i32> = vec![];
        let max_card: i32 = (self.n_die as i32) * (self.m_trials as i32);
        for c in 1..=max_card {
            cards.push(c);
        }
        cards
    }

    fn simulate_die(&self) -> Vec<i32> {
        let n: i32 = (self.n_die) as i32;
        let m: i32 = self.m_trials as i32;
        let mut die_trials: Vec<i32> = vec![];

        let mut rng = rand::rng();

        for trials in 1..=m {
            let res: i32 = random_range(1..=n);
            die_trials.push(res);
        }
        let sum: i32 = die_trials.iter().sum();
        die_trials.push(sum);


        die_trials
    }

    fn goal(&self) -> i32 {
        // faulhaber's
        let max: i32 = (self.n_die as i32) * (self.m_trials as i32);
        let mut sum: i32 = max * (max + 1);
        sum = sum / 2;
        sum
    }

    fn cartesian_power(&self) -> Vec<Vec<i32>> {
        let n: i32 = self.n_die as i32;
        let m: i32 = self.m_trials as i32;

        let mut res: Vec<Vec<i32>> = vec![];
        let mut curr: Vec<i32> = Vec::with_capacity(m as usize);

        fn dfs(n: i32, m: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, ) {
            if current.len() == m as usize {
                result.push(current.clone());
                return
            }
            for i in 1..=n {
                current.push(i);
                dfs(n, m, current, result);
                current.pop();
            }
        }

        dfs(n, m, &mut curr, &mut res);

        let length: usize = res.len();

        for i in 0..length {
            let s: i32 = res[i].iter().sum();
            res[i].push(s);
        }

        res
    }

    fn create_flip_map(&self) -> HashMap<i32, i32> {
        let cards = self.make_cards();
        let mut game_h: HashMap<i32, i32> = HashMap::new();

        for c in cards {
            game_h.insert(c, 1); //   1 -> flipped up , 0 -> flipped down
        }
        game_h
    }

    fn create_hashmap(&self) -> HashMap<i32, i32> {
        let cards = self.make_cards();
        let mut game_h: HashMap<i32, i32> = HashMap::new();

        for c in cards {
            game_h.insert(c, 0);
        }
        game_h
    }

    pub fn stb_random(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;


        while goal != 0 {
            let mut actions = self.simulate_die();

            // random choice strat

            let actions_length: i32 = actions.len() as i32;
            let random_choice = actions[random_range(0..actions_length) as usize];

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca.clone();
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, random_choice, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, random_choice, goal);
            println!("\n {:?}", state_string);
        }
    }

    pub fn stb_max(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;


        while goal != 0 {
            let mut actions = self.simulate_die();

            // optimal strat ~ max choice

            let mut optimal_choice = *actions.iter().max().unwrap();


            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, chosen_action, goal);
            println!("\n {:?}", state_string);
        }

        fn stb_MDP() -> () { () }

        fn stb_bellman() -> () { () }

        fn stb_SS() -> () { () }

        pub fn end(inst: STB) -> () {
            drop(inst);
        }
    }

    pub fn stb_min(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;


        while goal != 0 {
            let mut actions = self.simulate_die();

            // optimal strat ~ max choice

            let mut optimal_choice = *actions.iter().min().unwrap();


            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, chosen_action, goal);
            println!("\n {:?}", state_string);
        }
    }

    pub fn stb_median_single_die(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;


        while goal != 0 {
            let mut actions = self.simulate_die();

            let median = self.get_expectation();
            let single_median = median[0];

            fn get_error(actions: &Vec<i32>, sm: f32) -> usize {
                fn sse(a: i32, b: f32) -> i32 {
                    let c = b as i32;
                    let err: i32 = (a - c).pow(2);
                    err
                }
                let mut min_index: usize = 0;

                let mut err: Vec<i32> = vec![];
                for i in 0..actions.len() {
                    err.push(sse(actions[i], sm));
                }
                let min_err: i32 = *err.iter().min().unwrap();
                for j in 0..err.len() {
                    if err[j] == min_err {
                        min_index = j;
                    }
                }
                min_index
            }


            // optimal strat ~ max choice

            let mut optimal_choice = actions[get_error(&actions, single_median)];

            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, chosen_action, goal);
            println!("\n {:?}", state_string);
        }
    }

    pub fn stb_median_sum_die(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;


        while goal != 0 {
            let mut actions = self.simulate_die();

            let median = self.get_expectation();
            let single_median = median[1];

            fn get_error(actions: &Vec<i32>, sm: f32) -> usize {
                fn sse(a: i32, b: f32) -> i32 {
                    let c = b as i32;
                    let err: i32 = (a - c).pow(2);
                    err
                }
                let mut min_index: usize = 0;

                let mut err: Vec<i32> = vec![];
                for i in 0..actions.len() {
                    err.push(sse(actions[i], sm));
                }
                let min_err: i32 = *err.iter().min().unwrap();
                for j in 0..err.len() {
                    if err[j] == min_err {
                        min_index = j;
                    }
                }
                min_index
            }


            // optimal strat ~ max choice

            let mut optimal_choice = actions[get_error(&actions, single_median)];

            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, chosen_action, goal);
            println!("\n {:?}", state_string);
        }
    }

    pub fn stb_sum_choice(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;


        while goal != 0 {
            let mut actions = self.simulate_die();

            // optimal strat ~ max choice

            let mut optimal_choice = actions[actions.len() - 1];


            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, chosen_action, goal);
            println!("\n {:?}", state_string);
        }
    }
    
    pub fn stb_prob_choice(&self) -> () {
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;

        let prob: f32 = (self.m_trials as f32 - 1.0) / (self.m_trials as f32);


        while goal != 0 {
            let mut optimal_choice = 0;
            let mut actions = self.simulate_die();
            let p: f32 = random_range(0.0..1.0);

            if p > prob {
                optimal_choice = actions[actions.len() - 1];
            } else {
                let limit: i32 = (actions.len() - 2) as i32;
                let r: usize = random_range(0..limit) as usize;
                optimal_choice = actions[r];
            }

            // from paper

            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;


            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step, card_state_map, actions, chosen_action, goal);
            println!("\n {:?}", state_string);
        }
    }
    
    fn stb_prob(&self,p:f32) -> (i32,f32){
        let mut game_sequence: Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step: i32 = 0;

        let prob: f32 = p;

        let mut sum_choice : i32 = 0;
        while goal != 0 {
            let mut optimal_choice = 0;
            let mut actions = self.simulate_die();
            let p: f32 = random_range(0.0..1.0);

            if p > prob {
                optimal_choice = actions[actions.len() - 1];
                sum_choice += 1;
            } else {
                let limit: i32 = (actions.len() - 2) as i32;
                let r: usize = random_range(0..limit) as usize;
                optimal_choice = actions[r];
            }

            // from paper

            let chosen_action = optimal_choice;

            fn validate(actions: &mut Vec<i32>, fm: &mut HashMap<i32, i32>, gh: &mut HashMap<i32, i32>, ca: i32, goal: &mut i32) -> () {
                let mut actions_ref = ca;
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref, 0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative

                } else { // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref, 1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
            }
            validate(&mut actions, &mut card_state_map, &mut integer_history_map, chosen_action, &mut goal);
            game_step += 1;
            
        }
        let sum_portion : f32 = (sum_choice as f32 / game_step as f32 );
        let res = format!(" Game  Steps {:?} , Portion of Sum Choice {:?}" ,game_step,sum_portion);
        println!("{:?}",res);
        
        
        (game_step,sum_portion)

        
        
    }
    
    
    pub fn value_iteration(&self,d:f32,k:i32) -> Vec<Vec<f32>>{
        
        // actual goal for any (m,n)
        let integer_states = self.goal() as usize;
        let G = self.goal();
        let actions = self.m_trials * self.n_die;
        let denominator : f32 = actions as f32;

        
        // matrix that ranks all choices (mn x G) , {mn} rows (actions) , and {G} columns (state)
        
        let mut policy_sheet : Vec<Vec<f32>> = vec![vec![0.0 ;actions];integer_states];
        
        fn make_reward(curr_state : i32, action : i32, goal : i32) -> i32{ // essentially based on the G and how close we are to 0 the terminal state we change the potential reward
            if curr_state + action == goal{
                action
            }else if (curr_state + action > goal){
                let reward : i32 = goal - (curr_state + action);
                return reward;
            }else{
                let reward : i32 = action;
                return reward;
            }
        }

        fn get_prob(car_p : Vec<Vec<i32>>) -> Vec<f32>{


            let mut hm : HashMap<i32,i32> = HashMap::new();
            let mut actions : HashSet<i32> = HashSet::new();

            let mut p_vec : Vec<f32> = vec![];
            let mut c : f32 = 0.0;

            for s in car_p{
                let u = s.len();
                    for i in 0..u{
                        c += 1.0;
                        *hm.entry(s[i]).or_insert(0) += 1;
                        actions.insert(s[i]);
                    }
            }

            let mut p_vec : Vec<f32> = vec![];


            for a in actions.iter(){
                let int_count : f32 = *hm.get_mut(a).unwrap() as f32;
                p_vec.push(int_count / c);
            }
            
            p_vec
            }

        fn value_iteration(curr: f32, reward : f32 , p : f32 , max_next : f32 , d: f32) -> f32{

            let mut curr_ref = curr;
            curr_ref += p * (reward + (d * max_next));
            curr_ref
        }

        fn get_max(action_vec : &Vec<f32>) -> f32{
            let max = action_vec.iter().cloned().fold(f32::NEG_INFINITY,f32::max);
            max
        }

        // essentially we do it for any configuration of m and n

        let p_inst : Vec<Vec<i32>> = self.cartesian_power();

        let p_inst_len : usize = p_inst.len();

        let dens : f32 = p_inst[0..p_inst_len].to_vec().len() as f32;

        let curr_p_vec = get_prob(p_inst);

        let action_vector : Vec<usize> = (1..actions).collect();


        for sweep in 0..k{
            println!(" Iteration Count {:?}", sweep);
            for i in 0..policy_sheet.len(){
                for j in 0..policy_sheet[i].len(){
                    let curr_ref : f32 = policy_sheet[i][j];
                    let curr_a : i32 = (j + 1) as i32;
                    let curr_s : i32 = i as i32;
                    let curr_p : f32 = curr_p_vec[j];
                    let r : i32 = make_reward(curr_s,curr_a,G);
                    let f_r : f32 = (r as f32) * (0.00001); // scaling to avoid  inf during value iteration so R * (10 ** -4)
                    let new_state : i32 = (curr_s + r);
                    let new_state_index : usize = (new_state - 1) as usize;
                    let new_state_vec : Vec<f32> = policy_sheet[new_state_index].clone(); // 0 indexed
                    let max_a : f32 = get_max(&new_state_vec);
                    policy_sheet[i][j] += value_iteration(curr_s as f32,f_r,curr_p,max_a,d);
                }
            }
        }

        let M = format!(" Policy Sheet for  m = {:?}, n = {:?} Is V_k = {:?}",self.m_trials,self.n_die,policy_sheet);
        println!("Full Sweep {:?}",M);

        policy_sheet
    }
    
    fn get_max_f(&self,f : Vec<f32>) -> f32{
        let max : f32 = f.iter().cloned().fold(f32::NEG_INFINITY,f32::max);
        max
    }
    
    
    fn iteration_conversion(&self , V : Vec<Vec<f32>>) -> {
        
        let integer_states = V.len();
        let integer_actions = V[0].len();
        
        for i in 0..integer_states{
            
            while 
            
            V[i]
            
            
            
            
            
            
            
        }
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
    };
    
    fn make_policy_run_sim(&self,value_iterated: Vec<Vec<f32>> , k : i32 , discount_vector : Vec<f32>) -> {
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
    };
    

    
    
    

    pub fn stb_test(&self,p_vec:Vec<f32>) -> (i32,f32,f32){
        let base : i32 = 2;
        let mut min_step  : i32 = base.pow(30) - 1; // signed int max
        
        let m : usize = self.m_trials;
        let n : usize = self.n_die;
        
        
        let mut min_curr : i32 = min_step;
        let mut index_ct : usize = 0;
        let mut min_p : f32 = 0.0;
        let mut portion_of_sum : f32 = 0.0;
        
        for p in p_vec{
            let res = self.stb_prob(p);
            let steps = res.0;
            portion_of_sum = res.1;
            if steps < min_step{
                min_step = steps;
                min_p = p
            }
            index_ct += 1;
        }
        let optimal_policy = format!("Minimum Steps for m = {:?} , n = {:?}, Is  {:?} steps , Using Boundary {:?} , Portion of Sum {:?} ",m,n,min_step,min_p,portion_of_sum);
        (min_step,min_p,portion_of_sum)
    }
}


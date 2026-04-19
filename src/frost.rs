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

    
    



impl STB{
    
    pub fn new(n_die : usize, m_trials : usize) -> Result<Self, String> {
        if n_die < 2 || m_trials < 2 {
            return Err(" Either m or n is less than 2".to_string());
        }
        Ok(Self {n_die, m_trials})
    }
    
    pub fn get_expectation(&self) -> Vec<f32>{
        
        let m : f32 = self.m_trials as f32;
        
        let a: i32 = 1;
        let b: i32 = self.n_die as i32;
        let mean : f32 = ((a + b) as f32 / 2.0);
        let sum_mean : f32 = (m * mean);
        
        println!(" Single Die Expected Value  {:?}",mean);
        println!(" Sum Die Expected Value {:?}", sum_mean);
        vec![mean,sum_mean]
    }
    
    pub fn make_cards(&self) -> Vec<i32> {
        let mut cards : Vec<i32> = vec![];
        let max_card : i32 = (self.n_die as i32) * (self.m_trials as i32);
        for c in 1..=max_card{
            cards.push(c);
        }
        cards
        
    }
    
    pub fn simulate_die(&self) -> Vec<i32>{
        let n : i32 = (self.n_die) as i32;
        let m : i32 = self.m_trials  as i32;
        let mut die_trials : Vec<i32> = vec![];
        
        let mut rng = rand::rng();
        
        for trials in 1..=m{
            let res  : i32 = random_range(1..=n);
            die_trials.push(res);
        }
        let sum : i32 = die_trials.iter().sum();
        die_trials.push(sum);
        
        
        die_trials
        
    }
    
    pub fn goal(&self) -> i32{
        // faulhaber's
        let max : i32 = (self.n_die as i32) * (self.m_trials as i32);
        let mut sum : i32 = max * (max + 1);
        sum = sum / 2;
        sum
        
    }
    
    pub fn cartesian_power(&self) -> Vec<Vec<i32>>{
        let n : i32 = self.n_die as i32;
        let m : i32 = self.m_trials as i32;
        
        let mut res : Vec<Vec<i32>> = vec![];
        let mut curr : Vec<i32> = Vec::with_capacity(m as usize);
        
        fn dfs(n : i32 , m : i32 , current : &mut Vec<i32>,result : &mut Vec<Vec<i32>>,){
            if current.len() == m as usize{
                result.push(current.clone());
                return
            }
            for i in 1..=n {
                current.push(i);
                dfs(n, m, current, result);
                current.pop();
            }
        }
        
        dfs(n,m,&mut curr,&mut res);
        
        let length : usize = res.len();
        
        for i in 0..length{
            let s : i32 = res[i].iter().sum();
            res[i].push(s);
        }
        
        res
        }
    
    pub fn create_flip_map(&self) -> HashMap<i32,i32>{
        let cards = self.make_cards();
        let mut game_h : HashMap<i32,i32> = HashMap::new();
        
        for c in cards{
            game_h.insert(c,1); //   1 -> flipped up , 0 -> flipped down
        }
        game_h
    }
    
    pub fn create_hashmap(&self) -> HashMap<i32,i32>{
        let cards = self.make_cards();
        let mut game_h : HashMap<i32,i32> = HashMap::new();
    
        for c in cards{
        game_h.insert(c,0);
        }
        game_h
    }
    
    pub fn stb_random(&self) -> (){
        
        let mut game_sequence : Vec<i32> = vec![];
        let mut integer_history_map = self.create_hashmap();
        let mut card_state_map = self.create_flip_map();
        let mut goal = self.goal();
        let mut game_step : i32 = 0;
        
        
        while goal != 0{
            
            let mut actions = self.simulate_die();
            
            // random choice strat
            
            let actions_length : i32 = actions.len() as i32;
            let random_choice = actions[random_range(0..actions_length) as usize];
            
            fn validate(actions : &mut Vec<i32> , fm : &mut HashMap<i32,i32> , gh:&mut HashMap<i32,i32> , ca : i32 , goal : &mut i32) -> (){
                let mut actions_ref = ca.clone();
                if *fm.get(&actions_ref).unwrap() == 1 {  // flipped up , ie only valid actions is to flip it down (corresponds with -A)
                    fm.insert(actions_ref,0); // flip card state (ie overwriting fm)
                    actions_ref = -ca; // (-A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be negative
                    
                }else{ // flipped down , ie only valid action is to flip up (corresponds with +A)
                    fm.insert(actions_ref,1); // flip card state (ie overwriting fm)
                    actions_ref = ca; // (+A)
                    *gh.entry(actions_ref).or_insert(0) += 1; // (number of occurences of each integer)
                    *goal += actions_ref;  // updating game goal adding as values gaurenteed to be positive
                }
                
            }
            validate(&mut actions,&mut card_state_map,&mut integer_history_map,random_choice,&mut goal);
            game_step += 1;
            

            
            let state_string = format!("Game Step {} , \n card game_state {:?} ,\n  actions {:?} ,\n chosen_actions {} ,\n goal {}", game_step,card_state_map,actions,random_choice,goal);
            println!("\n {:?}", state_string);
            
        }
        

        
        
        




        
        
        
        
        
        
        
        
        
    }
    
    
    fn stb_optimal() -> () {
        ()
    }
    
    fn stb_MDP() -> () {()}
    
    
    
    
    fn stb_bellman() -> () {()}
    
    fn stb_SS() -> () {()}
    
    


    pub fn end(self, inst: STB) -> () {
        drop(inst);
        
    }
}


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
use rand::rng;
use rand::SeedableRng;
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
    m_die : usize,
    n_sided : usize,
    faced_up : bool
}


impl STB{
    
    pub fn new(n_sided : usize, m_die : usize, faced_up : bool) -> Result<Self, String> {
        if n_sided < 2 || m_die < 2 {
            return Err(" Either m or n is less than 2".to_string());
        }
        Ok(Self { n_sided, m_die, faced_up })
    }
    
    pub fn get_expectation(&self) -> Vec<f32>{
        
        let n : f32 = self.m_die as f32;
        
        let a: i32 = 1;
        let b: i32 = self.n_sided as i32;
        let mean : f32 = ((a + b) / 2) as f32;
        let sum_mean : f32 = (n * mean);
        
        println!(" Single Die Expected Value  {:?}",mean);
        println!(" Sum Die Expected Value {:?}", sum_mean);
        
        vec![mean,sum_mean]
    }
    
    pub fn make_cards(&self) -> Vec<i32> {
        let mut cards : Vec<i32> = vec![];
        let n : i32 = (self.n_sided + 1) as i32;
        for card in 1..n{
            cards.push(card);
        }
        cards
        
    }
    
    pub fn simulate_die(&self) -> Vec<i32>{
        let m : i32 = (self.m_die + 1) as i32;
        let n : i32 = self.n_sided  as i32;
        let mut die_trials : Vec<i32> = vec![];
        
        for trials in 1..m{
            let res : i32 = random_range(1..n);
            die_trials.push(res);
        }
        let sum : i32 = die_trials.iter().sum();
        die_trials.push(sum);
        
        
        die_trials
        
    }
    
    pub fn goal(&self) -> i32{
        let n : i32 = (self.n_sided as i32) * (self.m_die as i32);
        let mut sum : i32 = n * (n + 1);
        sum = sum / 2;
        sum
        
    }
    
    pub fn cartesian_power(&self) -> Vec<Vec<i32>>{
        let n : i32 = self.n_sided as i32;
        let m : i32 = self.m_die as i32;
        
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
    
    pub fn options(&self) -> Vec<i32>{
        let cartesian_power = self.cartesian_power();
        
        
        let mut paths : Vec<i32> = vec![];
        
        let length : usize = cartesian_power.len();
        let inner_length: usize = cartesian_power[0].len();
        
        // fix cartesian power / options function
        
        for i in 0..length{
            for j in 0..inner_length {
                paths.push(-cartesian_power[i][j]); // we can only flip down
            }
            
        }
        paths
        
    }
    
    
    pub fn stb_tree_bruteforce(&self) -> Vec<Vec<i32>> {

        // goal is the sum of integers from [1,...,mn]
        // we will subtract from the goal repeatedly until the value is exactly zero

        let goal: i32 = self.goal();
        let mut goal_ref : i32 = goal;

        let mut game_sequences: Vec<Vec<i32>> = vec![];
        let mut curr_options: Vec<i32> = self.options();
        let mut curr_path: Vec<i32> = vec![];
        let mut curr_parent: i32 = 0;
        let length: usize = curr_options.len();

        fn validate(curr: Vec<i32>, parent: i32) -> Vec<i32> {
            let mut new: Vec<i32> = vec![];
            let n: usize = curr.len();
            for i in 0..n {
                if curr[i] == parent {
                    new.push(-curr[i]);
                } else {
                    new.push(curr[i]);
                }
            }
            new
        }
        
        fn detect_cycle(curr : Vec<Vec<i32>>) -> bool{ // use this function on all parent values and essentially,
            // make every gam a graph and run cycle detection algorithim
            
            true
        }
        
        
        fn dfs(curr : i32, parent : i32 , path : Vec<i32> , game_seq : &mut Vec<Vec<i32>> , curr_options : Vec<i32> , options_length : usize) -> (){
            
            
            
            if curr == 0{
                game_seq.push(path.clone());
                return 
            }
            
            
            for i in 0..options_length{
                
                
                let curr_options = validate(curr_options.clone(),parent);
                let length : usize = curr_options.len();
                let curr_sum : i32 = curr + curr_options[i];
                let curr_parent : i32  = curr_options[i];
                let mut curr_path : Vec<i32> = path.clone();
                curr_path.push(curr_options[i]);
                if curr_options[i] == -parent{ // essentially we are only getting the perfect game , we either need this to have a global scope or ,use a hashset for cycle detection
                    return
                }else{
                    dfs(curr_sum,curr_parent,curr_path,game_seq,curr_options,length);
                }
                
            }
        }
        
        dfs(goal_ref,curr_parent,curr_path,&mut game_sequences,curr_options,length);
        
        game_sequences

    }


    fn stb_tree_optimal() -> () {
        ()
    }
    
    
    
    
    fn stb_simulation(&self,strat:bool) -> f32{
        0.0
        
        
        
    }
    
    
    


    pub fn end(self, inst: STB) -> () {
        drop(inst);
        
    }
}


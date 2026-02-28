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
    n_sided : usize
}


impl STB{
    pub fn new(n_sided : usize) -> Self{
        Self{
            n_sided
        }
    }

    fn get_expectation(&self) -> Vec<f32>{
        let a : f32 = 1.0;
        let b : f32 = self.n_sided as f32;
        let d : f32 = ((a + b) / 2.0);
        let d_add : f32 = (2.0 * d);

        println!(" Expected Value of N-Die {:?}", d);
        println!(" Expected Value of N-Die Sum {:?}",d_add);

        vec![d,d_add]

    }
    
    fn make_cards(&self) -> Vec<i32>{
        let mut n : i32 = self.n_sided as i32;
        let mut max_card : i32 = (n * 2);
        let mut c : i32 = 1;
        let mut cards : Vec<i32> = vec![];
        while c != max_card + 1{
            cards.push(c);
            c += 1;
        }
        cards
    }

    fn cartesian_product(&self) -> Vec<Vec<i32>>{
        let n : i32 = self.n_sided as i32 + 1;
        let mut c_products : Vec<Vec<i32>> = Vec::new();
        for i in 1..n{
            for j in 1..n{
                let prod : Vec<i32> = vec![i,j,i + j];
                c_products.push(prod);
            }
        }
        c_products
    }

    fn die(&self) -> Vec<i32>{
        let n : i32 = (self.n_sided as i32) + 1;
        let d1 = random_range(1..n);
        let d2 = random_range(1..n);
        let sum = d1 + d2;
        vec![d1,d2,sum]
    }
    
    fn goal(&self) -> i32{
        let mut n : i32 = self.n_sided as i32;
        let G = (n * (n + 1)) / 2;
        G
        // Faulhaber's Trick
    }
    
    pub fn stb_tree(&self) -> Vec<Vec<i32>>{
        
        let G = self.goal();
        let mut cards = self.make_cards();
        let mut curr_sum : i32 = 0;
        let mut game_paths : Vec<Vec<i32>> = vec![];
        
        
        fn dfs(curr : i32, goal : i32, curr_cards : &mut Vec<i32> , curr_path : &mut Vec<i32> , paths : &mut Vec<Vec<i32>>, cart_product : &Vec<Vec<i32>>, prev:i32) -> (){
            
            if goal == 0{ // terminating state E_G
                paths.push(curr_path.clone());
                return 
            }
            
            for tup in cart_product{ // 
                // we can just do the three dfs calls 
                let x = tup[0];
                let y = tup[1];
                let z = tup[2]; // z = x + y
                
                // we need to add termination branch when G is in the options
                // most probable strategy from analysis is x + y
                
                // avoiding cycles , 1st part of strategy
                
                
                if x == prev{
                    ()
                }else{
                    
                    dfs()
                }
                if y == prev{
                    ()
                    
                }else{
                    dfs()
                }
                
                if z == prev{
                    ()
                }else{
                    dfs()
                }
                

                
                
            }
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        }
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        vec![]
        
        
        
    }
    
    
    pub fn dfs_strat(self) -> (){
        
        
        
    }
    
    
    
    
    
    
    
    
    pub fn end(self, inst: STB) -> (){
        drop(inst);
    }
    

}


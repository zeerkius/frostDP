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


pub struct CoinChange{
    pub coins : Vec<usize>,
    pub T : i32
}


impl CoinChange{
    pub fn new(coins:Vec<usize>,T:i32) -> Self{
        Self{
            coins,
            T
        }

    }


    pub fn CoinChangeTotalWays(self) -> i32{
        if (self.T == 0) && (self.coins.is_empty()){
            return 1
        }
        if (self.T > 0) && (self.coins.is_empty()){
            return 0
        }

        // statically make dp table , dim(dp) = m x n

        let n : usize = self.coins.len() + 1;
        let m : usize = (self.T + 1) as usize;
        let coins : Vec<usize> = self.coins;


        let mut dp : Vec<Vec<i32>> = vec![vec![0;n];m];

        for j in 0..n{
            dp[0][j] = 1; // essentially expanding dp[0][0] = 1 across all j
        }

        for i in 1..m{
            for j in 1..n{

                dp[i][j] = dp[i][j - 1];

                if i >= coins[j - 1]{
                    dp[i][j] += dp[i - coins[j - 1]][j];
                }

            }
        }

        dp[m - 1][n - 1]

    }
















    pub fn end(self,inst:CoinChange) -> (){
        std::mem::drop(inst);
    }



}
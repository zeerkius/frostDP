# coin-change-analysis
A deep dive into the coin change problem: counting ways, optimizing coins, analyzing contributions, and exploring DP structures with unlimited and bounded coin scenarios.

## Features

### Combinatorial Analysis
- **Total ways** to reach a target `T` using unlimited coins.
- **Most/least contributing coins** across all solutions.
- Probabilistic and structural insights into the DP table.

### Optimization
- **Minimum number of coins** required to reach `T`.
- **Contribution analysis** for coins in optimized solutions.

### Enumeration
- Generate **all combinations** of coins summing to `T`.
- Generate **all distinct sets** of coins summing to `T`.

### Bounded Coin Variants
- **k-limited coins**: analyze the number of ways to reach `T`.
- **Minimum coins with bounded resources**.


## Example Usage

    let c = CoinChange::new(vec![1,2,5],10);
    let res =  c.k_resources(7);
    println!(" values {:?}",res);



# shut-the-box-game

Bob plays a single-player game of chance using two standard 6-sided dice and twelve cards numbered 1 to 12. When the game starts, all cards are placed face up on a table.
Each turn, Bob rolls both dice, getting numbers $x$ and $y$ respectively, each in the range 1,...,6. He must choose amongst three options: turn over card $x$, card $y$, or card $x+y$. (If the chosen card is already face down, it is turned to face up, and vice versa.)
If Bob manages to have all twelve cards face down at the same time, he wins
Alice plays a similar game, except that instead of dice she uses two fair coins, counting heads as 2 and tails as 1, and that she uses four cards instead of twelve. Alice finds that, with the optimal strategy for her game, the expected number of turns taken until she wins is approximately 5.673651
Assuming that Bob plays with an optimal strategy, what is the expected number of turns taken until he wins? Give your answer rounded to 6 places after the decimal point.


## Features 


### Naive Brute Force Traversal of the Game Tree
- **dfs** this will try every possible move in the game tree until the game is terminated `card = [-1,-1...-1]`.
- **dfs_optimal** this will use the analytically deducted optimal strategy to study terminations.
- **expectation-comparison**  we will compare expectations between each-other

## Example Usage

    let stb1 = STB::new(n-sided : 2);
    let res =  stb1.dfs();
    println!(" values {:?}",res); // this will enumerate all terminations or possible outcomes of the game until termination






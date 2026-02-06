
#include <iostream>
#include <vector>


struct CoinChange {
	int T;
	std::vector<int> coins;

	int TotalWays() {

		int n = coins.size() + 1;
		int m = T + 1;

		std::vector<std::vector<int>> dp;
		dp.resize(m, std::vector<int>(n, 0));

		for (int k = 0; k < n; ++k) { // base cases
			dp[0][k] = 1;
		}

		for (int i = 1; i < m; ++i) {
			for (int j = 1; j < n; ++j) {

				dp[i][j] = dp[i][j - 1];

				if (i >= coins[j - 1]) {
					dp[i][j] += dp[i - coins[j - 1]][j];
				}
			}
		}

		return dp[m - 1][n - 1];
	}


	int MaxContribution() {

	}


};



int main() {


    std::vector<int> denominations = { 1,2,5,10,20,50,100,200 };
	CoinChange inst;
	inst.coins = denominations;
	inst.T = 200;
    std::cout << inst.TotalWays() << "\n";

};







































};



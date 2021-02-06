#include <bits/stdc++.h>

using namespace std;

int main() {
    /*--------------INPUT CODE--------------*/ 
    int num_coins_types, total_to_make;
    cin >> num_coins_types >> total_to_make;

    vector<int> coin_types;
    while (num_coins_types--) {
        int x;
        cin >> x;
        coin_types.push_back(x);
    }

    /*---------------DP STUFF---------------*/
    int dp[total_to_make+1]; // This stores the results we calculate

    dp[0] = 0; // Minimum number of coins to make 0p is 0 (base case)

    for (int i = 1; i <= total_to_make; i++) {
        dp[i] = total_to_make + 1; // This is basically infinity
        for (int coin_value : coin_types) {
            if (i >= coin_value) {
                dp[i] = min(dp[i], dp[i - coin_value] + 1);
            }
        }
    }

    // Output answer
    cout << dp[total_to_make] << endl;
}

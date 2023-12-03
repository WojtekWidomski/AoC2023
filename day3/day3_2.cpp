#include <bits/stdc++.h>
using namespace std;

#define N 140
int m;

vector<string> sch;
vector<vector<bool>> number_used;

bool is_number(char c) {
    return (c >= '0' && c <= '9');
}

// Try to get number with one digit at i, j and replace it with '.'
// and increment success by 1 if successfully got number.
int get_number(int i, int j, int &success) {
    if (i < 0 || i >= N || j < 0 || j >= m) {
        return 1;
    }

    if (number_used[i][j] || !is_number(sch[i][j])) {
        return 1;
    }

    int min_j = j;
    int max_j = j;

    while (min_j > 0 && is_number(sch[i][min_j-1])) {
        min_j--;
    }
    while (max_j < m-1 && is_number(sch[i][max_j+1])) {
        max_j++;
    }

    int number = 0;
    int p = 1;

    for (int k = max_j; k >= min_j; k--) {
        number += (sch[i][k] - '0')*p;
        p*=10;
        number_used[i][k] = true;
    }
    success++;
    return number;
}

// Find all adjacent numbers, calculate sum and replace them with '.'
int adj_sum(int i, int j) {
    int result = 1;
    int adj_num = 0;
    number_used.assign(N, vector<bool>(m, false));
    result *= get_number(i-1, j-1, adj_num);
    result *= get_number(i-1, j, adj_num);
    result *= get_number(i-1, j+1, adj_num);
    result *= get_number(i, j-1, adj_num);
    result *= get_number(i, j+1, adj_num);
    result *= get_number(i+1, j-1, adj_num);
    result *= get_number(i+1, j, adj_num);
    result *= get_number(i+1, j+1, adj_num);
    if (adj_num == 2){
        return result;
    }
    return 0;
}

int main() {
    for (int i = 0; i < N; i++) {
        string s;
        cin >> s;
        sch.push_back(s);
    }

    m = sch[0].size();

    int ratio_sum = 0;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < m; j++) {
            if (sch[i][j] == '*') {
                ratio_sum += adj_sum(i, j);
            }
        }
    }

    cout << ratio_sum << "\n";

    return 0;
}
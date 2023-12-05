#include <bits/stdc++.h>
using namespace std;

int n;
vector<pair<long long, long long>> seed_ranges;

int main() {
    cin >> n;
    for (int i = 0; i < n; i++){
        long long start, length;
        cin >> start >> length;
        seed_ranges.push_back(make_pair(start, length));
    }



    return 0;
}
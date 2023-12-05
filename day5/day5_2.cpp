#include <bits/stdc++.h>
using namespace std;

// This is C++ solution, that uses converted input (inputcpp.in)
// It is still brutal solution, but with -O3, on 7700X, it runs
// under 2 minutes (and probably only a few minutes longer on older CPUs).

#define map_number 7

struct maprange {
    long long dest_start;
    long long src_start;
    long long length;
};

int n;
vector<pair<long long, long long>> seed_ranges;
vector<vector<maprange>> maps(map_number);

long long get_from_map(int map_idx, long long x) {
    for (maprange r : maps[map_idx]) {
        if (x >= r.src_start && x < r.src_start + r.length){
            return x + r.dest_start - r.src_start;
        }
    }
    return x;
}

int main() {
    cin >> n;
    for (int i = 0; i < n/2; i++){
        long long start, length;
        cin >> start >> length;
        seed_ranges.push_back(make_pair(start, length));
    }

    for (int i = 0; i < map_number; i++) {
        int ranges;
        cin >> ranges;
        for (int j = 0; j < ranges; j++) {
            maprange range;
            cin >> range.dest_start >> range.src_start >> range.length;
            maps[i].push_back(range);
        }
    }

    long long min_loc = LLONG_MAX;

    for (pair<long long, long long> r : seed_ranges) {
        // Show current range:
        // cerr << "(" << r.first << ", " << r.second << ")\n"; 
        for (long long s = r.first; s < r.first + r.second; s++) {
            // Show progress
            // if (s%10000000 == 0) {
            //     cerr << (100*(s-r.first))/(r.second) << "%\n";
            // }
            long long loc = s;
            for (int i = 0; i < map_number; i++) {
                loc = get_from_map(i, loc);
            }

            if (loc < min_loc) {
                min_loc = loc;
            }
        }
    }

    cout << min_loc << "\n";

    return 0;
}

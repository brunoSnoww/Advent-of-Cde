#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
using namespace std;
typedef vector<char> vc;

void solve(int N, const string &input)
{
    unordered_map<char, int> mp;
    int tail = 0;
    for (int i = 0; i < input.size(); ++i)
    {
        char c = input[i];
        if (i < N)
        {
            mp[c]++;
            continue;
        }

        if (mp.size() == N)
        {
            cout << i << endl;
            break;
        }
        mp[input[tail]]--;
        if (mp[input[tail]] == 0)
            mp.erase(input[tail]);
        mp[c]++;
        tail++;
    }
}

int main(void)
{
    string input;
    cin >> input;
    solve(4, input);
    solve(14, input);
    return 0;
}

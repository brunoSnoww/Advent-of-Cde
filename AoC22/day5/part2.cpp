#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <unordered_set>
#include <queue>
using namespace std;
typedef vector<char> vc;
int main(void)
{
    string input;
    vector<vc> towers(10);
    int test = 8;
    while (test--)
    {
        getline(cin, input);
        for (int i = 0, j = 1; i < input.size(); j++, i += 2)
        {
            if (input[i] != ',')
                towers[j].push_back(input[i]);
        }
    }
    for (vc &v : towers)
        reverse(begin(v), end(v));

    int t, from, to;
    while (scanf("move %d from %d to %d\n", &t, &from, &to) != EOF)
    {
        vc buffer;
        while (t--)
        {
            int x = towers[from].back();
            towers[from].pop_back();
            buffer.push_back(x);
        }
        for (int i = buffer.size() - 1; i >= 0; i--)
            towers[to].push_back(buffer[i]);
    }

    string ans = "";
    for (int i = 1; i < towers.size(); i++)
        ans += towers[i][towers[i].size() - 1];
    cout << ans << endl;
    cout << "\n";
    return 0;
}

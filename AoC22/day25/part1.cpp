#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
#include <map>
typedef int64_t ll;
#include <vector>
#include <fstream>
using namespace std;
#include <cstdlib>
#include <unistd.h>

string input;

ll convert(string s)
{
    ll ans = 0;
    int n = s.size();
    for (int i = n - 1; i >= 0; i--)
    {
        if (isdigit(s[i]))
        {
            ans += (ll)pow(5, n - 1 - i) * (ll)(s[i] - '0');
        }
        else
        {
            ll p = s[i] == '-' ? -1 : -2;
            ans += (ll)pow(5, n - 1 - i) * p;
        }
    }
    return ans;
}

int main(void)
{

    string input;
    ll val = 0;
    while (getline(cin, input))
    {
        val += convert(input);
    }
    cout << val << endl;
    vector<char> newBase = {'=', '-', '0', '1', '2'};
    ll nv = 0;
    string ans = "";
    while (val != 0)
    {
        nv = val + 2;
        val = nv / 5;
        ans.insert(0, 1, newBase[nv % 5]);
    }
    cout << ans << endl;
    return 0;
}

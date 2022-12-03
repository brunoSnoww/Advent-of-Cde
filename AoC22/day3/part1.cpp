#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <unordered_set>
#include <queue>
using namespace std;

int main(void)
{
    string input;
    int ans = 0;
    while (getline(cin, input))
    {
        int n = input.size();
        string s1 = input.substr(0, n / 2);
        string s2 = input.substr(n / 2);
        unordered_set<char> seen;
        for (char c : s1)
            seen.insert(c);

        for (char c : s2)
        {
            if (seen.count(c))
            {

                if (isupper(c))
                {
                    ans += (c - 'A') + 27;
                }
                else
                {
                    ans += (c - 'a') + 1;
                }
                break;
            }
        }
    }
    cout << ans << endl;
}
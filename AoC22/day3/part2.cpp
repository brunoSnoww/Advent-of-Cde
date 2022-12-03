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
    unordered_set<char> processed;
    string s1, s2, s3;
    while (!cin.eof())
    {
        cin >> s1 >> s2 >> s3;
        unordered_set<char> seen1, seen2;
        for (char c : s1)
            seen1.insert(c);
        for (char c : s2)
        {
            if (seen1.count(c))
            {
                seen2.insert(c);
            }
        }
        for (char c : s3)
        {
            if (seen2.count(c))
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
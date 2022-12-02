#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <unordered_map>
#include <queue>
using namespace std;

const int ME = 0;
const int OTHER = 1;
const int DRAW = 2;

int winner(char me, char other)
{
    if (me == 'X' && other == 'A' || me == 'Y' && other == 'B' || me == 'Z' && other == 'C')
        return DRAW;
    if (me == 'X')
    {
        return other == 'B' ? OTHER : ME;
    }
    else if (me == 'Y')
    {
        return other == 'C' ? OTHER : ME;
    }
    else
        return other == 'A' ? OTHER : ME;
}

int main(void)
{

    unordered_map<char, int> scores;
    scores['A'] = scores['X'] = 1;
    scores['B'] = scores['Y'] = 2;
    scores['C'] = scores['Z'] = 3;
    scores[ME] = 6;
    scores[OTHER] = 0;
    scores[DRAW] = 3;
    int ans = 0;
    string input;
    char me, other;
    while (!cin.eof())
    {
        getline(cin, input);
        other = input[0], me = input[2];
        int wins = winner(me, other);
        ans += scores[wins] + scores[me];
    }
    cout << ans << endl;
}
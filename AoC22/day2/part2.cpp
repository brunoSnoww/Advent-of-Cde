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
    if (me == 'X')
    {
        return OTHER;
    }
    else if (me == 'Y')
    {
        return DRAW;
    }
    else
        return ME;
}

char choose(char c, int winner)
{
    if (winner == ME || winner == OTHER)
    {
        if (c == 'A')
            return winner == ME ? 'B' : 'C';
        if (c == 'B')
            return winner == ME ? 'C' : 'A';
        return winner == ME ? 'A' : 'B';
    }
    return c;
}

int main(void)
{

    unordered_map<char, int> scores;
    scores['A'] = 1;
    scores['B'] = 2;
    scores['C'] = 3;
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
        char c = choose(other, wins);
        ans += scores[wins] + scores[c];
    }
    cout << ans << endl;
}
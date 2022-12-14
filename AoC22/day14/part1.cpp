#include <stdio.h>
#include <iostream>
#include <map>
#include <string.h>
#include <set>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;
typedef pair<int, int> ii;
set<ii> rocks;
set<ii> sands;
int SOURCE_Y = 500;
int SOURCE_X = 0;
string input;
int LIMIT = -1;
vector<string> tokenize(string s, string del = " ")
{
    int start = 0;
    int end = s.find(del);
    vector<string> strs;
    while (end != -1)
    {
        strs.push_back(s.substr(start, end - start));
        start = end + del.size();
        end = s.find(del, start);
    }
    strs.push_back(s.substr(start, end - start));

    return strs;
}

void drawRocks()
{
    while (getline(cin, input))
    {
        vector<string> tokens = tokenize(input);

        ii start(stoi(tokens[1]), stoi(tokens[0]));

        for (int i = 2; i < tokens.size(); i += 2)
        {
            int y = stoi(tokens[i]);
            int x = stoi(tokens[i + 1]);
            LIMIT = max(LIMIT, x);
            ii next(x, y);
            if (start.first == next.first)
            {
                int dy = start.second < next.second ? 1 : -1;
                for (int j = start.second; j != next.second + dy; j += dy)
                {
                    rocks.insert(ii(start.first, j));
                }
            }
            else
            {
                int dx = start.first < next.first ? 1 : -1;
                for (int i = start.first; i != next.first + dx; i += dx)
                {
                    rocks.insert(ii(i, start.second));
                }
            }
            start = next;
        }
    }
}

bool moveSand(ii &sand)
{
    int x = sand.first, y = sand.second;
    ii p1(x + 1, y), p2(x + 1, y - 1), p3(x + 1, y + 1);
    // cout << x << " " << y << endl;

    if (!rocks.count(p1) && !sands.count(p1))
    {
        sand = p1;
        return true;
    }
    if (!rocks.count(p2) && !sands.count(p2))
    {
        sand = p2;
        return true;
    }
    if (!rocks.count(p3) && !sands.count(p3))
    {
        sand = p3;
        return true;
    }
    sands.insert(sand);
    return false;
}

int main(void)
{
    drawRocks();

    int rep = 0;
    while (true)
    {
        ii sand(SOURCE_X, SOURCE_Y);
        while (moveSand(sand))
        {
            if (sand.first > LIMIT)
                break;
        }
        if (sand.first > LIMIT)
            break;
        rep++;
    }
    cout << rep << endl;

    return 0;
}
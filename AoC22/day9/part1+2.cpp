#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;
typedef pair<int, int> ii;
set<ii> part1, part2;
int xH, yH;
void moveHead(char c)
{
    if (c == 'R')
    {
        xH += 1;
    }
    if (c == 'L')
    {
        xH -= 1;
    }
    if (c == 'U')
    {
        yH -= 1;
    }
    if (c == 'D')
    {
        yH += 1;
    }
}
void moveTail(int x, int y, int &xT, int &yT)
{

    int dx = x - xT, dy = y - yT;
    if (abs(dx) <= 1 && abs(dy) <= 1)
        return;
    dx = x > xT ? 1 : x == xT ? 0
                              : -1;
    dy = y > yT ? 1 : y == yT ? 0
                              : -1;
    xT += dx;
    yT += dy;
}

int main(void)
{
    char c;
    int m;
    xH = 0, yH = 0;
    vector<ii> tails(9, ii(0, 0));

    while (scanf("%c %d\n", &c, &m) != EOF)
    {
        while (m--)
        {
            moveHead(c);

            // the head of knot 0 is the head of the rope
            moveTail(xH, yH, tails[0].first, tails[0].second);
            for (int i = 0; i + 1 < tails.size(); ++i)
            {
                // every knot is the "head" of the following knot in the rope
                moveTail(tails[i].first, tails[i].second, tails[i + 1].first, tails[i + 1].second);
            }
            part1.insert(ii(tails[0].first, tails[0].second));
            part2.insert(ii(tails.back().first, tails.back().second));
        }
    }
    cout << "PART1: " << part1.size() << endl;
    cout << "PART2: " << part2.size() << endl;
    return 0;
}
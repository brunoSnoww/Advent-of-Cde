#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <unordered_set>
#include <queue>
using namespace std;

bool pointsIsInside(int x, int i1, int i2)
{
    return i1 <= x && x <= i2;
}

bool overlaps(int x1, int x2, int y1, int y2)
{
    return pointsIsInside(x1, y1, y2) ||
           pointsIsInside(x2, y1, y2) ||
           pointsIsInside(y1, x1, x2) ||
           pointsIsInside(y2, x1, x2);
}

bool contains(int x1, int x2, int y1, int y2)
{
    return x1 <= y1 && x2 >= y2 || y1 <= x1 && y2 >= x2;
}

int main(void)
{
    int x1, x2, y1, y2;
    int part1 = 0, part2 = 0;
    while (scanf("%d-%d,%d-%d\n", &x1, &x2, &y1, &y2) != EOF)
    {
        if (contains(x1, x2, y1, y2))
        {
            part1++;
        }
        if (overlaps(x1, x2, y1, y2))
        {
            part2++;
        }
    }
    cout << "PART1: " << part1 << endl;
    cout << "PART2: " << part2 << endl;
}
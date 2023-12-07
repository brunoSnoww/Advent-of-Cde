#include <stdio.h>
#include <iostream>
#include <string.h>
#include <vector>
using namespace std;

typedef pair<int, int> ii;
int INF = 1e7 + 7;
vector<ii> point;
vector<int> dist;

void search(int x, int y)
{
    if (x >= 0 && x <= 4000000 && y >= 0 && y <= 4000000)
    {
        bool b = true;
        for (int j = 0; j < dist.size(); j++)
            if (abs(point[j].first - x) + abs(point[j].second - y) <= dist[j])
                b = false;
        if (b)
        {
            cout << x << ' ' << y;
            exit(0);
        }
    }
}

int main(void)
{
    int xs, ys, xb, yb;

    while (scanf("Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d\n", &xs, &ys, &xb, &yb) != EOF)
    {
        int dx = abs(xs - xb), dy = abs(ys - yb);
        dist.push_back(dx + dy);
        point.push_back(ii(xs, ys));
    }

    return 0;
}
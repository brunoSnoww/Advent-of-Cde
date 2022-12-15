#include <stdio.h>
#include <iostream>
#include <string.h>
#include <unordered_set>
#include <vector>
using namespace std;

typedef pair<int, int> ii;
int INF = 1e7 + 7;
int ROW = 2000000;

struct hashFunction
{
    size_t operator()(const pair<int,
                                 int> &x) const
    {
        return x.first ^ x.second;
    }
};

int main(void)
{
    int xs, ys, xb, yb;
    vector<ii> point;
    vector<int> dist;
    unordered_set<ii, hashFunction> bacons;
    while (scanf("Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d\n", &xs, &ys, &xb, &yb) != EOF)
    {
        bacons.insert(ii(xb, yb));
        int dx = abs(xs - xb), dy = abs(ys - yb);
        dist.push_back(dx + dy);
        point.push_back(ii(xs, ys));
    }
    unordered_set<ii, hashFunction> count;

    for (int x = -INF; x < INF; ++x)
    {
        for (int i = 0; i < dist.size(); i++)
        {
            int d = abs(point[i].first - x) + abs(point[i].second - ROW);
            if (d <= dist[i] && !bacons.count(ii(x, ROW)))
            {
                count.insert(ii(x, ROW));
            }
        }
    }
    cout << count.size() << endl;
    return 0;
}
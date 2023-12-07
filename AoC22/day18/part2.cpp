#include <stdio.h>
#include <iostream>
#include <string.h>
#include <map>
#include <set>
#include <vector>
#include <cstdlib>
#include <unistd.h>

using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef pair<int, ii> iii;

vector<int> dx = {0, 0, -1, 1};
vector<int> dy = {-1, 1, 0, 0};

void dfs(int &c, int x, int y, int z, map<ii, vector<int>> &mp, set<iii> &vis, map<iii, int> &gaps)
{
    if (vis.count(iii(x, ii(y, z))))
        return;
    vis.insert(iii(x, ii(y, z)));
    bool isEdge = false;
    for (int k = 0; k < 4; k++)
    {
        int nx = x + dx[k];
        int ny = y + dy[k];
        if (!mp.count(ii(nx, ny)))
        {
            isEdge = true;
            false;
        }
    }
}

int main(void)
{
    int N = 13;
    int x, y, z;
    map<ii, vector<int>> mp;
    while (scanf("%d,%d,%d\n", &x, &y, &z) != EOF)
    {
        mp[ii(x, y)].push_back(z);
    }
    for (auto it = mp.begin(); it != mp.end(); it++)
        sort(begin(mp[it->first]), end(mp[it->first]));
    int area = 0;
    for (auto it = mp.begin(); it != mp.end(); it++)
    {
        int x = it->first.first, y = it->first.second;
        vector<int> z = it->second;
        for (int j = 0; j < z.size(); j++)
        {
            // above
            if (j + 1 == z.size())
                area++;
            else if (z[j + 1] != z[j] + 1)
                area++;
            // down
            if (j == 0)
                area++;
            else if (z[j - 1] != z[j] - 1)
                area++;
            // sides
            for (int k = 0; k < 4; k++)
            {
                int nx = x + dx[k];
                int ny = y + dy[k];
                ii tmp = ii(nx, ny);
                // side
                if (!mp.count(tmp))
                    area++;
                else
                {
                    if (!binary_search(begin(mp[tmp]), end(mp[tmp]), z[j]))
                        area++;
                }
            }
        }
    }

    cout << area << endl;
    return 0;
}

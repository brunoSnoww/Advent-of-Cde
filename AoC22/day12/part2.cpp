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
vector<string> matrix;
vector<int> dx = {0, 0, 1, -1};
vector<int> dy = {1, -1, 0, 0};
int xS, yS, xE, yE;
int N, M;
bool isValid(int x, int y) { return x >= 0 && x < N && y >= 0 && y < M; }

int bfs(const vector<string> &matrix, ii point)
{
    queue<ii> q;
    q.push(point);
    vector<vector<int>> d;
    d.resize(N, vector<int>(M, -1));
    while (!q.empty())
    {
        ii front = q.front();
        q.pop();
        int x = front.first, y = front.second;
        for (int k = 0; k < 4; ++k)
        {
            int nx = x + dx[k], ny = y + dy[k];
            if (isValid(nx, ny) && d[nx][ny] == -1)
            {
                int d1 = matrix[x][y] - 'a', d2 = matrix[nx][ny] - 'a';
                if (d2 <= d1 + 1)
                {
                    d[nx][ny] = d[x][y] + 1;
                    q.push(ii(nx, ny));
                }
            }
        }
    }
    return d[xE][yE];
}

int main(void)
{
    string input;
    int row = 0;
    vector<ii> points;
    while (!cin.eof())
    {
        getline(cin, input);
        for (int i = 0; i < input.size(); ++i)
        {
            if (input[i] == 'S')
            {
                xS = row;
                yS = i;
                input[i] = 'a';
            }
            if (input[i] == 'E')
            {
                xE = row;
                yE = i;
                input[i] = 'z';
            }
            if (input[i] == 'a')
                points.push_back(ii(row, i));
        }
        matrix.push_back(input);
        row++;
    }

    N = matrix.size(), M = matrix[0].size();
    int ans = INT_MAX;

    for (ii p : points)
    {
        int dist = bfs(matrix, p);
        if (dist != -1)
            ans = min(dist, ans);
    }

    cout << ans + 1 << endl;

    return 0;
}
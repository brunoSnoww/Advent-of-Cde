#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;

vector<int> dx = {0, 0, 1, -1};
vector<int> dy = {1, -1, 0, 0};
int ans = 0, N, M;
bool isEdge(int x, int y) { return x == 0 || x == N - 1 || y == 0 || y == M - 1; }
bool isValid(int x, int y) { return x >= 0 && x < N && y >= 0 && y < M; }

void dfs(const vector<string> &matrix, vector<vector<bool>> &vis, int x, int y)
{
    if (!isValid(x, y) || vis[x][y])
        return;
    vis[x][y] = true;
    int val = matrix[x][y] - '0';
    if (isEdge(x, y))
        ans++;
    else
    {
        // far,far,far way from ideal, but for small input size , whatever

        bool top = true, left = true, right = true, bottom = true;
        for (int i = x - 1; i >= 0; i--)
        {
            if (matrix[i][y] >= matrix[x][y])
                top = false;
        }
        for (int i = x + 1; i < N; i++)
        {
            if (matrix[i][y] >= matrix[x][y])
                bottom = false;
        }
        for (int i = y - 1; i >= 0; i--)
        {
            if (matrix[x][i] >= matrix[x][y])
                left = false;
        }
        for (int i = y + 1; i < M; i++)
        {
            if (matrix[x][i] >= matrix[x][y])
                right = false;
        }
        if (left || right || top || bottom)
        {
            ans++;
        }
    }
    for (int i = 0; i < 4; i++)
    {
        int nx = x + dx[i], ny = y + dy[i];
        dfs(matrix, vis, nx, ny);
    }
}

int main(void)
{
    vector<string> matrix;
    string input;
    while (getline(cin, input))
    {
        matrix.push_back(input);
    }
    N = matrix.size(), M = matrix[0].size();
    vector<vector<bool>> vis(N, vector<bool>(M, false));
    dfs(matrix, vis, 0, 0);
    cout << ans << endl;
}
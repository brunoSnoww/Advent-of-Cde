#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
#include <map>

#include <vector>
#include <fstream>
using namespace std;
#include <cstdlib>
#include <unistd.h>
vector<string> matrix;
typedef pair<int, int> ii;
typedef pair<ii, int> iii;
typedef pair<ii, ii> iiii;

vector<int> dx = {1, -1, 0, 0, 0};
vector<int> dy = {0, 0, 1, -1, 0};
int n, m, T;
map<ii, vector<char>> blizzards;
unordered_map<int, map<ii, vector<char>>> timeToGridMap;

int gcd(int x, int y)
{
    return y == 0 ? x : gcd(y, x % y);
}

int lcm(int x, int y)
{
    return (x * y) / (gcd(x, y));
}

void print(map<ii, vector<char>> &blizzards, int x, int y)
{
    // printf("\033[2J\033[1;1H"); // reset terminal

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < m; ++j)
        {
            if (i == x && j == y)
            {
                cout << 'E';
                continue;
            }
            if (i == 0 || i == n - 1 || j == 0 || j == m - 1)
            {
                cout << '#';
                continue;
            }
            vector<char> currentSpot = blizzards[ii(i, j)];
            if (currentSpot.size() == 0)
            {
                cout << '.';
                continue;
            }
            for (int k = 0; k < currentSpot.size(); k++)
            {
                if (currentSpot.size() > 1)
                {
                    cout << currentSpot.size();
                    break;
                }
                else
                    cout << currentSpot[0];
            }
        }
        cout << endl;
    }
}

int bfs(ii start, ii end, int time)
{
    queue<iii> q;
    q.push(iii(start, time));
    set<iii> visited;

    while (true)
    {
        iii next = q.front();
        q.pop();
        int x = next.first.first, y = next.first.second, time = next.second;
        if (visited.count(iii(ii(x, y), time % T)))
            continue;
        visited.insert(iii(ii(x, y), time % T));

        map<ii, vector<char>> nextBlizzards = timeToGridMap[(time + 1) % T];
        for (int k = 0; k < 5; ++k)
        {
            int nx = x + dx[k], ny = y + dy[k];
            if (ii(nx, ny) == end)
            {
                return time + 1;
            }
            if (ii(nx, ny) == start || !(nx <= 0 || nx >= n - 1 || ny <= 0 || ny >= m - 1))
            {
                if (nextBlizzards[ii(nx, ny)].size() == 0)
                {
                    q.push(iii(ii(nx, ny), time + 1));
                }
            }
        }
    }
}

void move()
{
    map<ii, vector<char>> newBlizzards;

    for (int i = 1; i < n - 1; i++)
    {
        for (int j = 1; j < m - 1; ++j)
        {
            vector<char> currentSpot = blizzards[ii(i, j)];
            for (char c : currentSpot)
            {
                if (c == '>')
                {
                    int newCol = j + 1 == m - 1 ? 1 : j + 1;
                    newBlizzards[ii(i, newCol)].push_back('>');
                }
                if (c == '<')
                {
                    int newCol = j - 1 == 0 ? m - 2 : j - 1;
                    newBlizzards[ii(i, newCol)].push_back('<');
                }
                if (c == 'v')
                {
                    int newRow = i + 1 == n - 1 ? 1 : i + 1;
                    newBlizzards[ii(newRow, j)].push_back('v');
                }
                if (c == '^')
                {
                    int newRow = i - 1 == 0 ? n - 2 : i - 1;
                    newBlizzards[ii(newRow, j)].push_back('^');
                }
            }
        }
    }
    blizzards = newBlizzards;
}

int main(void)
{
    vector<string> grid;
    ifstream inFile("input.txt");
    string input;
    while (getline(inFile, input))
    {
        grid.push_back(input);
    }
    n = grid.size(), m = grid[0].size();
    for (int i = 1; i < n - 1; i++)
    {
        for (int j = 1; j < m - 1; ++j)
        {

            if (grid[i][j] != '.')
                blizzards[ii(i, j)].push_back(grid[i][j]);
        }
    }
    T = lcm(n - 2, m - 2);
    for (int i = 0; i < T; i++)
    {
        timeToGridMap[i] = blizzards;
        move();
    }
    ii start(0, 1);
    ii end(n - 1, m - 2);
    int time1 = bfs(start, end, 0);
    int time2 = bfs(end, start, time1);
    int time3 = bfs(start, end, time2);
    cout << "Part1: " << time1 << endl;
    cout << "Part2: " << time3 << endl;
}
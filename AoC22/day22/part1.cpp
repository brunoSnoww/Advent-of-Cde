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
unordered_map<int, int> initPos;
typedef pair<int, int> ii;
int c = 0;
void move(int &row, int &col, vector<string> &grid, string dir)
{

    if (dir == "R")
    {
        int M = grid[row].size();
        int nextCol;
        if (col + 1 == M || grid[row][col + 1] == ' ')
            nextCol = initPos[row];
        else
            nextCol = col + 1;
        if (grid[row][nextCol] == '#')
            return;
        else
            col = nextCol;
    }
    if (dir == "L")
    {
        int M = grid[row].size();
        int nextCol;
        if (col - 1 < 0 || grid[row][col - 1] == ' ')
        {
            int k = M - 1;
            while (grid[row][k] == ' ')
                k--;
            col = k;
            return;
        }
        else
            nextCol = col - 1;
        if (grid[row][nextCol] == '#')
            return;
        else
            col = nextCol;
    }
    if (dir == "U")
    {
        int nextRow;
        if (row - 1 >= 0 && grid[row - 1][col] == '.')
        {
            row = row - 1;
            return;
        }
        if (row - 1 < 0 || grid[row - 1][col] == ' ')
        {
            int k = row;
            while (grid[k][col] != ' ')
            {
                k++;
                if (k == grid.size())
                    break;
            }
            row = k - 1;
        }
    }
    if (dir == "D")
    {
        int N = grid.size();
        if (row + 1 < N && grid[row + 1][col] == '.')
        {
            row = row + 1;
            return;
        }
        if (row + 1 == N || grid[row + 1][col] == ' ')
        {
            int k = row;
            while (grid[k][col] != ' ')
            {
                k--;
                if (k < 0)
                    break;
            }
            row = k + 1;
        }
    }
}

string nextDir(string currDir, string command)
{
    if (currDir == "R")
    {
        return command == "R" ? "D" : "U";
    }
    if (currDir == "L")
    {
        return command == "R" ? "U" : "D";
    }
    if (currDir == "U")
    {
        return command == "R" ? "R" : "L";
    }

    return command == "R" ? "L" : "R";
}

void print(vector<string> &g, int x, int y, string dir)
{
    printf("\033[2J\033[1;1H"); // reset terminal

    int n = g.size();
    ;
    for (int i = max(x - 20, 0); i < min(x + 20, n); i++)
    {
        int m = g[i].size();
        for (int j = 0; j < m; j++)
        {
            if (i == x && j == y)
            {
                if (dir == "R")
                    cout << '>';
                if (dir == "L")
                    cout << '<';
                if (dir == "U")
                    cout << '^';
                if (dir == "D")
                    cout << 'V';
            }
            else
                cout << g[i][j];
        }
        cout << endl;
    }
    usleep(5000); // sample test
}

int main(void)
{
    ifstream inFile("biggrid.txt");
    string line;
    vector<string> grid;
    int row = 0;
    int maxSize = -1;
    while (getline(inFile, line))
    {
        int i = 0;
        while (line[i] == ' ')
        {
            i++;
        }

        initPos[row++] = i;
        grid.push_back(line);
        maxSize = max(maxSize, (int)line.size());
    }
    for (auto &s : grid)
    {
        if (s.size() < maxSize)
        {
            int offset = maxSize - s.size();
            while (offset--)
            {
                s.push_back(' ');
            }
        }
    }

    vector<string> commands;
    ifstream inFile2("bigcommands.txt");
    getline(inFile2, line);
    string s = "";
    int i = 0;
    while (i < line.size())
    {
        if (isdigit(line[i]))
            s.push_back(line[i]);
        else
        {
            if (s != "")
                commands.push_back(s);
            commands.push_back(string(1, line[i]));
            s.clear();
        }
        i++;
    }

    if (s != "")
        commands.push_back(s);
    string currDir = "R";
    int x = 0;
    int y = initPos[x];
    for (string c : commands)
    {
        if (c == "R" || c == "L")
        {
            currDir = nextDir(currDir, c);
        }
        else
        {
            int t = stoi(c);
            while (t--)
            {
                move(x, y, grid, currDir);
                // print(grid, x, y, currDir);
            }
        }
    }
    int p = currDir == "R" ? 0 : currDir == "L" ? 2
                             : currDir == "D"   ? 1
                                                : 3;
    cout << 1000 * (x + 1) + 4 * (y + 1) + p << endl;
}
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

typedef pair<int, int> ii;
int n, m;
set<ii> elfs;
vector<string> grid;
int P;
const int NORTH = 0;
const int SOUTH = 1;
const int WEST = 2;
const int EAST = 3;
map<ii, int> posCount;
map<ii, ii> nextPos;
int prioritiesRound = 0;
vector<int> priorities = {NORTH, SOUTH, WEST, EAST};
vector<int> dx = {0, 0, 1, -1, 1, 1, -1, -1};
vector<int> dy = {1, -1, 0, 0, 1, -1, 1, -1};
int L = INT_MAX, R = INT_MIN, U = INT_MAX, D = INT_MIN;
void print()
{
    printf("\033[2J\033[1;1H"); // reset terminal

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < m; ++j)
        {
            if (elfs.count(ii(i, j)))
                cout << '#';
            else
                cout << '.';
        }
        cout << endl;
    }
    puts("");
    fflush(stdout);

    usleep(1000000); // sample test
}

vector<vector<ii>> checkPositions(ii elf)
{
    int x = elf.first, y = elf.second;
    vector<vector<ii>> res;
    res.push_back({ii(x - 1, y), ii(x - 1, y - 1), ii(x - 1, y + 1)});
    res.push_back({ii(x + 1, y), ii(x + 1, y - 1), ii(x + 1, y + 1)});
    res.push_back({ii(x, y - 1), ii(x - 1, y - 1), ii(x + 1, y - 1)});
    res.push_back({ii(x, y + 1), ii(x + 1, y + 1), ii(x - 1, y + 1)});
    rotate(begin(res), begin(res) + P, end(res));
    return res;
}

void updatePriorities()
{
    P = (P + 1) % 4;
    rotate(begin(priorities), begin(priorities) + 1, end(priorities));
}

ii getPos(ii elf, int idx)
{
    int x = elf.first, y = elf.second;
    int p = priorities[idx];
    if (p == NORTH)
    {
        return ii(x - 1, y);
    }
    if (p == SOUTH)
    {
        return ii(x + 1, y);
    }
    if (p == WEST)
    {
        return ii(x, y - 1);
    }
    return ii(x, y + 1);
}

void moveElf(ii elf)
{
    int x = elf.first, y = elf.second;
    int c = 0;
    for (int k = 0; k < 8; k++)
    {
        int nx = x + dx[k], ny = y + dy[k];
        if (nx < 0 || nx == n || ny < 0 || ny == m || !elfs.count(ii(nx, ny)))
            c++;
    }
    if (c == 8)
    {
        posCount[elf] = 1;
        return;
    }
    vector<vector<ii>> possibleDirs = checkPositions(elf);
    int idx = 0;
    c = 0;
    for (vector<ii> p : possibleDirs)
    {
        int c = 0;
        for (ii dir : p)
        {
            if (!elfs.count(dir))
            {
                c++;
            }
        }
        if (c == 3)
        {
            ii newElf = getPos(elf, idx);
            posCount[newElf]++;
            nextPos[elf] = newElf;
            return;
        }

        idx++;
    }
    posCount[elf] = 1;
}

void roundSix()
{
    posCount.clear();
    nextPos.clear();
    set<ii> newElfs;
    for (ii elf : elfs)
    {
        moveElf(elf);
    }
    for (auto it = begin(posCount); it != end(posCount); it++)
    {
        int count = it->second;
        if (count > 1)
        {
            for (ii elf : elfs)
                if (nextPos[elf] == it->first)
                    newElfs.insert(elf);
        }
        else
        {
            newElfs.insert(it->first);
        }
    }
    elfs = newElfs;
    for (ii e : elfs)
    {
        L = min(L, e.second);
        R = max(R, e.second);
        U = min(U, e.first);
        D = max(D, e.first);
    }
    updatePriorities();
}

bool finish()
{
    for (ii p : elfs)
    {
        if (!(p.first == 0 || p.first == n - 1 || p.second == 0 || p.second == m - 1))
            return false;
    }
    return true;
}

int main(void)
{
    ifstream inFile("input.txt");
    string input;
    int r = 0;
    while (getline(inFile, input))
    {
        for (int c = 0; c < input.size(); ++c)
        {
            m = input.size();
            if (input[c] == '#')
                elfs.insert(ii(r, c));
        }
        r++;
    }
    n = r;

    for (int i = 0; i < 12; ++i)
    {
        roundSix();
    }

    cout << (D - U + 1) * (R - L + 1) - elfs.size() << endl;

    return 0;
}
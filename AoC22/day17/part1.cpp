#include <stdio.h>
#include <iostream>
#include <string.h>
#include <unordered_set>
#include <vector>
#include <cstdlib>
#include <unistd.h>

using namespace std;
typedef long long ll;
struct hashFunction
{
    size_t operator()(const pair<int,
                                 int> &x) const
    {
        return x.first ^ x.second;
    }
};

ll HEIGHT = 3;
ll WIDTH = 7;
typedef pair<ll, ll> ii;
unordered_set<ii, hashFunction> rocks;

vector<ii> getShape(int i)
{
    vector<ii> shape1 = {
        ii(HEIGHT, 2),
        ii(HEIGHT, 3),
        ii(HEIGHT, 4),
        ii(HEIGHT, 5)};
    vector<ii> shape2 = {
        ii(HEIGHT, 3),
        ii(HEIGHT + 1, 2),
        ii(HEIGHT + 1, 3),
        ii(HEIGHT + 1, 4),
        ii(HEIGHT + 2, 3)};
    vector<ii> shape4 = {
        ii(HEIGHT, 2),
        ii(HEIGHT + 1, 2),
        ii(HEIGHT + 2, 2),
        ii(HEIGHT + 3, 2)};
    vector<ii> shape3 = {
        ii(HEIGHT, 2),
        ii(HEIGHT, 3),
        ii(HEIGHT, 4),
        ii(HEIGHT + 1, 4),
        ii(HEIGHT + 2, 4)};
    vector<ii> shape5 = {
        ii(HEIGHT, 2),
        ii(HEIGHT, 3),
        ii(HEIGHT + 1, 2),
        ii(HEIGHT + 1, 3),
    };

    if (i == 1)
        return shape1;
    if (i == 2)
        return shape2;
    if (i == 3)
        return shape3;
    if (i == 4)
        return shape4;
    return shape5;
}

bool checkPointMove(ii p, char c)
{
    if (c == '>')
    {
        if (p.second + 1 >= WIDTH || rocks.count(ii(p.first, p.second + 1)))
            return false;
    }
    if (c == '<')
    {
        if (p.second - 1 < 0 || rocks.count(ii(p.first, p.second - 1)))
            return false;
    }
    return true;
}

void moveCheck(vector<ii> &shape, char c)
{
    bool good = true;
    for (ii p : shape)
    {
        good = good && checkPointMove(p, c);
    }

    if (good)
    {
        for (ii &p : shape)
        {
            if (c == '>')
                p.second++;
            else
                p.second--;
        }
    }
}

bool check(ii p)
{
    if (p.first - 1 < 0 || rocks.count(ii(p.first - 1, p.second)))
        return false;

    return true;
}

bool checkShape(vector<ii> &shape)
{
    bool good = true;
    for (ii p : shape)
    {
        good = good && check(p);
    }

    for (ii &p : shape)
    {
        if (good)
            p.first--;
        else
        {
            HEIGHT = max(HEIGHT, p.first + 4);
            rocks.insert(p);
        }
    }

    return good;
}

void print(vector<ii> shape)
{
    printf("\033[2J\033[1;1H"); // reset terminal
    puts("");
    for (int i = HEIGHT + 5; i >= 0; i--)
    {
        for (int j = 0; j < WIDTH; j++)
        {
            if (rocks.count(ii(i, j)))
                cout << "#";
            else
            {
                bool found = false;
                for (ii p : shape)
                {
                    if (i == p.first && j == p.second)
                    {
                        cout << "@";
                        found = true;
                        break;
                    }
                }
                if (!found)
                    cout << ".";
            }
        }
        cout << endl;
    }
    puts("");
    fflush(stdout);
}

int main(void)
{
    string commands;
    cin >> commands;
    int it = 0;
    long long R = 2022 - 1;
    int N = commands.size();

    int S = 1;
    for (long long rep = 0; rep < R; rep++)
    {
        if (S == 6)
            S = 1;
        vector<ii> shape = getShape(S);
        while (true)
        {
            // print(shape);
            moveCheck(shape, commands[(it) % N]); // push
            it++;
            bool good = checkShape(shape); // fall
            if (!good)
            {
                // print(shape);
                break;
            }
        }
        S++;
    }

    cout << HEIGHT << endl;
    return 0;
}

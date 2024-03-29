#include <stdio.h>
#include <iostream>
#include <map>
#include <string.h>
#include <set>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;
#include <unistd.h>
#include <cstdlib>

typedef pair<int, int> ii;
set<ii> rocks;
set<ii> sands;
int SOURCE_Y = 500;
int SOURCE_X = 0;
string input;
int LIMIT = -1;
vector<string> tokenize(string s, string del = " ")
{
    int start = 0;
    int end = s.find(del);
    vector<string> strs;
    while (end != -1)
    {
        strs.push_back(s.substr(start, end - start));
        start = end + del.size();
        end = s.find(del, start);
    }
    strs.push_back(s.substr(start, end - start));

    return strs;
}

void print(ii sand, int iter)
{
    printf("\033[2J\033[1;1H"); // reset terminal
    puts("");
    for (int c = 0; c <= LIMIT + 2; c++)
    {
        for (int r = 490; r <= 550; ++r)
        {
            ii p(c, r);
            if (c == SOURCE_X && r == SOURCE_Y)
                printf("+ ");
            else if (rocks.count(p))
                printf("# ");
            else if (sands.count(p) || p == sand)
                printf("o ");
            else
                printf(". ");
        }
        printf("\n");
    }
    puts("");
    fflush(stdout);

    usleep(100000); // sample test
}
void drawRocks()
{
    while (getline(cin, input))
    {
        vector<string> tokens = tokenize(input);

        ii start(stoi(tokens[1]), stoi(tokens[0]));

        for (int i = 2; i < tokens.size(); i += 2)
        {
            int y = stoi(tokens[i]);
            int x = stoi(tokens[i + 1]);
            LIMIT = max(LIMIT, x);
            ii next(x, y);
            if (start.first == next.first)
            {
                int dy = start.second < next.second ? 1 : -1;
                for (int j = start.second; j != next.second + dy; j += dy)
                {
                    rocks.insert(ii(start.first, j));
                }
            }
            else
            {
                int dx = start.first < next.first ? 1 : -1;
                for (int i = start.first; i != next.first + dx; i += dx)
                {
                    rocks.insert(ii(i, start.second));
                }
            }
            start = next;
        }
    }
}

bool moveSand(ii &sand)
{
    int x = sand.first, y = sand.second;
    ii p1(x + 1, y), p2(x + 1, y - 1), p3(x + 1, y + 1);
    for (ii p : {p1, p2, p3})
    {
        if (!rocks.count(p) && !sands.count(p))
        {
            sand = p;
            return true;
        }
    }
    sands.insert(sand);
    return false;
}

int main(void)
{ // Providing a seed value
    srand((unsigned)time(NULL));
    drawRocks();
    for (int i = -1000; i <= 1000; ++i)
    {
        rocks.insert(ii(LIMIT + 2, i));
    }
    int rep = 1;

    while (true)
    {
        ii sand(SOURCE_X, SOURCE_Y);
        printf("\033[1;%dm", 31 + (rand() % 7)); // start color

        while (moveSand(sand))
        {
            print(sand, rep);
            if (sand.first == SOURCE_X && sand.second == SOURCE_Y)
                break;
        }
        printf("\033[0m"); // end color

        if (sand.first == SOURCE_X && sand.second == SOURCE_Y)
            break;
        rep++;
    }
    cout << rep << endl;

    return 0;
}
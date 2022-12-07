#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <vector>
#include <unordered_set>
#include <unordered_map>
using namespace std;

string input;
unordered_map<string, long long> dirSizes;
vector<string> path_so_far;

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

void read()
{
    while (getline(cin, input))
    {
        vector<string> strs = tokenize(input);
        if (strs[0] == "$")
        {
            if (strs[1] == "cd")
            {
                if (strs[2] == "..")
                    path_so_far.pop_back();
                else
                    path_so_far.push_back(strs[2]);
            }
        }

        else if (strs[0] != "dir")
        {
            int size = stoi(strs[0]);
            for (int i = 0; i < path_so_far.size(); ++i)
            {
                // must be absolute path!
                // dirs A and B can have same subdir C
                // we must be able to differentiate them
                string p = "";
                for (int j = 0; j <= i; ++j)
                    p += path_so_far[j];
                dirSizes[p] += size;
            }
        }
    }
}

int main(void)
{

    read();
    long long part1 = 0, part2 = INT_MAX;
    int toFree = dirSizes["/"] - 40000000;
    for (auto it : dirSizes)
    {
        if (it.second <= 100000)
            part1 += it.second;
        if (it.second >= toFree)
            part2 = min(part2, it.second);
    }
    cout << "PART1: " << part1 << endl;
    cout << "PART2: " << part2 << endl;
    return 0;
}

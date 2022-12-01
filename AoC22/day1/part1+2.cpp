#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <queue>
using namespace std;
priority_queue<int, vector<int>, greater<int>> pq;

void addToQ(int x)
{
    pq.push(x);
    if (pq.size() > 3)
        pq.pop();
}

int solve()
{
    string str;
    int curr = 0;
    while (getline(cin, str))
    {
        if (str == "")
        {
            addToQ(curr);
            curr = 0;
        }
        else
        {
            curr += stoi(str);
            if (cin.eof())
                addToQ(curr);
        }
    }
    int ans = 0;
    while (!pq.empty())
    {
        ans += pq.top();
        pq.pop();
        if (pq.size() == 1)
            cout << "PART1 : " << pq.top() << endl;
    }
    cout << "PART2: " << ans << endl;
    return ans;
}

int main(void)
{
    solve();
    return 0;
}

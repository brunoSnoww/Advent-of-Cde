#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;
typedef uint64_t ll;
typedef pair<char, ll> ops;
int M = 1;
string input;

ll mod(ll x) { return ((x % M) + M) % M; }

struct Monkey
{
    ll id;
    ll d; // divisor
    ll toThrowIfTrue, toThrowIfFalse;
    vector<ll> items;
    ops op;
    ll operation(ll x)
    {
        if (op.first == '^')
            return mod(x * x);
        if (op.first == '*')
        {
            return mod(x * op.second);
        }
        return mod(x + op.second);
    }
    ll throwToMonkey(ll x)
    {
        ll t = operation(x) / 3;
        if (t % d == 0)
            return toThrowIfTrue;
        return toThrowIfFalse;
    }

    void addItem(ll x)
    {
        items.push_back(x);
    }
    void removeItem(ll x)
    {
        items.erase(remove(items.begin(), items.end(), x), items.end());
    }

    void read()
    {
        cin >> id;
        cin >> input;
        while (input != "old")
        {
            items.push_back(stoi(input));
            cin >> input;
        }
        cin >> input;
        if (input == "*")
        {
            cin >> input;
            if (input == "old")
            {
                op = ops('^', -1);
            }
            else
                op = ops('*', stoi(input));
        }
        else
        {
            cin >> input;
            op = ops('+', stoi(input));
        }
        cin >> d;
        cin >> toThrowIfTrue;
        cin >> toThrowIfFalse;
    }
};

int main(void)
{

    vector<Monkey> monkeys;
    ll ROUNDS = 20;
    while (!cin.eof())
    {
        Monkey m;
        m.read();
        monkeys.push_back(m);
        M *= m.d;
    }
    vector<ll> times(monkeys.size(), 0);

    for (ll r = 0; r < ROUNDS; ++r)
    {
        for (ll i = 0; i < monkeys.size(); ++i)
        {
            times[i] += monkeys[i].items.size();
            for (ll x : monkeys[i].items)
            {
                ll mk = monkeys[i].throwToMonkey(x);
                monkeys[mk].addItem(monkeys[i].operation(x) / 3);
            }
            monkeys[i].items.clear();
        }
    }
    sort(begin(times), end(times));
    cout << "PART1: " << times[times.size() - 1] * times[times.size() - 2] << endl;
}
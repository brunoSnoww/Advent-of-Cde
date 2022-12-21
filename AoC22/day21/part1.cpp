#include <stdio.h>
#include <iostream>
#include <string.h>
#include <unordered_map>
#include <vector>
#include <cstdlib>
#include <unistd.h>

using namespace std;
typedef __int64_t ll;
int INF = -1;
string currentId = "";

struct Node
{
    double value = INF;
    char op;
    string n1 = "", n2 = ""; // ids of operators
    string id = "";
    void read()
    {
        string input;
        getline(cin, input);
        int i = 0;
        while (input[i] != ':')
            id += input[i++];
        currentId = id;
        i += 2;
        if (isdigit(input[i]))
        {
            ll b = 0;
            while (i < input.size())
                b = 10 * b + (input[i++] - '0');
            value = b;
            return;
        }
        else
        {
            while (input[i] != ' ')
                n1 += input[i++];
            i++;
            op = input[i];
            i += 2;
            while (i < input.size())
                n2 += input[i++];
        }
    }
};
unordered_map<string, Node> mp;
vector<Node> v;
double getVal(string nodeId)
{
    if (nodeId == "humn")
        cout << "FUCK" << endl;
    if (mp[nodeId].value != INF)
    {
        // cout << mp[nodeId].value << endl;
        return mp[nodeId].value;
    }
    double v1 = getVal(mp[nodeId].n1),
           v2 = getVal(mp[nodeId].n2);
    double val;
    char op = mp[nodeId].op;
    if (op == '+')
        val = v1 + v2;
    if (op == '-')
        val = v1 - v2;
    if (op == '*')
        val = v1 * v2;
    if (op == '/')
        val = v1 / v2;
    mp[nodeId].value = val;
    return val;
}
int main(void)
{
    int N = 1605;
    v.resize(N);
    int i = 0;
    while (i < N)
    {
        v[i].read();
        mp[currentId] = v[i];
        i++;
    }
    cout << (ll)getVal("root") << endl;

    return 0;
}
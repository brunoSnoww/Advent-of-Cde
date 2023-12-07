#include <stdio.h>
#include <iostream>
#include <map>
#include <string>
#include <set>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;
typedef vector<int> vi;

const int INTEGER = 0;
const int LIST = 1;

struct Packet
{
    int type;
    int integer = 0;
    vector<Packet> subPacket;
    void read(string &input, int &i)
    {
        int j = i;
        if (isdigit(input[j]))
        {
            type = INTEGER;
            while (j < input.size() && input[j] != ',' && input[j] != ']')
            {
                integer = 10 * integer + (input[j] - '0');
                j++;
            }
            j = i + 1;
        }
        else
        {
            NestedInteger p;
        }
    }
    bool isInteger()
    {
        return type == INTEGER;
    }
}

struct NestedIterator
{
    vector<int> v;
    int it = -1;
    void add(const vector<NestedInteger> &L)
    {
        for (int i = 0; i < L.size(); i++)
        {
            if (L[i].isInteger())
                v.push_back(L[i].getInteger());
            else
            {
                add(L[i].getList());
            }
        }
    }
    NestedIterator(vector<NestedInteger> &nestedList)
    {
        add(nestedList);
    }

    int next()
    {
        ++it;
        return v[it];
    }

    bool hasNext()
    {
        return it + 1 < v.size();
    }
};

struct Packet
{
    vi list;
    int numb = 0;
    int type;
    void read(string &input, int &idx)
    {
        int j = idx;
        if (input[j] == '[')
        {
            int num = 0;
            while (input[++j] != ']')
            {
                if (input[j] == ',')
                {
                    list.push_back(num);
                    num = 0;
                }
                else
                {
                    num = 10 * num + (input[j] - '0');
                }
            }
        }
        else
        {
            while (j < input.size() && input[j] != ',' && input[j] != ']')
            {
                numb = 10 * numb + (input[j] - '0');
            }
        }
        idx = j;
    }
};

vector<vi> read(string &input, int index)
{
}

int main(void)
{
}
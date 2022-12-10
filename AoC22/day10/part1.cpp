#include <stdio.h>
#include <iostream>
#include <string>

using namespace std;

int cycles = 0;
int R = 1;
int S = 20;
int STRENGTH = 0;
int SPRITE_POS = 1;
int CRT_POS = 0;

void mod(int &x, int M) { x = ((x % M) + M) % M; }

void checkDraw()
{
    cycles++;
    // PART 1
    if (cycles == S)
    {
        STRENGTH += R * S;
        S += 40;
    }
    //
    // PART 2
    if (SPRITE_POS - 1 <= CRT_POS && CRT_POS <= SPRITE_POS + 1)
    {
        cout << "# ";
    }
    else
    {
        cout << ". ";
    }
    if (cycles % 40 == 0)
    {
        cout << endl;
    }
    mod(++CRT_POS, 40);
    //
}

int main(void)
{
    string input;
    while (!cin.eof())
    {
        cin >> input;
        if (input == "noop")
        {
            checkDraw();
        }
        else
        {
            cin >> input;
            checkDraw();
            checkDraw();
            R += stoi(input);
            SPRITE_POS = R;
        }
    }
    cout
        << endl
        << "STRENGTH: " << STRENGTH << endl;
}

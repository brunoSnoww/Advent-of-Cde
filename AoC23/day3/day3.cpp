#include <cctype>
#include <fstream>
#include <iostream>
#include <map>
#include <vector>
using namespace std;

typedef pair<int, int> ii;
vector<int> dx = {0, 0, 1, -1, 1, -1, 1, -1};
vector<int> dy = {-1, 1, 0, 0, 1, -1, -1, 1};
int N, M;
bool isValid(int i, int j) { return i >= 0 && i < N && j >= 0 && j < M; }
using namespace std;

int main() {
  vector<vector<char>> grid;

  ifstream inputFile("./input3.txt");

  if (!inputFile.is_open()) {
    cerr << "Error opening the file." << endl;
    return 1;
  }

  vector<string> lines;
  string line;
  while (getline(inputFile, line)) {
    lines.push_back(line);
  }

  inputFile.close();

  N = lines.size();
  M = lines[0].size();
  grid.resize(N, vector<char>(M));

  for (int i = 0; i < N; ++i) {
    for (int j = 0; j < M; ++j) {
      grid[i][j] = lines[i][j];
    }
  }
  int sum = 0;
  int jj;
  bool isGood = false;
  bool isGear = false;
  int gearRatio = 0;
  bool sawGear;
  ii lastGear;
  map<ii, int> gears;
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < M; ++j) {
      if (isdigit(grid[i][j])) {
        int tmp = 0;
        isGood = false;
        isGear = false;
        jj = j;
        sawGear = false;
        while (true) {
          if (jj == M || !isdigit(grid[i][jj])) {
            break;
          }
          for (int k = 0; k < 8; ++k) {
            int di = i + dx[k];
            int dj = jj + dy[k];
            if (isValid(di, dj)) {
              if (!isdigit(grid[di][dj]) && grid[di][dj] != '.') {
                if (grid[di][dj] == '*') {
                  sawGear = true;
                  lastGear = ii(di, dj);
                  if (gears.count(ii(di, dj)))
                    isGear = true;
                }
                isGood = true;
              }
            }
          }
          tmp = 10 * tmp + (grid[i][jj] - '0');
          jj++;
        }
        if (isGood)
          sum += tmp;
        if (isGear)
          gearRatio += tmp * gears[lastGear];
        else if (sawGear) {
          gears[lastGear] = tmp;
        }
        j = jj;
      }
    }
  }
  cout << sum << endl;
  cout << gearRatio << endl;

  return 0;
}

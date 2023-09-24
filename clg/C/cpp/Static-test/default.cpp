#include <iostream>
using namespace std;

int main() {
  static int tea2, team = 10;

  cout << "1 " << team << endl;

  team++;
  tea2++;

  cout << "1 " << team << " 2 " << tea2 << endl;
}

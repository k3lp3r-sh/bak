#include <iostream>
using namespace std;

class A {
public:
  int x;

protected:
  int y;

private:
  int z;
};

class B : public A {
public:
  void display() {
    y = 20;
    // z=25;

    cout << y << endl;
    // cout << z << endl;
  }
};

int main() {
  class B;
  cout << B.display() << endl;
}

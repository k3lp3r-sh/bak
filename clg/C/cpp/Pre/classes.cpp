#include <iostream>
using namespace std;

class Markinfo {
public:
  char name;
  int mark1, mark2, mark3, mark4, mark5;
  int rollno;
  int calculateGrade() {
    if (mark1 + mark5 + mark2 + mark4 + mark3 > 250) {
      return 1;
    } else {
      return 0;
    }
  };
};

int main() {
  Markinfo obj;

  obj.mark1 = 100;
  obj.mark2 = 10 obj.mark3 = 0;
  obj.mark4 = 0;
  obj.mark5 = 0;
  obj.name = 'a';

  int c = obj.calculateGrade();

  cout << c << endl;
}

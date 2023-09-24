#include <iostream>
#include <string>
using namespace std;

class book {

  string title;
  string author;
  string isbn;

public:
  void displayInfo(book arr[], int i) {

    cout << "{" << endl;
    for (int j = 0; j < i; j++) {
      cout << "[Title: " << arr[j].title;
      cout << " Author:" << arr[j].author;
      cout << " ISBN:" << arr[j].isbn << "]," << endl;
    }
    cout << "}" << endl;
  }

  book findBookInfo(book arr[], int i) {
    string title;
    int flag = 0;
    cin >> title;

    for (int j = 0; j < i; j++) {
      if (arr[j].title == title) {
        return arr[j];
        flag = 1;
      }
    }
    if (flag == 0) {
      cout << "Title does not exist";
    }
  }

  void datadump(book arr) {
    cout << arr.title << endl;
    cout << arr.author << endl;
    cout << arr.isbn << endl;
  }

  void addaBook(book arr[], int i) {
    cout << "Enter Title" << endl;
    cin >> arr[i].title;

    cout << "Enter Author" << endl;
    cin >> arr[i].author;

    cout << "Enter ISBN" << endl;
    cin >> arr[i].isbn;
  }
};

int main() {

  book books[100], result;

  for (int i = 0; i < 5; i++) {
    books[0].addaBook(books, i);
  }

  result = result.findBookInfo(books, 5);

  result.datadump(result);

  // for (int i = 0; i < 5; i++) {
  //   books[0].displayInfo(books, 5);
  // }
  books[0].addaBook(books, 5);
}

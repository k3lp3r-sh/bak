import java.io.*;
import java.util.*;

public class fibonacci {

  public static void main(String[] args) {
    int a, temp;
    Scanner input = new Scanner(System.in);

    a = input.nextInt();

    int d1 = 0;
    int d2 = 1;

    for (int i = 0; i < a; i++) {
      System.out.println(d1);
      temp = d1;
      d1 = d1 + d2;
      d2 = temp;

    }
  }
}

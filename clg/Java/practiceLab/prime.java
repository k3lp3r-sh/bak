import java.io.*;
import java.util.*;

public class prime {

  public static void main(String[] args) {
    int a, temp;
    Scanner input = new Scanner(System.in);
    a = input.nextInt();
    int i;
    int flag = 0;
    for (i = 2; i < a; i++) {
      if (a % i == 0) {
        flag = 1;
        break;
      }
    }
    if (flag == 1) {
      System.out.println("Not prime");
    } else {
      System.out.println("prime");
    }

  }
}

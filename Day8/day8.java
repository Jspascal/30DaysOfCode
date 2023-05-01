import java.io.*;
import java.util.*;

public class Solution {

  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
    int t = in.nextInt();
    in.nextLine();
    HashMap<String, String> contacts = new HashMap<String, String>();

    for (int i = 0; i < t; i++) {
      String[] contact = in.nextLine().split(" ", 2);
      contacts.put(contact[0], contact[1]);
    }

    for (int i = 0; i < t; i++) {
      String contact_name = in.nextLine();
      if (contacts.containsKey(contact_name))
        System.out.println(contact_name + "=" + contacts.get(contact_name));
      else
        System.out.println("Not found");
    }
  }
}

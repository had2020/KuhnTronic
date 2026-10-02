import java.util.Scanner;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.PrintWriter;

public class Main {
    public static void write_to(String text) throws IOException {
        FileOutputStream bufOut = null;
        bufOut = new FileOutputStream("../temp.swp");
        PrintWriter outFS = new PrintWriter(bufOut);
        outFS.println(text);
        outFS.close();
    }

    public static String read_from() throws FileNotFoundException {
        FileInputStream bufIn = null;
        bufIn = new FileInputStream("../temp.swp");
        Scanner inFS = new Scanner(bufIn);
        String r = inFS.nextLine();
        inFS.close();
        return r;
    }

    public static void main(String[] args) {
        Scanner scnr = new Scanner(System.in);

        System.out.println("Input the move Call, Fold, Check or Bet");
        String move = scnr.next();

        if (move.contentEquals("Call")) { // PLEASE DO NOT USE REGULAR EQUALITY CHECKS ON STRINGS. MUST USE CONTENTEQUALS()
            System.out.println("You called");
        } else if (move.contentEquals("Fold")) {
            System.out.println("You Folded");
        } else if (move.contentEquals("Check")) {
            System.out.println("You Checked");
        } else if (move.contentEquals("Bet")) {
            System.out.println("You Bet");
        }

        }

        scnr.close();
    }
}

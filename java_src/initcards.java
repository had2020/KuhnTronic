import java.util.Random;
import java.util.Scanner;
// java Scanner learn input and output

public class initcards {
    public static void main(String[] args) {

        // gen   3 King 2 Queen 1 Joker
        // AI:
        // User:
        // Card middle: 
    Random hands = new Random();
    Scanner in = new Scanner(System.in);
   
    int hands_player1 = hands.nextInt(1,4);

    int hands_player2 = hands.nextInt(1,4);
    while (hands_player2 == hands_player1){
        hands_player2 = hands.nextInt(1,4);
    }

    int card_middle = hands.nextInt(1,4);
     while (card_middle == hands_player1 || card_middle == hands_player2){
        card_middle = hands.nextInt(1,4);
     }
         //player 1 always starts first
         //cost 1 chip per player per tern, 2 in pot. 

    int chips_table = 2;
    int chips_player1 = 1;
    int chips_player2 = 1;

    System.out.println("Your card is: " + hands_player1);
    System.out.println("\u001B[0m" +"Would you like to " + "\u001B[32m" + "Check (1)"+ "\u001B[0m" + " or " + "\u001B[31m" + "Bet(2)" + "\u001B[0m" + "?:");
    int choice = in.nextInt();

    while (choice != 1 && choice != 2){
        System.out.println("Type Check to check, and Bet to bet.");
        choice = in.nextInt();
    }
    if (choice == 2){

    }


    
    








}






}

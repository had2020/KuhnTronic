import java.util.Random;
import java.util.Scanner;
// java Scanner learn input and output

public class initcards {
    // Note I had to move the states by 1 foward as 0 is now no_action.
enum GameState { // -> next best move
  KEmpty , // -> KCheck
  KCheck , // -> KCheckBet
  KBet , // -> KBetCall
  KCheckBet , // -> KCheckBetCall
  // End game states
  KCheckCheck ,
  KCheckBetFold ,
  KCheckBetCall ,
  KBetFold ,
  KBetCall ,

  QEmpty , // -> QCheck
  QCheck , // -> QCheckCheck
  QBet , // -> QBetFold
  QCheckBet , // -> QCheckBetFold
  // End game states
  QCheckCheck ,
  QCheckBetFold ,
  QCheckBetCall ,
  QBetFold ,
  QBetCall ,

  JEmpty , // -> JBet
  JCheck , // -> JCheckBet
  JBet , // -> JBetFold
  JCheckBet , // -> JCheckBetFold
  // End game states
  JCheckCheck ,
  JCheckBetFold ,
  JCheckBetCall ,
  JBetFold ,
  JBetCall ,

  NoAction ,
};




    public static void main(String[] args) {

        // gen   3 King 2 Queen 1 Joker
        // AI:
        // User:
        // Card middle:
    Random hands = new Random();
    Scanner in = new Scanner(System.in);

    // this uint8_t stores the next best action index, for the AI.
    // The best move is index + 1 as I shifted them by 1.
    GameState[] node_tree = new GameState[] {
        GameState.KCheck, GameState.KCheckBet, GameState.KBetCall, GameState.KCheckBetCall,
        GameState.NoAction, GameState.NoAction, GameState.NoAction, GameState.NoAction, GameState.NoAction,
        GameState.QCheck, GameState.QCheckCheck, GameState.QBetFold, GameState.QCheckBetFold,
        GameState.NoAction, GameState.NoAction, GameState.NoAction, GameState.NoAction, GameState.NoAction,
        GameState.JBet, GameState.JCheckBet, GameState.JBetFold, GameState.JCheckBetFold,
        GameState.NoAction, GameState.NoAction, GameState.NoAction, GameState.NoAction, GameState.NoAction
    };

    // Example within the while with the players:
    // P1 (AI) goes first, and they have King, but P2 Joker.
    GameState gameState = node_tree[GameState.KEmpty.ordinal()];
    
        // Ante
    int chips_table = 2;
    int chips_player1 = 1;
    int chips_player2 = 1;


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
         //cost 1 chip per player per tern, 1 more to bet, 2 in pot.


        //while loop below: if player1(ai) checked, the player can bet. When the AI bet, player one needs to check or fold

    System.out.println("Your card is: " + hands_player2);
    System.out.println("\u001B[0m" +"Would you like to " + "\u001B[32m" + "Check (1)"+ "\u001B[0m" + " or " + "\u001B[31m" + "Bet(2)" + "\u001B[0m" + "?:");
    int choice = in.nextInt();

     //if player 1 and 2 checked, showdown. if player 1 bet, +1 pot, 
     

     while (hands_player2 = 3);


    while (choice != 1 && choice != 2){
        System.out.println("Type 1 to check, and 2 to bet.");
        choice = in.nextInt();
    }
    if (choice == 2){
    chips_player2 = chips_player2 - 1;
    chips_table = chips_table + 1;
        //player 2 is asked to check, fold or raise
    }


    }












}

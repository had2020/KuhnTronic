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


    int hands_p1 = hands.nextInt(1,4);

    int hands_p2 = hands.nextInt(1,4);
    while (hands_p2 == hands_p1){
        hands_p2 = hands.nextInt(1,4);
    }

    int card_middle = hands.nextInt(1,4);
     while (card_middle == hands_p1 || card_middle == hands_p2){
        card_middle = hands.nextInt(1,4);
     }
         //player 1 always starts first
         //cost 1 chip per player per tern, 1 more to bet, 2 in pot.

    int chips_table = 2;
    int chips_p1 = 1;
    int chips_p2 = 1;
    GameState end = GameState.NoAction;

    System.out.println("Your card is: " + hands_p1);
    System.out.println("\u001B[0m" +"Would you like to " + "\u001B[32m" + "Check (1)"+ "\u001B[0m" + " or " + "\u001B[31m" + "Bet(2)" + "\u001B[0m" + "?:");
    int choice = in.nextInt();

while (choice != 1 && choice != 2){
    System.out.println("Type 1 to check, and 2 to bet.");
    choice = in.nextInt();
}
if (choice == 2){
    chips_p1 = chips_p1 + 1;
    chips_table = chips_table + 1;
    //player 2 is asked to call or fold
    int p2 = hands.nextInt(2);
    if (p2 == 0) {
        System.out.println("Player 2 calls.");
        chips_p2 = chips_p2 + 1;
        chips_table = chips_table + 1;

        if (hands_p1 == 3){
            end = GameState.KBetCall;
        } else if (hands_p1 == 2){
            end = GameState.QBetCall;
        } else {
            end = GameState.JBetCall;
        }
    } else {
        System.out.println("Player 2 folds.");
        if (hands_p1 == 3){
            end = GameState.KBetFold;
        } else if (hands_p1 == 2){
            end = GameState.QBetFold;
        } else {
            end = GameState.JBetFold;
        }
    }
} else {
    System.out.println("You check.");

    int p2 = hands.nextInt(2);
    if (p2 == 0) {
        System.out.println("Player 2 checks.");

        if (hands_p1 == 3){
            end = GameState.KCheckCheck;
        } else if (hands_p1 == 2){
            end = GameState.QCheckCheck;
        } else {
            end = GameState.JCheckCheck;
        }
    } else {
        System.out.println("Player 2 bets.");
        chips_p2 = chips_p2 + 1;
        chips_table = chips_table + 1;
    }
}
System.out.println("Pot: " + chips_table);
System.out.println(end);


    }
}











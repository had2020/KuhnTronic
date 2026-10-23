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
    // P1 (CPU) goes first, and they have King, but P2 Joker.
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
    int winner = 0;
    int start_chips = 10;   // each player starts the hand with 10 chips

System.out.println("\u001B[31m" + "Your card is: " + hands_p2 + "\u001B[0m");
System.out.println("Chips - You: " + (start_chips - chips_p2) + ", CPU: " + (start_chips - chips_p1) + ", Pot: " + chips_table);

//player 1 (CPU) goes first: it bets with the Joker (1) and checks otherwise
int choice = 1;
if (hands_p1 == 1){
    choice = 2;
}

if (choice == 2){
    System.out.println("Player 1 (CPU) bets.");
    chips_p1 = chips_p1 + 1;
    chips_table = chips_table + 1;
    System.out.println("Chips - You: " + (start_chips - chips_p2) + ", CPU: " + (start_chips - chips_p1) + ", Pot: " + chips_table);

    System.out.println("Type 1 to call, and 2 to fold.");
    int choice2 = in.nextInt();
    while (choice2 != 1 && choice2 != 2){
        System.out.println("Type 1 to call, and 2 to fold.");
        choice2 = in.nextInt();
    }
    if (choice2 == 1){
        System.out.println("You call.");
        chips_p2 = chips_p2 + 1;
        chips_table = chips_table + 1;
        System.out.println("Chips - You: " + (start_chips - chips_p2) + ", CPU: " + (start_chips - chips_p1) + ", Pot: " + chips_table);

        if (hands_p1 == 3){
            end = GameState.KBetCall;
        } else if (hands_p1 == 2){
            end = GameState.QBetCall;
        } else {
            end = GameState.JBetCall;
        }
    } else {
        System.out.println("You fold.");
        winner = 1;

        if (hands_p1 == 3){
            end = GameState.KBetFold;
        } else if (hands_p1 == 2){
            end = GameState.QBetFold;
        } else {
            end = GameState.JBetFold;
        }
    }
} else {
    System.out.println("Player 1 (CPU) checks.");

    System.out.println("Type 1 to check, and 2 to bet.");
    int choice2 = in.nextInt();
    while (choice2 != 1 && choice2 != 2){
        System.out.println("Type 1 to check, and 2 to bet.");
        choice2 = in.nextInt();
    }
    if (choice2 == 1){
        System.out.println("You check.");
        if (hands_p1 == 3){
            end = GameState.KCheckCheck;
        } else if (hands_p1 == 2){
            end = GameState.QCheckCheck;
        } else {
            end = GameState.JCheckCheck;
        }
    } else {
        System.out.println("You bet.");
        chips_p2 = chips_p2 + 1;
        chips_table = chips_table + 1;
        System.out.println("Chips - You: " + (start_chips - chips_p2) + ", CPU: " + (start_chips - chips_p1) + ", Pot: " + chips_table);

        if (hands_p1 == 3){
            System.out.println("Player 1 (CPU) calls.");
            chips_p1 = chips_p1 + 1;
            chips_table = chips_table + 1;
            System.out.println("Chips - You: " + (start_chips - chips_p2) + ", CPU: " + (start_chips - chips_p1) + ", Pot: " + chips_table);
            end = GameState.KCheckBetCall;
        } else {
            System.out.println("Player 1 (CPU) folds.");
            winner = 2;
            if (hands_p1 == 2){
                end = GameState.QCheckBetFold;
            } else {
                end = GameState.JCheckBetFold;
            }
        }
    }
}
if (winner == 0){
    System.out.println("Player 1 (CPU) has: " + hands_p1);
    if (hands_p1 > hands_p2){
        winner = 1;
    } else {
        winner = 2;
    }
}
if (winner == 2){
    System.out.println("You win the pot of " + chips_table + " chips (profit: " + chips_p1 + ").");
    System.out.println("Final chips - You: " + (start_chips - chips_p2 + chips_table) + ", CPU: " + (start_chips - chips_p1));
} else {
    System.out.println("Player 1 (CPU) wins the pot of " + chips_table + " chips (you lose " + chips_p2 + ").");
    System.out.println("Final chips - You: " + (start_chips - chips_p2) + ", CPU: " + (start_chips - chips_p1 + chips_table));
}


System.out.println("Pot: " + chips_table);
System.out.println(end);


    
    }
}








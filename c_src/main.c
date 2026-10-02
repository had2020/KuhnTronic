#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "time.h"
#include <unistd.h>

/* A node can be best thought of as this:
    Possible Card states: K=2, Q=1, J=0;

    u8 History states:
    Empty = 0,        // P1 turn 1: ""
    Check = 1,        // P2 turn 1 after P1 check: "C"
    Bet = 2,          // P2 turn 1 after P1 bet: "B"
    CheckBet = 3,     // P1 turn 2 after P1 check & P2 bet: "CB"
    // End Game states
    CheckCheck = 4,   // "CC"  -> Showdown (Pot: 2) | Winner: High card (P1 if C1 > C2, else P2)
    CheckBetFold = 5, // "CBF" -> P2 wins (Pot: 2)  | Winner: P2 (+1 profit, P1 folds)
    CheckBetCall = 6, // "CBC" -> Showdown (Pot: 4) | Winner: High card (P1 if C1 > C2, else P2)
    BetFold = 7,      // "BF"  -> P1 wins (Pot: 2)  | Winner: P1 (+1 profit, P2 folds)
    BetCall = 8,      // "BC"  -> Showdown (Pot: 4) | Winner: High card (P1 if C1 > C2, else P2)
*/

// TODO redesign for Leduc Hold'em Poker
enum GameState { // -> next best move
  KEmpty = 0, // -> KCheck
  KCheck = 1, // -> KCheckBet
  KBet = 2, // -> KBetCall
  KCheckBet = 3, // -> KCheckBetCall
  // End game states
  KCheckCheck = 4,
  KCheckBetFold = 5,
  KCheckBetCall = 6,
  KBetFold = 7,
  KBetCall = 8,

  QEmpty = 9, // -> QCheck
  QCheck = 10, // -> QCheckCheck
  QBet = 11, // -> QBetFold
  QCheckBet = 12, // -> QCheckBetFold
  // End game states
  QCheckCheck = 13,
  QCheckBetFold = 14,
  QCheckBetCall = 15,
  QBetFold = 16,
  QBetCall = 17,

  JEmpty = 18, // -> JBet
  JCheck = 19, // -> JCheckBet
  JBet = 20, // -> JBetFold
  JCheckBet = 21, // -> JCheckBetFold
  // End game states
  JCheckCheck = 22,
  JCheckBetFold = 23,
  JCheckBetCall = 24,
  JBetFold = 25,
  JBetCall = 26,

  NoAction = 27,
};

int main(int argc, char *argv[]) {
    // this uint8_t stores the next best action index
    const enum GameState node_tree[27] = {
        KCheck, KCheckBet, KBetCall, KCheckBetCall,
        NoAction, NoAction, NoAction, NoAction, NoAction,
        QCheck, QCheckCheck, QBetFold, QCheckBetFold,
        NoAction, NoAction, NoAction, NoAction, NoAction,
        JBet, JCheckBet, JBetFold, JCheckBetFold,
        NoAction, NoAction, NoAction, NoAction, NoAction,
    };

    if (argc == 2) {
        printf("Raw debugging mode enabled. Input first State as a number: \n");

        while(1) {
            int input_s;
            scanf("%d", &input_s);
            printf("%d\n", node_tree[input_s]);
        }

    } else {
        FILE* fptr;

        fptr = fopen("../temp.swp", "r");

        if (fptr == NULL) {
            printf("Error openning file!");
        } else {
            fclose(fptr);
        }


        printf("The C interface is running, press control+c to kill it.\n");
        while (1) {
            usleep(1000000);
            break;
        }
    }

    return 0;
}

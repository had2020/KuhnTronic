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

// Note I had to move the states by 1 foward as 0 is now no_action.
enum GameState { // -> next best move
  KEmpty = 1, // -> KCheck
  KCheck = 2, // -> KCheckBet
  KBet = 3, // -> KBetCall
  KCheckBet = 4, // -> KCheckBetCall
  // End game states
  KCheckCheck = 5,
  KCheckBetFold = 6,
  KCheckBetCall = 7,
  KBetFold = 8,
  KBetCall = 9,

  QEmpty = 10, // -> QCheck
  QCheck = 11, // -> QCheckCheck
  QBet = 12, // -> QBetFold
  QCheckBet = 13, // -> QCheckBetFold
  // End game states
  QCheckCheck = 14,
  QCheckBetFold = 15,
  QCheckBetCall = 16,
  QBetFold = 17,
  QBetCall = 18,

  JEmpty = 19, // -> JBet
  JCheck = 20, // -> JCheckBet
  JBet = 21, // -> JBetFold
  JCheckBet = 22, // -> JCheckBetFold
  // End game states
  JCheckCheck = 23,
  JCheckBetFold = 24,
  JCheckBetCall = 25,
  JBetFold = 26,
  JBetCall = 27,

  NoAction = 0,
};

// this uint8_t stores the next best action index
// The best move is index + 1 as I shifted them by 1.
const enum GameState node_tree[28] = {
NoAction, KCheck, KCheckBet, KBetCall, KCheckBetCall,
NoAction, NoAction, NoAction, NoAction, NoAction,
QCheck, QCheckCheck, QBetFold, QCheckBetFold,
NoAction, NoAction, NoAction, NoAction, NoAction,
JBet, JCheckBet, JBetFold, JCheckBetFold,
NoAction, NoAction, NoAction, NoAction, NoAction,
};

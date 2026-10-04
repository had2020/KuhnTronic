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

// this uint8_t stores the next best action index
const enum GameState node_tree[27] = {
KCheck, KCheckBet, KBetCall, KCheckBetCall,
NoAction, NoAction, NoAction, NoAction, NoAction,
QCheck, QCheckCheck, QBetFold, QCheckBetFold,
NoAction, NoAction, NoAction, NoAction, NoAction,
JBet, JCheckBet, JBetFold, JCheckBetFold,
NoAction, NoAction, NoAction, NoAction, NoAction,
};

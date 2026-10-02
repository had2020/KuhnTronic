#include <stdint.h>

enum LearnOpcodes {
    SUB_DIFF    = 0x0, // R_d = R_s1 - R_s2 (Wrapping delta)
    MAC_STEP    = 0x1, // R_d += R_s1 * R_s2 (State accumulation)
    CMOV_DELTA  = 0x2, // R_d = (R_s2 != 0) ? (R_d + R_s1) : R_d
    XOR_MUT     = 0x3, // R_d ^= (R_s1 - R_s2) (Differential mutation)
    ROTL_ACC    = 0x4, // R_d += (R_s1 <<< (R_s2 & 63))
    CLAMP_STATE = 0x5, // R_d = (R_s1 < R_s2) ? R_s1 : R_s2 (Unsigned min)
    NAND_GATE   = 0x6, // R_d = !(R_s1 & R_s2)
    DECAY_L2    = 0x7, // R_d = R_s1 - (R_s1 >> (R_s2 & 7))
    MOMENTUM_UPD= 0x8, // R_d = (R_d >> 1) + (R_s1 - R_s2) (EMA weight decay/update)
    ABS_DELTA   = 0x9, // R_d = (R_s1 > R_s2) ? (R_s1 - R_s2) : (R_s2 - R_s1) (L1 loss term)
    SIGN_FLIP   = 0xA, // R_d = (R_s2 & 1) ? -R_s1 : R_s1 (Gradient sign reflection)
    SWAR_LEARN  = 0xB, // R_d = SIMD8_SUB(R_s1, R_s2) (8x8-bit parallel byte deltas)
    SCALED_ADD  = 0xC, // R_d += R_s1 >> (R_s2 & 63) (Learning-rate attenuated addition)
    MASK_MUTATE = 0xD, // R_d ^= R_s1 & ~R_s2 (Selective register bit-clearing)
    NORM_BOUND  = 0xE, // R_d = (R_s1 > R_s2) ? R_s2 : R_s1 (Upper bound ceiling clamp)
    RESET_ZERO  = 0xF  // R_d = (R_s1 == R_s2) ? 0 : R_d (Zero-out saturated or dead
};

enum RunOpcodes {
    MIX_ADD      = 0x0, // R_d = R_s1 + R_s2
    XOR_MASK     = 0x1, // R_d = R_s1 ^ R_s2
    ROTL_VAR     = 0x2, // R_d = (R_s1 << (R_s2 & 63)) | (R_s1 >> ((64 - (R_s2 & 63)) & 63))
    SWAP_NIB     = 0x3, // R_d = ((R_s1 & 0x0F0F...) << 4) | ((R_s1 & 0xF0F0...) >> 4)
    NAND_ACT     = 0x4, // R_d = !(R_s1 & R_s2)
    CMOV_NZ      = 0x5, // R_d = (R_s2 != 0) ? R_s1 : R_d
    POPCNT_MIX   = 0x6, // R_d = popcount(R_s1 ^ R_s2)
    EXTRACT_OUT  = 0x7, // R_d = R_s1 & 0xFF
    SHL_VAR      = 0x8, // R_d = R_s1 << (R_s2 & 63)
    SHR_VAR      = 0x9, // R_d = R_s1 >> (R_s2 & 63)
    BIT_REVERSE  = 0xA, // R_d = bit_reverse64(R_s1)
    ADD_SWAR8    = 0xB, // R_d = SIMD8_ADD(R_s1, R_s2) (8x8-bit parallel lane addition)
    MUL_HIGH     = 0xC, // R_d = ((unsigned __int128)R_s1 * R_s2) >> 64
    AND_NOT      = 0xD, // R_d = R_s1 & !R_s2
    XNOR_ACT     = 0xE, // R_d = !(R_s1 ^ R_s2)
    PACK_NIBBLES = 0xF  // R_d = (R_s1 & 0x0F0F...) | ((R_s2 & 0x0F0F...) << 4)
};

enum ChoiceOpcodes {
    CMP_THRESH   = 0x0, // R1 = (R_s1 >= R_s2) ? 1 : 0
    PROB_MASK    = 0x1, // R1 = ((R_s1 & 0xFF) < (R_s2 & 0xFF)) ? 1 : 0
    BIT_DECIDE   = 0x2, // R1 = (R_s1 >> (R_s2 & 63)) & 1
    SOFT_MAX2    = 0x3, // R1 = (popcount(R_s1) > popcount(R_s2)) ? 1 : 0
    ARGMAX_PAIR  = 0x4, // R1 = (R_s1 > R_s2) ? R_s1 : R_s2
    MASK_ACTION  = 0x5, // R1 = R_s1 & R_s2 & 1 (Legal action filter)
    REGRET_ACC   = 0x6, // R_d += (R_s1 > R_s2) ? (R_s1 - R_s2) : 0
    EMIT_CHOICE  = 0x7, // R1 = R_s1 & 1
    MAJORITY_VOTE= 0x8, // R1 = (popcount(R_s1 ^ R_s2) < 32) ? 1 : 0 (Bit hamming consensus)
    HYPER_PLANE  = 0x9, // R1 = ((R_s1 * R_s2) >> 63) & 1 (Sign bit of 64-bit projection)
    MINMAX_CHECK = 0xA, // R1 = (R_s1 < R_s2) ? 0 : 1 (Conservative bound check)
    XOR_PARITY   = 0xB, // R1 = popcount(R_s1 & R_s2) & 1 (Parity decision bit)
    ENTROPY_GATE = 0xC, // R1 = (popcount(R_s1) == popcount(R_s2)) ? 1 : 0 (Balanced state test)
    LOGIT_SHIFT  = 0xD, // R1 = ((R_s1 >> (R_s2 & 63)) > 0) ? 1 : 0 (Thresholding shifted feature)
    CLAMP_ACTION = 0xE, // R1 = (R_s1 > 1) ? (R_s2 & 1) : R_s1 (Bound action output to [0,1])
    BLEND_CHOICE = 0xF  // R1 = ((R_s1 ^ R_s2) & 1) (Stochastic tie-breaker selection)
};

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

int main() {
    uint8_t State[4] = {NoAction, NoAction, NoAction, NoAction};

    const enum GameState node_tree[27] = {
        KCheck, KCheckBet, KBetCall, KCheckBetCall,
        NoAction, NoAction, NoAction, NoAction, NoAction,
        QCheck, QCheckCheck, QBetFold, QCheckBetFold,
        NoAction, NoAction, NoAction, NoAction, NoAction,
        JBet, JCheckBet, JBetFold, JCheckBetFold,
        NoAction, NoAction, NoAction, NoAction, NoAction,
    };

    //
    for (uint32_t i = UINT32_MAX; i > 0; i--) {

    }

    return 0;
}

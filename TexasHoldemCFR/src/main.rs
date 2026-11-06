use input_method::*;

//mod suffle;

#[repr(usize)]
pub enum CardNum {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

#[repr(usize)]
pub enum CardSymbol {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

pub struct CardRep {
    pub num: CardNum,
    pub pip: CardSymbol,
}

impl CardRep {
    /// First 13 inputs are the flatten card number magnitudes.
    /// Next are 4 inputs for the Card Symbol magnitudes.
    pub fn to_bit_buf(self) -> [u8; 3] {
        let mut r: [u8; 3] = [0, 0, 0];
        let sym_nth = self.num as usize;
        let sym_nth_byte = (sym_nth / 8) as usize;
        let pip_nth = self.pip as usize;
        r[sym_nth_byte] = r[sym_nth >> sym_nth_byte % 8];

        return r;
    }

    /* Not for discrete models.
    pub fn to_f64_buf(self) -> [f64; 17] {
        let mut r = [0.0; 17];

        r[self.num as usize] = 1.0;
        r[13 + self.pip as usize] = 1.0;

        return r;
    }*/
}

struct Model {
    weights0: u8,
    thresholds0: u8,

    weights1: u8,
    thresholds1: u8,
}

impl Model {
    fn forward(&mut self, inputs: u8) -> u8 {
        let xnor_result = !(inputs ^ self.weights0);

        // Hardware POPCNT: 1-3 cycles
        let set_bits = xnor_result.count_ones() as u8;

        // Branchless activation (Thresholding)
        // If set_bits > threshold, return 1, else 0
        let layer_out0 = (set_bits > self.thresholds0) as u8;

        let xnor_result1 = !(layer_out0 ^ self.weights1);
        let set_bits1 = xnor_result1.count_ones() as u8;
        return (set_bits1 > self.thresholds1) as u8;
    }
}

const XOR_DATASET: [([u8; 2], u8); 4] = [
    // ( [x0,  x1 ], expected_y )
    ([0, 0], 0),
    ([0, 1], 1),
    ([1, 0], 1),
    ([1, 1], 0),
];

/* TODO: fill with the 52 card desk, to suffle cards  */
// https://en.wikipedia.org/wiki/Standard_52-card_deck
//const FIFTYTWO_CARD_DECK: [CardRep; 52] = {};

fn main() {
    /* TODO! Input Vector
    Each card takes: 17 elements
    Up to 5 community cards: 85 elements
    The Flop: 3 cards
    The Turn: 1 card
    The River: 1 card */

    let card_num = CardRep {
        num: CardNum::Two,
        pip: CardSymbol::Clubs,
    };

    /*
    let mut model: Model = Model {
        weights0: 0,
        thresholds0: 5,
        weights1: 0,
        thresholds1: 4,
    };

    // train
    for epoch in 0..512 {
        let row = epoch % 4;
        while true {
            let input: [u8; 2] = XOR_DATASET[row].0;
            let expected = XOR_DATASET[row].1;
            let vectorized_input = (input[0] << 4) | (input[1] & 0b00001111);
            let output = model.forward(vectorized_input);
            if expected == output {
                break;
            } else {
                // mutate some random bits here
                /*
                *
                * impl BinaryLayer {
                    pub fn update_threshold(&mut self, popcnt_result: u32, target: u64, output: u64) {
                        if output != target {
                            if target == 1 {
                                // False Negative: Neuron needs to fire easier. Lower the bar.
                                self.threshold = self.threshold.saturating_sub(1);
                            } else {
                                // False Positive: Neuron fired too easily. Raise the bar.
                                self.threshold = self.threshold.saturating_add(1);
                            }
                        }
                    }
                }
                */
            }
        }
    }
    */
}

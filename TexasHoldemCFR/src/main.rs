use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{Linear, Module, Optimizer, SGD, VarBuilder, VarMap, linear};
use input_method::*;

#[repr(usize)]
pub enum CardNum {
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

#[repr(usize)]
pub enum CardSymbol {
    Clubs = 1,
    Diamonds = 2,
    Hearts = 3,
    Spades = 4,
}

pub struct CardRep {
    pub num: CardNum,
    pub pip: CardSymbol,
}

impl CardRep {
    /// First 13 inputs are the flatten card number magnitudes.
    /// Next are 4 inputs for the Card Symbol magnitudes.
    pub fn to_f64_buf(self) -> [f64; 17] {
        let mut r = [0.0; 17];

        r[self.num as usize] = 1.0;
        r[13 + self.pip as usize] = 1.0;

        return r;
    }
}

//https://docs.rs/candle-core/0.11.0/candle_core/
struct SimpleMLP {
    fc1: Linear,
    fc2: Linear,
}

impl SimpleMLP {
    fn new(vs: VarBuilder) -> Result<Self> {
        let fc1 = linear(2, 4, vs.pp("fc1"))?;
        let fc2 = linear(4, 1, vs.pp("fc2"))?;
        Ok(Self { fc1, fc2 })
    }

    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let h = self.fc1.forward(xs)?.relu()?;
        candle_nn::ops::sigmoid(&self.fc2.forward(&h)?)
    }
}

fn main() -> Result<()> {
    let device = Device::Cpu;

    /* TODO! Input Vector
    Each card takes: 17 elements
    Up to 5 community cards: 85 elements
    The Flop: 3 cards
    The Turn: 1 card
    The River: 1 card

    */

    // XOR
    let x_data: &[f32] = &[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];
    let y_data: &[f32] = &[0.0, 1.0, 1.0, 0.0];

    let x = Tensor::from_slice(x_data, (4, 2), &device)?;
    let y = Tensor::from_slice(y_data, (4, 1), &device)?;

    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = SimpleMLP::new(vs)?;

    let mut opt = SGD::new(varmap.all_vars(), 0.5)?;

    for epoch in 0..=500 {
        let preds = model.forward(&x)?;

        let loss = preds.sub(&y)?.sqr()?.mean_all()?;

        opt.backward_step(&loss)?;

        if epoch % 100 == 0 {
            let loss_val: f32 = loss.to_scalar()?;
            println!("Epoch {:3} | Loss: {:.6}", epoch, loss_val);
        }
    }

    let test_preds = model.forward(&x)?;
    println!("\nTrained Predictions vs Targets:");
    for i in 0..4 {
        let pred: f32 = test_preds.get(i)?.get(0)?.to_scalar()?;
        let target = y_data[i];
        println!(
            "Input: [{:.0}, {:.0}] -> Pred: {:.4} (Target: {:.0})",
            x_data[i * 2],
            x_data[i * 2 + 1],
            pred,
            target
        );
    }

    Ok(())
}

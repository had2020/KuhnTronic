use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{Linear, Module, Optimizer, SGD, VarBuilder, VarMap, linear};

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

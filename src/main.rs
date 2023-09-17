mod pipeline;

use candle_core::{Device, Tensor};

use tokenizers::{Tokenizer, Model};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;
    Pip

    let tokenizer = Tokenizer::from_pretrained("microsoft/codebert-base", None)?;
    let tokenizer = Model::from_pretrained("microsoft/codebert-base", None)?;

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0f32, 1., (3, 4), &device)?;

    let c = a.matmul(&b)?;
    println!("{c}");
    Ok(())
}
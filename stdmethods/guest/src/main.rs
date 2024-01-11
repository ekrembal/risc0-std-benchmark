#![no_main]


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: Vec<u32> = env::read();
    // calculate the sum
    let sum: u32 = input.iter().sum();

    // write public output to the journal
    env::commit(&sum);
}

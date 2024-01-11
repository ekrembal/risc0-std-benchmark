#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let num_input: u32 = env::read();
    let mut sum: u32 = 0;
    for _ in 0..num_input {
        let input: u32 = env::read();
        sum += input;
    }

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&sum);
}

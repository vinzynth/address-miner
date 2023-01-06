use ethers::types::Address;
use ethers_core::rand;
use ethers_signers::{
    coins_bip39::{English, Mnemonic},
    MnemonicBuilder, Signer, WalletError,
};

use num_cpus;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;

fn main() -> Result<(), WalletError> {
    let num_cores = num_cpus::get();

    let bits = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();
    for _ in 0..num_cores {
        let bit_clone = Arc::clone(&bits);
        let handle = thread::spawn(move || {
            let mut old_bits = 0;
            'outer: loop {
                match create_wallet() {
                    Ok((address, phrase)) => {
                        for i in (0..(old_bits / 8)).rev() {
                            if address[i] > 0 {
                                continue 'outer;
                            }
                        }

                        let new_bits = count_leading_zeros(address.as_bytes());
                        old_bits = bit_clone.load(Ordering::Relaxed);
                        if new_bits > old_bits {
                            bit_clone.store(new_bits, Ordering::Relaxed);
                            println!("{:4} {:?} {}", new_bits, address, phrase);
                        }
                    }
                    Err(e) => {
                        println!("Error {}", e);
                        return;
                    }
                };
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn create_wallet() -> Result<(Address, String), WalletError> {
    let mut rng = rand::thread_rng();
    let phrase = Mnemonic::<English>::new(&mut rng);
    let phrase_str = phrase.to_phrase()?;

    let wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase_str.as_ref())
        .build()?;

    let address = wallet.address();

    Ok((address, phrase_str))
}

fn count_leading_zeros(bytes: &[u8]) -> usize {
    let mut count = 0;
    for byte in bytes {
        if *byte == 0 {
            count += 8;
        } else {
            count += byte.leading_zeros();
            break;
        }
    }
    count as usize
}

# Vanity Address Miner

This CLI tool creates BIP-39 based mnemonics which have a vanity public address with leading zeros.
Public addresses will be derived according to the BIP-44 derivation path `m/44'/60'/0'/0/0`.

## Usage

Build with `cargo build --release` to enable compiler optimizations.

The output follows the following format:
`<zero bits> <public address> <mnemonic>`

Sample output:
```
   1 0x44c8c315c62bf054e990177f49ba9266aa398039 trick swarm use ...
   2 0x37ac29919060aa226b783934b0717921d1609f4e surround oblige ...
  10 0x0035d38abbbad1a97a307def494a7dfae5eb02f4 ask sadness poem...
  11 0x001a5b3f37e58f293b30748418fefcfd6c1b1516 fuel engine enti...
  13 0x00068b243c97baff36c8937bba93982d655f3fb7 rookie march str...
  14 0x0003043d05cbd8cc4f582ee1af601447238d85b4 produce man kid ...
  16 0x0000b3a5390d1e5e7d73182f05ca2293449c9cd4 exhibit vessel t...
  ...
```

## Random Number Generation

This program utilizes the `ethers` library, which itself relies on the commonly used `rand` library and initializes
the random number generator with a seed from the operating system.

# Use at your own risk
Read into articles about the Profanity hack (September 2022) for more information about possible consequences when using
vanity addresses.
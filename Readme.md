# Solana Secp256k1

This crate leverages two Solana syscalls—`big_mod_exp` (for Fermat's Little Theorem) and `secp256k1_recover`—to create compute unit (CU)-efficient implementations of all the mathematical functions required to utilize the Secp256k1 curve for arbitrary on-chain cryptographic operations. Most notably, scalar tweaking and elliptic curve (EC) multiplication now cost just 25,000 CUs, a 200x reduction from their initial ~5,000,000 CU cost. This library supports highly performant versions of:

- Point compression
- Point decompression
- Point addition (ECAdd)
- Public key generation (MulG)
- Point multiplication (ECMul)
- Key tweaking (`ECAdd(P, MulG(scalar))`)
- Negate scalar \( P \)
- Negate scalar \( N \)
- Modular inverse of \( P \) (Modinv \( P \))
- Modular inverse of \( N \) (Modinv \( N \))

### Mathematical Explanation

Unlike the Ethereum implementation that applies a Keccak-256 hash and truncates the recovered point into an address, Solana's implementation of `ecrecover` returns an uncompressed public key point. Therefore, the mathematical formula for `ecrecover` on Solana can be defined as:

\[ Q = r^{-1}(sR - zG) \]

where:

- \( Q \) is the recovered point.
- \( r \) is the nonce.
- \( R \) is a point with the \( x \)-coordinate of \( r \) and the \( y \)-coordinate defined by the recovery ID \( v \).
- \( z \) is the hash scalar.
- \( G \) is the generator point.

The input parameters we can control are \( z \), \( v \), \( r \), and \( s \).

By leveraging this, we can utilize `ecrecover` to perform a variety of cryptographic functions. For example:

###### ECMul (Elliptic Curve Multiplication)

To perform ECMul, we zero out the right-hand side of the equation by setting the hash scalar \( z = 0 \). This simplifies the formula to:

\[ Q = r^{-1}(sR) \]

If we set \( s = k \cdot r \), we can eliminate the modular inverse, reducing the formula to:

\[ Q = kR \]

##### Scalar Tweaking

We can expand upon the ECMul example by utilizing the right-hand side of the equation, \( -zG \). This term represents a `MulG` operation, generating a public key point from a scalar value. By negating the input scalar and multiplying by \( r \) to cancel out the modular inverse, we reduce the formula to:

\[ Q = sR + zG \]

This enables an efficient implementation of tweaked public keys.

### Use Cases

This crate primarily enables efficient on-chain verification of Schnorr signatures and facilitates TapTweaks for on-chain Taproot address generation. This allows Solana not only to verify Bitcoin transactions but also to act as an MPC provider for transaction creation and liquidity management via on-chain Bitcoin wallets. Additionally, this library opens up possibilities for:

- Pedersen commitments
- On-chain ECDSA/Schnorr signing, enabling PDA signers on Bitcoin/Ethereum
- Ring signatures
- Bulletproofs

### Disclaimer

While this library will be audited, remember to use it at your own risk.

### TODO

- Auditing
- Reimplement point doubling method
- Improve ECAdd performance
- Enhance testing
- Optimize syscalls with `no_std` variants
- Remove dependency on `solana-program`
- Implement multiple compile targets for more efficient implementations in Rust/WASM
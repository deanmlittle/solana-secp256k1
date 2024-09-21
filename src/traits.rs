use crate::*;

/// # Point from Scalar
///
/// Abuse the secp256k1_ecrecover syscall to create a public key point from a private key scalar onchain
///
/// The mathematical formula for calculating s in an v,r,s tuplet is:
/// v = R.Y % 2 (Polarity of the Y coordinate of R)
/// r = R.X (X-coordinate of ephemeral key k)
/// s = 𝑘^−1 (ℎ+𝑟∗𝑝𝑟𝑖𝑣𝐾𝑒𝑦) modulus 𝑁
///
/// Thus, if k=1 and h=0, we can eliminate both the k^-1, as the modular inverse of any number and 1 is always 1, and the integer addition of h. The resulting formula is thus simplified to become:
///
/// s = (𝑟∗𝑝𝑟𝑖𝑣𝐾𝑒𝑦) modulus 𝑁
///
/// By using the precomputed r value of k=1, also known as the generator point G of the curve, and that and R.Y will always be even, we can create a valid ecdsa signature onchain, enabling us to use ecrecover to efficiently recover what its public key would have been. This allows us to generate uncompressed points from scalars onchain at a discount of ~4 million CUs compared to a naive implementation.
pub trait Secp256k1Point:
    TryFrom<[u8; 32]> + From<[u8; 65]> + Clone + PartialEq + Eq
{
    /// ### Size
    /// 
    /// Size of the Point type, 33 bytes compressed or 64 bytes uncompressed
    const SIZE: usize;
    
    /// ### Is Odd
    /// Returns true if the 𝑌-coordinate of the underlying point is odd 
    fn is_odd(&self) -> bool;

    /// ### Is Even
    /// Returns true if the 𝑌-coordinate of the underlying point is even 
    fn is_even(&self) -> bool;

    /// ### 𝑋
    /// Returns the 𝑋 coordinate of the underlying point  
    fn x(&self) -> [u8; 32];

    /// ### 𝑋
    /// Returns the 𝑋 coordinate of the underlying point  
    fn y(&self) -> [u8; 32];

    /// ### Lift 𝑋 Unchecked
    /// 
    /// Finds the corresponding 𝑌-coordinate of a given 𝑋-coordinate. This does not guarantee the
    /// returned point is on curve. Only use this when you know you are dealing with a valid point. 
    /// Otherwise, consider lift_x_checked
    fn lift_x_unchecked(x: &[u8; 32]) -> Self;

    /// ### Lift 𝑋
    /// 
    /// Works the same way as Lift 𝑋 unchecked, additionally checking if the returned 𝑌-coordinate is on
    /// curve. Useful for situations where you already know that an 𝑋-coordinate is on curve, such as
    /// decompressing the public key that corresponds to a validated signature.
    fn lift_x(x: &[u8; 32]) ->Result<Self, Secp256k1Error>;

    /// ### Invert
    /// 
    /// Inverts the parity of the 𝑌-coordinate of a point.
    fn invert(&mut self);  
}
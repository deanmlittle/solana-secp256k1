use dashu::integer::{fast_div::ConstDivisor, modular::IntoRing, UBig};
use solana_nostd_secp256k1_recover::secp256k1_recover;

use crate::*;
pub struct Curve;

impl Curve {
    /// ### Curve order 𝑁
    /// 
    /// 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141
    /// 
    /// This is the order 𝑁 of the secp256k1 elliptic curve. The order of the curve is 
    /// a large prime number that determines the size of the cyclic group generated by 
    /// the base point 𝐺.
    /// 
    /// All valid private keys must be less than 𝑁, and all points on the curve are 
    /// within a cyclic group of this order.
    pub const N: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xfe, 0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36,
        0x41, 0x41,
    ];

    /// ### Curve order 𝑁-2
    /// 
    /// 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd036413f
    ///
    /// The precomputed value of 𝑁−2 where 𝑁 is the order of the secp256k1 curve. 
    /// Used in modular arithmetic operations, such as modular inverse.
    pub const N_SUB_2: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xfe, 0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36,
        0x41, 0x3f,
    ];

    /// ### Curve order 𝑁-2
    /// 
    /// 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd036413f
    ///
    /// The precomputed value of 𝑁/2 where 𝑁 is the order of the secp256k1 curve. 
    /// Used in integer comparison for high S checks in ECDSA.
    pub const N_DIV_2: [u8;32] = [
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
        0xFF, 0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D, 0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B, 
        0x20, 0xA0
    ];

    /// ### Field Prime Modulus 𝑃
    /// 
    /// 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f
    /// 
    /// This is the prime number 𝑃 that defines the finite field over which secp256k1 is defined. 
    /// All arithmetic operations on the curve are performed modulo this prime number.
    pub const P: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff,
        0xfc, 0x2f,
    ];

    /// ### Field Prime Modulus 𝑃-2
    /// 
    /// 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2d
    ///  
    /// This represents 𝑃−2, where 𝑃 is the field prime of the secp256k1 curve. This value 
    /// is modular arithmetic operations, such as modular inverse.
    pub const P_SUB_2: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff,
        0xfc, 0x2d,
    ];

    /// ### Field Prime Modulus 𝑃+1/4
    /// 
    /// 0x3fffffffffffffffffffffffffffffffffffffffffffffffffffffffbfffff0c
    /// 
    /// This is the value 𝑃+1/4, where 𝑃 is the field prime. This value is precomputed for 
    /// efficiency and is used in calculating square roots in the field.
    pub const P_1_4: [u8; 32] = [
        0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xbf, 0xff,
        0xff, 0x0c,
    ];

    /// ### Generator Point 𝐺
    /// 
    /// 𝐺.𝑋: 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798
    /// 𝐺.𝑌: 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8
    /// 
    /// 𝐺 is a fixed point on the secp256k1 curve used as the base point for generating public keys. 
    /// All elliptic curve operations are performed with respect to this generator point.
    pub const G: UncompressedPoint = UncompressedPoint([
        0x79, 0xBE, 0x66, 0x7E, 0xF9, 0xDC, 0xBB, 0xAC, 0x55, 0xA0, 0x62, 0x95, 0xCE, 0x87, 0x0B,
        0x07, 0x02, 0x9B, 0xFC, 0xDB, 0x2D, 0xCE, 0x28, 0xD9, 0x59, 0xF2, 0x81, 0x5B, 0x16, 0xF8,
        0x17, 0x98, 0x48, 0x3A, 0xDA, 0x77, 0x26, 0xA3, 0xC4, 0x65, 0x5D, 0xA4, 0xFB, 0xFC, 0x0E,
        0x11, 0x08, 0xA8, 0xFD, 0x17, 0xB4, 0x48, 0xA6, 0x85, 0x54, 0x19, 0x9C, 0x47, 0xD0, 0x8F,
        0xFB, 0x10, 0xD4, 0xB8,
    ]);

    /// ### Add Mod Point 𝑁
    /// 
    /// Adds two scalars modulus curve order N.
    pub fn add_mod_n(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        // Convert the input &[u8; 32] to BigUint
        let a_int = UBig::from_be_bytes(a);
        let b_int = UBig::from_be_bytes(b);
        let n_int = UBig::from_be_bytes(&Self::N);
    
        // Perform the modular addition
        let res_int = (a_int + b_int) % n_int;
        let res_bytes = res_int.to_be_bytes();
    
        // Prepare a fixed 32-byte array for the result
        let mut result = [0u8; 32];
    
        // Copy the bytes from res_bytes into result, ensuring correct size
        let len = res_bytes.len();
        result[32 - len..].copy_from_slice(&res_bytes);
        result
    }    

    /// ### Mul Mod Point 𝑁
    /// 
    /// Multiplies a scalar by another scalar modulus curve order N. Typically used to create
    /// a normalized nonce/private key scalar.
    pub fn mul_mod_n(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        // Convert the input &[u8; 32] to Integer
        let a_int = UBig::from_be_bytes(a);
        let b_int = UBig::from_be_bytes(b);
        let n_int = UBig::from_be_bytes(&Self::N);

        // Perform the modular addition
        let res_int = (a_int * b_int) % n_int;
        let res_bytes = res_int.to_be_bytes();
    
        // Prepare a fixed 32-byte array for the result
        let mut result = [0u8; 32];
    
        // Copy the bytes from res_bytes into result, ensuring correct size
        let len = res_bytes.len();
        result[32 - len..].copy_from_slice(&res_bytes);
        result
    }

    /// ### Add Mod Point 𝑃
    /// 
    /// Adds two scalars modulus prime order 𝑃.
    pub fn add_mod_p(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        // Convert the input &[u8; 32] to BigUint
        let a_int = UBig::from_be_bytes(a);
        let b_int = UBig::from_be_bytes(b);
        let p_int = UBig::from_be_bytes(&Self::P);
    
        // Perform the modular addition
        let res_int = (a_int + b_int) % p_int;
        let res_bytes = res_int.to_be_bytes();
    
        // Prepare a fixed 32-byte array for the result
        let mut result = [0u8; 32];
    
        // Copy the bytes from res_bytes into result, ensuring correct size
        let len = res_bytes.len();
        result[32 - len..].copy_from_slice(&res_bytes);
        result
    }    

    /// ### Mul Mod Point 𝑃
    /// 
    /// Multiplies a scalar by another scalar modulus prime order 𝑃.
    pub fn mul_mod_p(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        // Convert the input &[u8; 32] to Integer
        let a_int = UBig::from_be_bytes(a);
        let b_int = UBig::from_be_bytes(b);
        let p_int = UBig::from_be_bytes(&Self::P);

        // Perform the modular addition
        let res_int = (a_int * b_int) % p_int;
        let res_bytes = res_int.to_be_bytes();
    
        // Prepare a fixed 32-byte array for the result
        let mut result = [0u8; 32];
    
        // Copy the bytes from res_bytes into result, ensuring correct size
        let len = res_bytes.len();
        result[32 - len..].copy_from_slice(&res_bytes);
        result
    }

    /// ### Decompress Point
    /// 
    /// Decompresses a point by recovering it with parity
    pub fn decompress(p: CompressedPoint) -> Result<UncompressedPoint, Secp256k1Error> {
        let mut s = [0u8;64];
        s[..32].clone_from_slice(&p.0[1..33]);
        s[32..].clone_from_slice(&p.0[1..33]);
        Ok(UncompressedPoint(secp256k1_recover(&[0u8; 32], false, &s)?))
    }

    /// ### Decompress Point Unchecked
    /// 
    /// Decompresses a point by recovering it with parity without checking it is on curve
    pub fn decompress_unchecked(p: CompressedPoint) -> UncompressedPoint {
        let mut s = [0u8;64];
        s[..32].clone_from_slice(&p.0[1..33]);
        s[32..].clone_from_slice(&p.0[1..33]);
        UncompressedPoint(secp256k1_recover(&[0u8; 32], p.is_odd(), &s).expect("Point off curve"))
    }

    /// ### Lift X coordinate to curve
    /// 
    /// Lifts an X coordinate to curve and checks for a valid Y coordinate
    pub fn lift_x(x: &[u8;32]) -> Result<UncompressedPoint, Secp256k1Error> {        
        let p = UBig::from_be_bytes(&Curve::P);

        // Calculate right side: x³ + 7
        let x_3 = UBig::from_be_bytes(x).cubic() + UBig::from_word(7);
        
        let divisor = ConstDivisor::new(p.clone());
        let exp = x_3.clone().into_ring(&divisor);
        
        // Calculate y = (x³ + 7)^((p+1)/4) mod p
        let y = exp.pow(&(UBig::from_be_bytes(&Curve::P_1_4))).residue() % &p;

        if y.sqr() % &p != x_3 % &p {
            return Err(Secp256k1Error::InvalidYCoordinate);
        }

        let y_bytes = y.to_be_bytes();

        let mut point = [0u8;64];
        point[..32].clone_from_slice(x);
        point[64 - y_bytes.len()..].clone_from_slice(&y_bytes);
        Ok(UncompressedPoint(point))
    }

    /// ### Lift X coordinate to curve unchecked
    /// 
    /// Lifts an X coordinate to curve and checks for a valid Y coordinate
    pub fn lift_x_unchecked(x: &[u8;32]) -> UncompressedPoint {        
        let p = UBig::from_be_bytes(&Curve::P);

        // Calculate right side: x³ + 7
        let x_3 = UBig::from_be_bytes(x).cubic() + UBig::from_word(7);
        
        let divisor = ConstDivisor::new(p.clone());
        let exp = x_3.into_ring(&divisor);
        
        // Calculate y = (x³ + 7)^((p+1)/4) mod p
        let y = exp.pow(&(UBig::from_be_bytes(&Curve::P_1_4))).residue() % &p;

        let y_bytes = y.to_be_bytes();

        let mut point = [0u8;64];
        point[..32].clone_from_slice(x);
        point[64 - y_bytes.len()..].clone_from_slice(&y_bytes);
        UncompressedPoint(point)
    }

    /// # Fast Mod 𝑃
    /// 
    /// Taking advantage of the fact that:
    /// - This function is only used with 256-bit numbers
    /// - We will almost never need to mod 𝑃 as 𝑃 is very large
    /// - In the case that we do, we only need to handle the last 8 bytes.
    /// 
    /// We can optimize this beyond 
    pub fn fast_mod_p(a: &mut [u8; 32]) {
        // Transmute the &mut [u8; 32] into &mut [u64; 4]
        let a_u64: &mut [u64; 4] = unsafe { std::mem::transmute(a) };
    
        if a_u64[0] < u64::MAX || a_u64[1] < u64::MAX || a_u64[2] < u64::MAX {
            return;
        }
    
        let max = 0xfffffffefffffc2f;
        if a_u64[3] >= max {
            a_u64[0] = 0;
            a_u64[1] = 0;
            a_u64[2] = 0;
            a_u64[3] -= max;
        }
    }

    /// # Fast Mod 𝑁
    /// 
    /// Taking advantage of the fact that:
    /// - This function is only used with 256-bit numbers
    /// - We will almost never need to mod 𝑁 as 𝑁 is very large
    /// - In the case that we do, we only need to handle the last 33 bytes.
    /// 
    /// While this may not necessarily be faster in all cases, it will be faster 
    /// on average to veto modulus by the first limb.
    pub fn fast_mod_n(a: &mut [u8; 32]) {
        // Transmute the &mut [u8; 32] into &mut [u64; 4]
        let a_u64: &mut [u64; 4] = unsafe { std::mem::transmute(a) };
    
        // This will almost always be true. Skip to avoid allocating and comparing remaining limbs
        if a_u64[0] < u64::MAX {
            return
        }

        let n1 = u64::MAX-1;
        let n2 = 0xbaaedce6af48a03bu64;
        let n3 = 0xbfd25e8cd036413fu64;
        
        // This will also almost always be true, if not, we can directly set a_u64[0] to 0 and subtract the remaining limbs
        if a_u64[1] < n1 && a_u64[2] < n2 && a_u64[3] < n3 {
            return
        }

        // Subtraction with borrow propagation (from LSB to MSB)
        let (new3, borrow3) = a_u64[3].overflowing_sub(n3);
        let (new2, borrow2) = a_u64[2].overflowing_sub(n2 + borrow3 as u64);
        let (new1, _) = a_u64[1].overflowing_sub(n1 + borrow2 as u64);

        // Assign results back
        a_u64[3] = new3;
        a_u64[2] = new2;
        a_u64[1] = new1;
        a_u64[0] = 0;
    }

    /// ### Negate
    /// 
    /// Negates the provided 32-byte value `𝒌` modulo order 𝑁 of the Secp256k1 curve.
    /// 
    /// Elliptic curve negation of `𝒌` over curve order 𝑁.
    /// 
    /// Calculate `𝑁 - 𝒌` and stores the result back into `𝒌`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use solana_secp256k1::Curve;
    /// let mut k: [u8; 32] = [
    ///     0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ///     0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ///     0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    ///     0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
    /// ]; // Some 32-byte scalar value
    /// Curve::negate_n(&mut k);
    /// // `k` now contains the value (𝑁 - original_k) modulo 𝑁.
    /// ```
    /// 
    pub fn negate_n(k: &[u8; 32]) -> [u8;32] {
        let n = UBig::from_be_bytes(&Curve::N);
        let x = ((&n + &n - UBig::from_be_bytes(k)) % &n).to_be_bytes();
        let mut r = [0u8;32];
        r[32-x.len()..].clone_from_slice(&x);
        r
    }

    pub fn negate_n_assign(k: &mut [u8; 32]) {
        let n = UBig::from_be_bytes(&Curve::N);
        let x = ((n.clone() + n.clone() - UBig::from_be_bytes(k)) % n.clone()).to_be_bytes();
        k[..32-x.len()].clone_from_slice(&[0u8;32][..32-x.len()]);
        k[32-x.len()..].clone_from_slice(&x);
    }

    pub fn negate_p(k: &[u8; 32]) -> [u8;32] {
        let p = UBig::from_be_bytes(&Curve::P);
        let x = ((p.clone() + p.clone() - UBig::from_be_bytes(k)) % p.clone()).to_be_bytes();
        let mut r = [0u8;32];
        r.clone_from_slice(&x);
        r
    }

    pub fn negate_p_assign(k: &mut [u8; 32]) {
        let p = UBig::from_be_bytes(&Curve::P);
        let x = ((p.clone() + p.clone() - UBig::from_be_bytes(k)) % p.clone()).to_be_bytes();
        k[..32-x.len()].clone_from_slice(&[0u8;32][..32-x.len()]);
        k[32-x.len()..].clone_from_slice(&x);
    }

    // TODO: Check which is cheaper on CUs.
    // pub fn negate_p_assign(k: &mut [u8; 32]) {
    //     let mut borrow = 0u8;
    //     for i in (0..32).rev() {
    //         let (res, b) = Self::P[i].overflowing_sub(k[i] + borrow);
    //         k[i] = res;
    //         borrow = if b { 1 } else { 0 };
    //     }
    // }

    /// ### Modular Inverse 𝑁
    /// 
    /// Calculates the modular inverse of `𝒌` using Fermat's Little Theorem, which states that
    /// for a prime modulus 𝑷, the modular inverse of 𝒌 modulo `𝑁` is given by `𝒌⁽ᴺ⁻²⁾ mod 𝑁`.
    /// 
    /// The modulus with the exponent `𝑁-2`.
    /// 
    /// The `big_mod_exp` function is used to efficiently compute the modular exponentiation.
    /// 
    /// # Example
    ///
    /// ```rust
    /// use solana_secp256k1::Curve;
    /// 
    /// let k: [u8; 32] = [
    ///     0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ///     0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ///     0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    ///     0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
    /// ]; // Some 32-byte scalar value
    /// let inv_k = Curve::mod_inv_n(&k);
    /// // `inv_k` now contains the value of (𝒌⁻¹) modulo 𝑁.
    /// ```
    pub fn mod_inv_n(k: &[u8]) -> Result<[u8; 32], Secp256k1Error> {
        let mut inv_k: [u8; 32] = [0u8; 32];
        let ring = ConstDivisor::new(UBig::from_be_bytes(&Self::N));
        let res = ring.reduce(UBig::from_be_bytes(k)).inv().ok_or(Secp256k1Error::ArithmeticOverflow)?.residue().to_be_bytes();
        inv_k[32-res.len()..].clone_from_slice(&res);
        Ok(inv_k)
    }

    /// ### Modular Inverse 𝑃
    /// 
    /// Calculates the modular inverse of `𝒌` using Fermat's Little Theorem, which states that
    /// for a prime modulus 𝑷, the modular inverse of 𝒌 modulo `𝑃` is given by `𝒌⁽ᴺ⁻²⁾ mod 𝑃`.
    /// 
    /// The modulus with the exponent `𝑃-2`.
    /// 
    /// The `big_mod_exp` function is used to efficiently compute the modular exponentiation.
    /// 
    /// # Example
    ///
    /// ```rust
    /// use solana_secp256k1::Curve;
    /// 
    /// let k: [u8; 32] = [
    ///     0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ///     0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ///     0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    ///     0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
    /// ]; // Some 32-byte scalar value
    /// 
    /// let inv_k = Curve::mod_inv_p(&k);
    /// // `inv_k` now contains the value of (𝒌⁻¹) modulo 𝑃.
    /// ```
    pub fn mod_inv_p(k: &[u8]) -> Result<[u8; 32], Secp256k1Error> {
        let mut inv_k: [u8; 32] = [0u8; 32];
        let ring = ConstDivisor::new(UBig::from_be_bytes(&Self::P));
        let res = ring.reduce(UBig::from_be_bytes(k)).inv().ok_or(Secp256k1Error::ArithmeticOverflow)?.residue().to_be_bytes();
        inv_k[32-res.len()..].clone_from_slice(&res);
        Ok(inv_k)
    }

    /// ### Mul 𝐺
    /// 
    /// Abuse Secp256k1Recover to Calculate the UncompressedPoint of Scalar `𝒌`
    /// 
    /// # Example
    ///
    /// ```rust
    /// use solana_secp256k1::Curve;
    /// 
    /// let k: [u8; 32] = [
    ///     0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ///     0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ///     0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    ///     0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
    /// ]; // Some 32-byte scalar value
    /// let p = Curve::mul_g(&k);
    /// ```
    pub fn mul_g(k: &[u8;32]) -> Result<UncompressedPoint, Secp256k1Error> {
        let result = Self::mul_mod_n(&k, &Self::G.x());
        let mut s = [0u8;64];
        s[..32].clone_from_slice(&Self::G.x());
        s[32..].clone_from_slice(&result);
        Ok(UncompressedPoint(secp256k1_recover(&[0u8; 32], false, &s)?))
    }

    /// ### Ecmul
    /// 
    /// Abuses Secp256k1 ECRecover to perform efficient ECMul, adding a point to itself N times.
    /// This would ordinarily cost about ~5 million CUs. Perplexingly, we can make it cost way
    /// less than ECAdd by abusing a bespoke half-implementation of an old Ethereum precompile.
    /// In fact, not following the original implementation actually makes it work even better.
    /// 
    /// This unlocks efficient implementations of ECDH, Pedersen commitments.
    /// 
    /// # Example
    ///
    /// ```rust
    /// use solana_secp256k1::{Curve, CompressedPoint};
    /// 
    /// let k: [u8; 32] = [
    ///     0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ///     0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    ///     0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
    ///     0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
    /// ]; // Some 32-byte scalar value
    /// let point = CompressedPoint([
    ///     0x03, 0x40, 0xa7, 0xc9, 0xe2, 0x07, 0x2a, 0xe2, 
    ///     0x5c, 0x1b, 0x79, 0xca, 0x79, 0xe9, 0x56, 0x57, 
    ///     0x61, 0xc4, 0xa6, 0xe8, 0xd3, 0x0a, 0xc2, 0x5b, 
    ///     0x15, 0x41, 0xe0, 0x2e, 0xbb, 0x8d, 0xd2, 0x31, 
    ///     0xdf,
    /// ]); 
    /// // A compressed or uncompressed point
    /// let p = Curve::ecmul::<CompressedPoint>(&point, &k);
    /// ```
    
    pub fn ecmul<T: Secp256k1Point>(point: &T, k: &[u8;32]) -> Result<UncompressedPoint, Secp256k1Error> {
        let result = Self::mul_mod_n(&point.x(), k);
        let mut s = [0u8;64];
        s[..32].clone_from_slice(&point.x());
        s[32..].clone_from_slice(&result);
        Ok(UncompressedPoint(secp256k1_recover(&[0u8; 32], point.is_odd(), &s)?))
    }
}

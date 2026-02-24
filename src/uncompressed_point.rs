use core::{fmt::{Debug, Formatter}, ops::Add};

use dashu::integer::{fast_div::ConstDivisor, UBig};
use solana_nostd_secp256k1_recover::secp256k1_recover;

#[cfg(feature="big-mod-exp")]
use solana_nostd_big_mod_exp::big_mod_exp;


use crate::{CompressedPoint, Curve, Secp256k1Error, Secp256k1Point};

pub const SEC1_OCTET_UNCOMPRESSED: u8 = 0x04;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct UncompressedPoint(pub [u8; Self::SIZE]);

impl Secp256k1Point for UncompressedPoint {
    const SIZE: usize = 64;

    fn is_odd(&self) -> bool {
        self.0[63] & 1 != 0
    }

    fn is_even(&self) -> bool {
        self.0[63] & 1 != 1
    }

    fn x(&self) -> [u8; 32] {
        [
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
            self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14],
            self.0[15], self.0[16], self.0[17], self.0[18], self.0[19], self.0[20], self.0[21],
            self.0[22], self.0[23], self.0[24], self.0[25], self.0[26], self.0[27], self.0[28],
            self.0[29], self.0[30], self.0[31],
        ]
    }

    fn y(&self) -> [u8; 32] {
        [
            self.0[32], self.0[33], self.0[34], self.0[35], self.0[36], self.0[37], self.0[38],
            self.0[39], self.0[40], self.0[41], self.0[42], self.0[43], self.0[44], self.0[45],
            self.0[46], self.0[47], self.0[48], self.0[49], self.0[50], self.0[51], self.0[52],
            self.0[53], self.0[54], self.0[55], self.0[56], self.0[57], self.0[58], self.0[59],
            self.0[60], self.0[61], self.0[62], self.0[63],
        ]
    }

    #[cfg(feature="big-mod-exp")]
    fn lift_x(x: &[u8; 32]) -> Result<Self, Secp256k1Error> {
        // y^2 = x^3 + 7 mod P
        let x_3 = (&UBig::from_be_bytes(x).pow(3) + UBig::from_word(7)) % &UBig::from_be_bytes(&Curve::P);
        // Use big_mod_exp for cheap cubed root
        let y = big_mod_exp(&x_3.to_be_bytes(), &Curve::P_1_4, &Curve::P);
        if (&UBig::from_be_bytes(&y).pow(2) % &UBig::from_be_bytes(&Curve::P)) != x_3 {
            return Err(Secp256k1Error::InvalidYCoordinate);
        }
        let mut x_y = [0u8; 64];
        x_y[..32].clone_from_slice(x);
        x_y[32..].clone_from_slice(&y);
        Ok(Self(x_y))
    }

    #[cfg(feature="big-mod-exp")]
    fn lift_x_unchecked(x: &[u8; 32]) -> Self {
        // We first compute y^2 = x^3 + 7 mod P
        let x_3 = (&UBig::from_be_bytes(x).pow(3) + UBig::from_word(7)) % &UBig::from_be_bytes(&Curve::P);
        // Use big_mod_exp for cheap cubed root
        let y = big_mod_exp(&x_3.to_be_bytes(), &Curve::P_1_4, &Curve::P);
        let mut x_y = [0u8; 64];
        x_y[..32].clone_from_slice(x);
        x_y[32..].clone_from_slice(&y);
        Self(x_y)
    }

    #[cfg(not(feature="big-mod-exp"))]
    fn lift_x(x: &[u8; 32]) -> Result<Self, Secp256k1Error> {
        Curve::lift_x(x)
    }

    #[cfg(not(feature="big-mod-exp"))]
    fn lift_x_unchecked(x: &[u8; 32]) -> Self {
        Curve::lift_x_unchecked(x)
    }

    fn invert(&mut self) {
        let y = (UBig::from_be_bytes(&Curve::P) - UBig::from_be_bytes(&self.y())).to_be_bytes();
        self.0[32..64].clone_from_slice(&[0u8;32]);
        self.0[64-y.len()..].clone_from_slice(&y);
    }
    
    fn compress(&self) -> CompressedPoint {
        CompressedPoint::from(*self)
    }
    
    fn decompress(&self) -> UncompressedPoint {
        *self
    }

    fn tweak(&self, tweak: [u8; 32]) -> Result<Self, Secp256k1Error> {
        // Compute z = (-r * k) mod N
        let z_scalar = ((UBig::from_be_bytes(&Curve::negate_n(&self.x())) * UBig::from_be_bytes(&tweak)) % UBig::from_be_bytes(&Curve::N)).to_be_bytes();
               
        // Ensure z and s are 32 bytes
        let mut z = [0u8; 32];
        z[32 - z_scalar.len()..].copy_from_slice(&z_scalar);

        let s: [u8; 64] = [
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
            self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15],
            self.0[16], self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22], self.0[23],
            self.0[24], self.0[25], self.0[26], self.0[27], self.0[28], self.0[29], self.0[30], self.0[31],
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
            self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15],
            self.0[16], self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22], self.0[23],
            self.0[24], self.0[25], self.0[26], self.0[27], self.0[28], self.0[29], self.0[30], self.0[31],
        ];

        // Use ecrecover with negated z to perform ECAdd
        Ok(UncompressedPoint(secp256k1_recover(&z, self.is_odd(), &s)?))
    }
}

impl Add<UncompressedPoint> for UncompressedPoint {
    type Output = UncompressedPoint;

    fn add(self, rhs: UncompressedPoint) -> Self::Output {
        if self.y() == rhs.y() && self.x() == rhs.x() {
            return self.double();
        }

        let p = UBig::from_be_bytes(&Curve::P);
        let p2 = &p + &p;
        let ring = ConstDivisor::new(p.clone());

        let x_p = UBig::from_be_bytes(&self.x());
        let y_p = UBig::from_be_bytes(&self.y());
        let x_q = UBig::from_be_bytes(&rhs.x());
        let y_q = UBig::from_be_bytes(&rhs.y());

        // modinv(x_q - x_p, p) — computed directly without byte round-trip
        let inv = ring.reduce(&x_q + &p - &x_p).inv().expect("Points are inverses");
        let inv = inv.residue();

        // m = (y_q - y_p) * modinv(x_q - x_p, p)
        let m = (&y_q + &p - &y_p) * inv % &p;

        // xr = m^2 - x_p - x_q (add 2p: x_p + x_q can be up to 2p-2)
        let xr = (&m * &m + &p2 - &x_p - &x_q) % &p;

        // yr = m * (x_p - xr) - y_p
        let yr = (&m * (&x_p + &p - &xr) + &p - &y_p) % &p;

        // Convert results back to [u8; 32]
        let xr_bytes = xr.to_be_bytes();
        let yr_bytes = yr.to_be_bytes();

        let mut result = [0u8; 64];
        result[32 - xr_bytes.len()..32].copy_from_slice(&xr_bytes);
        result[64 - yr_bytes.len()..].copy_from_slice(&yr_bytes);

        UncompressedPoint(result)
    }
}

impl Add<CompressedPoint> for UncompressedPoint {
    type Output = UncompressedPoint;

    fn add(self, rhs: CompressedPoint) -> Self::Output {
        self.add(rhs.decompress())
    }
}

impl Debug for UncompressedPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl UncompressedPoint {
    pub fn to_sec1_bytes(&self) -> [u8; 65] {
        [
            SEC1_OCTET_UNCOMPRESSED,
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
            self.0[16],
            self.0[17],
            self.0[18],
            self.0[19],
            self.0[20],
            self.0[21],
            self.0[22],
            self.0[23],
            self.0[24],
            self.0[25],
            self.0[26],
            self.0[27],
            self.0[28],
            self.0[29],
            self.0[30],
            self.0[31],
            self.0[32],
            self.0[33],
            self.0[34],
            self.0[35],
            self.0[36],
            self.0[37],
            self.0[38],
            self.0[39],
            self.0[40],
            self.0[41],
            self.0[42],
            self.0[43],
            self.0[44],
            self.0[45],
            self.0[46],
            self.0[47],
            self.0[48],
            self.0[49],
            self.0[50],
            self.0[51],
            self.0[52],
            self.0[53],
            self.0[54],
            self.0[55],
            self.0[56],
            self.0[57],
            self.0[58],
            self.0[59],
            self.0[60],
            self.0[61],
            self.0[62],
            self.0[63],
        ]
    }
}

impl TryFrom<CompressedPoint> for UncompressedPoint {
    type Error = Secp256k1Error;

    fn try_from(x: CompressedPoint) -> Result<Self, Secp256k1Error> {
        let mut point = UncompressedPoint::lift_x(&x.x())?;
        if point.is_odd() != x.is_odd() {
            point.invert();
        }
        Ok(point)
    }
}

impl From<[u8; 65]> for UncompressedPoint {
    fn from(p: [u8; 65]) -> Self {
        let mut s = [0u8; 64];
        s.clone_from_slice(&p[1..]);
        UncompressedPoint(s)
    }
}

impl TryFrom<[u8; 32]> for UncompressedPoint {
    type Error = Secp256k1Error;

    fn try_from(scalar: [u8; 32]) -> Result<Self, Self::Error> {
        Curve::mul_g(&scalar)
    }
}

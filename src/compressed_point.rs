use dashu::integer::UBig;
use solana_nostd_secp256k1_recover::secp256k1_recover;

use crate::*;
use core::{fmt::{Debug, Formatter}, ops::{Add, Mul}};

pub const SEC1_OCTET_COMPRESSED_EVEN: u8 = 0x02;
pub const SEC1_OCTET_COMPRESSED_ODD: u8 = 0x03;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CompressedPoint(pub [u8; Self::SIZE]);

impl Secp256k1Point for CompressedPoint {
    const SIZE: usize = 33;

    fn is_odd(&self) -> bool {
        self.0[0] == SEC1_OCTET_COMPRESSED_ODD
    }

    fn is_even(&self) -> bool {
        self.0[0] == SEC1_OCTET_COMPRESSED_EVEN
    }

    fn x(&self) -> [u8; 32] {
        [
            self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7], self.0[8],
            self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15],
            self.0[16], self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22],
            self.0[23], self.0[24], self.0[25], self.0[26], self.0[27], self.0[28], self.0[29],
            self.0[30], self.0[31], self.0[32],
        ]
    }

    fn y(&self) -> [u8; 32] {
        // Raise X to uncompressed point first
        let mut p = UncompressedPoint::lift_x_unchecked(&self.x());
        // If resulting Y-coordinate polarity doesn't match, invert it.
        if p.is_even() != self.is_even() {
            p.invert()
        }
        p.y()
    }

    fn lift_x(x: &[u8; 32]) -> Result<Self, Secp256k1Error> {
        Ok(UncompressedPoint::lift_x(x)?.into())
    }

    fn lift_x_unchecked(x: &[u8; 32]) -> Self {
        UncompressedPoint::lift_x_unchecked(x).into()
    }

    fn invert(&mut self) {
        self.0[0] = self.is_even() as u8 + 2;
    }
    
    fn compress(&self) -> CompressedPoint {
        *self
    }
    
    fn decompress(&self) -> UncompressedPoint {
        Curve::decompress_unchecked(*self)
    }

    fn tweak(&self, tweak: [u8; 32]) -> Result<Self, Secp256k1Error> {
        // Compute z = (-r * k) mod N
        let z_scalar = ((UBig::from_be_bytes(&Curve::negate_n(&self.x())) * UBig::from_be_bytes(&tweak)) % UBig::from_be_bytes(&Curve::N)).to_be_bytes();
               
        // Ensure z and s are 32 bytes
        let mut z = [0u8; 32];
        z[32 - z_scalar.len()..].copy_from_slice(&z_scalar);

        let s: [u8; 64] = [
            self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7], self.0[8], 
            self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15], self.0[16], 
            self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22], self.0[23], self.0[24], 
            self.0[25], self.0[26], self.0[27], self.0[28], self.0[29], self.0[30], self.0[31], self.0[32],
            self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7], self.0[8], 
            self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15], self.0[16], 
            self.0[17], self.0[18], self.0[19], self.0[20], self.0[21], self.0[22], self.0[23], self.0[24], 
            self.0[25], self.0[26], self.0[27], self.0[28], self.0[29], self.0[30], self.0[31], self.0[32],
        ];

        // Use ecrecover with negated z to perform ECAdd
        Ok(UncompressedPoint(secp256k1_recover(&z, self.is_odd(), &s)?).into())
    }
}

impl Mul<&[u8;32]> for CompressedPoint {
    type Output = Result<UncompressedPoint, Secp256k1Error>;

    fn mul(self, k: &[u8;32]) -> Self::Output {
        Curve::ecmul(&self, k)
    }
}

impl Add<UncompressedPoint> for CompressedPoint {
    type Output = UncompressedPoint;

    fn add(self, point: UncompressedPoint) -> Self::Output {
        point.add(self)
    }
}

impl Add for CompressedPoint {
    type Output = UncompressedPoint;

    fn add(self, point: Self) -> Self::Output {
        let p = point.decompress();
        p.add(point)
    }
}

impl Debug for CompressedPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl From<UncompressedPoint> for CompressedPoint {
    fn from(p: UncompressedPoint) -> Self {
        CompressedPoint([
            p.is_odd() as u8 + 2,
            p.0[0],
            p.0[1],
            p.0[2],
            p.0[3],
            p.0[4],
            p.0[5],
            p.0[6],
            p.0[7],
            p.0[8],
            p.0[9],
            p.0[10],
            p.0[11],
            p.0[12],
            p.0[13],
            p.0[14],
            p.0[15],
            p.0[16],
            p.0[17],
            p.0[18],
            p.0[19],
            p.0[20],
            p.0[21],
            p.0[22],
            p.0[23],
            p.0[24],
            p.0[25],
            p.0[26],
            p.0[27],
            p.0[28],
            p.0[29],
            p.0[30],
            p.0[31],
        ])
    }
}

impl From<[u8; 65]> for CompressedPoint {
    fn from(p: [u8; 65]) -> Self {
        UncompressedPoint::from(p).into()
    }
}

impl TryFrom<[u8; 32]> for CompressedPoint {
    type Error = Secp256k1Error;

    fn try_from(scalar: [u8; 32]) -> Result<Self, Secp256k1Error> {
        Ok(UncompressedPoint::try_from(scalar)?.into())
    }
}

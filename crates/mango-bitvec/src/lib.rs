// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

//! This library defines a BitVec struct that represents a bit vector.

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::ops::BitAnd;

// Every u8 is used as a bucket of 8 bits. Total max buckets = 256 / 8 = 32.
const BUCKET_SIZE: usize = 8;
const MAX_BUCKETS: usize = 32;

/// BitVec represents a bit vector that supports 4 operations:
/// 1. Marking a position as set.
/// 2. Checking if a position is set.
/// 3. Count set bits.
/// 4. Get the index of the last set bit.
/// Internally, it stores a vector of u8's (as Vec<u8>).
/// * The first 8 positions of the bit vector are encoded in the first element of the vector, the
///   next 8 are encoded in the second element, and so on.
/// * Bits are read from left to right. For instance, in the following bitvec
///   [0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001], the 3rd and 31st positions are set.
/// * Each bit of a u8 is set to 1 if the position is set and to 0 if it's not.
/// * We only allow setting positions upto u8::MAX. As a result, the size of the inner vector is
///   limited to 32 (= 256 / 8).
/// * Once a bit has been set, it cannot be unset. As a result, the inner vector cannot shrink.
/// * The positions can be set in any order.
/// * A position can set more than once -- it remains set after the first time.
///
/// # Examples:
/// ```
/// use mango_bitvec::BitVec;
/// let mut bv = BitVec::default();
/// bv.set(2);
/// bv.set(5);
/// assert!(bv.is_set(2));
/// assert!(bv.is_set(5));
/// assert_eq!(false, bv.is_set(0));
/// assert_eq!(bv.count_ones(), 2);
/// assert_eq!(bv.last_set_bit(), Some(5));
///
/// // A bitwise AND of BitVec can be performed by using the `&` operator.
/// let mut bv1 = BitVec::default();
/// bv1.set(2);
/// bv1.set(3);
/// let mut bv2 = BitVec::default();
/// bv2.set(2);
/// let intersection = bv1 & bv2;
/// assert!(intersection.is_set(2));
/// assert_eq!(false, intersection.is_set(3));
/// ```
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize)]
pub struct BitVec {
    #[serde(with = "serde_bytes")]
    inner: Vec<u8>,
}

impl BitVec {
    // TODO(abhayb): Remove after migration to new wire format.
    #[allow(dead_code)]
    /// Sets the bit at position @pos.
    pub fn set(&mut self, pos: u8) {
        // This is optimised to: let bucket = pos >> 3;
        let bucket: usize = pos as usize / BUCKET_SIZE;
        if self.inner.len() <= bucket {
            self.inner.resize(bucket + 1, 0);
        }
        // This is optimized to: let bucket_pos = pos | 0x07;
        let bucket_pos = pos as usize - (bucket * BUCKET_SIZE);
        self.inner[bucket] |= 0b1000_0000 >> bucket_pos as u8;
    }

    // TODO(abhayb): Remove after migration to new wire format.
    #[allow(dead_code)]
    /// Checks if the bit at position @pos is set.
    pub fn is_set(&self, pos: u8) -> bool {
        // This is optimised to: let bucket = pos >> 3;
        let bucket: usize = pos as usize / BUCKET_SIZE;
        if self.inner.len() <= bucket {
            return false;
        }
        // This is optimized to: let bucket_pos = pos | 0x07;
        let bucket_pos = pos as usize - (bucket * BUCKET_SIZE);
        (self.inner[bucket] & (0b1000_0000 >> bucket_pos as u8)) != 0
    }

    // TODO(kostas): Remove after applying it to multi-sig.
    #[allow(dead_code)]
    /// Returns the number of set bits.
    pub fn count_ones(&self) -> u32 {
        self.inner.iter().map(|a| a.count_ones()).sum()
    }

    // TODO(kostas): Remove after applying it to multi-sig.
    #[allow(dead_code)]
    /// Returns the index of the last set bit.
    pub fn last_set_bit(&self) -> Option<u8> {
        self.inner
            .iter()
            .rev()
            .enumerate()
            .find(|(_, byte)| byte != &&0u8)
            .map(|(i, byte)| {
                (8 * (self.inner.len() - i) - byte.trailing_zeros() as usize - 1) as u8
            })
    }
}

impl BitAnd for BitVec {
    type Output = BitVec;

    /// Returns a new BitVec that is a bitwise AND of two BitVecs.
    fn bitand(self, other: Self) -> Self {
        let len = std::cmp::min(self.inner.len(), other.inner.len());
        let mut ret = BitVec {
            inner: Vec::with_capacity(len),
        };
        for i in 0..len {
            ret.inner.push(self.inner[i] & other.inner[i]);
        }
        ret
    }
}

// We impl custom deserialization to ensure that the length of inner vector does not exceed
// 32 (= 256 / 8).
impl<'de> Deserialize<'de> for BitVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = serde_bytes::ByteBuf::deserialize(deserializer)?.into_vec();
        if v.len() > MAX_BUCKETS {
            return Err(D::Error::custom(format!("BitVec too long: {}", v.len())));
        }
        Ok(BitVec { inner: v })
    }
}

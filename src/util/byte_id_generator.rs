//! This is a small set of utility features that can be used to map a number to a slice of bytes

use num_traits::cast::ToPrimitive;
use std::iter::{Iterator, IntoIterator};

//================================================================================
// Stateless Functions
//================================================================================

/// Get the byte encoded id from some index into the encoding.
/// This returns and Err when the idx is larger than can possibly be encoded in "byte_count".
pub fn get_id(idx: u64, byte_count: u32) -> Result<Vec<u8>, &'static str> {
    if idx > (u64::pow(2, byte_count*8) - 1)  as u64 {
        Err("Tried to get a byte id larger than allowed bytes would allow")
    } else {
        // Convert the idx to a byte array and then convert the required number of bytes to vec
        let bytes = idx.to_be_bytes();
        Ok((&bytes[(8-byte_count as usize)..8]).to_vec())
    }
}

/// Gets the byte encoded id from some index into the encoding.
/// This does not check that the index passed in is valid.
///
/// # Safety
/// Technically this shouldn't cause any kind of safety issues but it does mean
/// that the result will be incorrect.
#[inline]
pub unsafe fn get_id_unchecked(idx: u64, byte_count: u32) -> Vec<u8> {
    // Convert the idx to a byte array and then convert the required number of bytes to vec
    let bytes: [u8; 8] = idx.to_be_bytes();
    (&bytes[(8-byte_count as usize)..8]).to_vec()
}

//================================================================================
// Stateful Generator
//================================================================================

/// Generates the byte ids from a given index and can be built to use a specified number of bytes
/// or to only generate up to a maximum index.
/// This is essentially a wrapper around converting some larger unsigned int e.g 0x345678 into
/// a collection of its bytes: [0x34, 0x56, 0x78] whilst providing safety to ensure an id is not
/// requested than cannot be stored in the desired number of bytes.
/// It also brings convenience in the form of iterators.
///
/// # Usage
/// This is used to map symbols in an [Alphabet](crate::alphabet::Alphabet) to a unique and valid unicode
/// representation.
/// For example [A, C, T, G] may become [0x0, 0x1, 0x2, 0x3].
/// This reduces memory usage for sequences whilst simplifying a lot of operations on them.
pub struct ByteIdGenerator {
    /// The maximum index that will be required
    max: u64,
    /// The number of bytes used for the encoding
    num_bytes: u32,
}

impl ByteIdGenerator {
    /// Create a new UnicodeIdGenerator that can return a valid unicode mapping for numbers from
    /// 0 to max_idx inclusive.
    pub fn from_max(max_idx: u64) -> ByteIdGenerator {
        // This was the only formula I could think of that didn't have rounding issues
        // problem being it won't work when max_idx is u64::MAX because of overflow.
        let num_bytes = if max_idx != std::u64::MAX {
            (f64::log2((max_idx + 1) as f64).ceil() / 8.0).ceil() as u32
        } else {
            8
        };

        ByteIdGenerator { max: max_idx, num_bytes }
    }

    /// Create a new UnicodeIdGenerator that can return a valid unicode mapping for numbers from 0
    /// up to (2^(n*8) - 1) where n is the number of bytes specified.
    /// # Note
    /// The value of num_bytes should be reasonable such that the max value ((2^(n*8) - 1)) will fit
    /// in a u64 integer. Basically as long as num_bytes <= 8 you will be fine.
    pub fn from_byte_count(num_bytes: u32) -> ByteIdGenerator {
        ByteIdGenerator {
            max: u64::pow(2, num_bytes*8) - 1,
            num_bytes
        }
    }

    /// Get the maximum index this generator will generate an id for
    #[inline]
    pub fn max(&self) -> u64 {
        self.max
    }

    /// Get the number of bytes this generator will encode with
    #[inline]
    pub fn num_bytes(&self) -> u32 {
        self.num_bytes
    }

    /// Similar to a stateful call of the [get_id()] function (uses local max bound instead).
    /// Get the byte encoded id from some index into the encoding.
    /// This returns and Err when the idx is larger than the max of the generator.
    pub fn get_id(&self, idx: u64) -> Result<Vec<u8>, &'static str> {
        if idx > self.max {
            Err("Tried to get a byte id larger than max generator size")
        } else {
            unsafe { Ok(get_id_unchecked(idx, self.num_bytes)) }
        }
    }

    /// Stateful call of the [get_id_unchecked()] function.
    /// Get the byte encoded id from some index into the encoding.
    /// This does not check that the idx passed in is valid.
    ///
    /// # Safety
    /// Technically this shouldn't cause any kind of safety issues but it does mean
    /// that the result will be incorrect.
    #[inline]
    pub unsafe fn get_id_unchecked(&self, idx: u64) -> Vec<u8> {
        get_id_unchecked(idx, self.num_bytes)
    }
}

//================================================================================
// Iteration
//================================================================================

/// An iterator over a ByteIdGenerator.
/// Goes from 0 to the maximum value of the generator.
pub struct ByteIdIter<'a> {
    index: u64,
    generator: &'a ByteIdGenerator,
}

impl<'a> Iterator for ByteIdIter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.index > self.generator.max() {
            return None;
        }

        let res = unsafe {
            Some(self.generator.get_id_unchecked(self.index))
        };
        self.index += 1;
        res

    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.generator.max - self.index).to_usize();

        if let Some(size) = size {
            return (size, Some(size));
        }

        (std::usize::MAX, None)
    }
}

impl<'a> IntoIterator for &'a ByteIdGenerator {
    type Item = Vec<u8>;
    type IntoIter = ByteIdIter<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        ByteIdIter {
            index: 0,
            generator: self
        }
    }
}

//================================================================================
// Tests
//================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that the from_max function calculates the correct minimum number of bytes to use
    #[test]
    fn from_max_byte_count() {
        let a = ByteIdGenerator::from_max(100);
        let b = ByteIdGenerator::from_max((std::u8::MAX as u64) + 1);
        let c = ByteIdGenerator::from_max(0x10000);
        let d = ByteIdGenerator::from_max(std::u64::MAX - 500);
        let e = ByteIdGenerator::from_max(std::u64::MAX);

        assert_eq!(a.num_bytes(), 1);
        assert_eq!(b.num_bytes(), 2);
        assert_eq!(c.num_bytes(), 3);
        assert_eq!(d.num_bytes(), 8);
        assert_eq!(e.num_bytes(), 8);
    }

    /// Ensures that the correct bytes are returned
    #[test]
    fn byte_result() {
        let a = ByteIdGenerator::from_byte_count(3);
        assert_eq!(a.get_id(33).unwrap(),      vec![0x00, 0x00, 0x21], "For 33");
        assert_eq!(a.get_id(256).unwrap(),     vec![0x00, 0x01, 0x00], "For 256");
        assert_eq!(a.get_id(0xC1045).unwrap(), vec![0x0C, 0x10, 0x45], "For 0xC1045");

        let a = ByteIdGenerator::from_byte_count(2);
        assert_eq!(a.get_id(33).unwrap(),      vec![0x00, 0x21], "For 33");
        assert_eq!(a.get_id(256).unwrap(),     vec![0x01, 0x00], "For 256");
    }

    /// Ensures that get_id() will panic when expected
    #[test]
    fn bad_idx() {
        let a = ByteIdGenerator::from_byte_count(2);

        match a.get_id(0x10000) {
            Ok(_) => panic!("get_id(0x10000) worked for two byte generator"),
            Err(_) => ()
        }

        let a = ByteIdGenerator::from_max(345);

        match a.get_id(346) {
            Ok(_) => panic!("get_id(346) worked for byte generator with max 345"),
            Err(_) => ()
        }
    }

    /// Test iteration from a single byte construction
    #[test]
    fn iteration_byte() {
        let a = ByteIdGenerator::from_byte_count(1);

        let mut expected: u8 = 0;
        for i in &a {
            assert_eq!(i, vec![expected]);
            if expected < std::u8::MAX {
                expected += 1;
            }
        }
    }

    /// Tests iteration from a multiple byte construction
    #[test]
    fn iteration_bytes() {
        let a = ByteIdGenerator::from_byte_count(2);

        let mut expected_low: u8 = 0;
        let mut expected_high: u8 = 0;
        for i in &a {
            assert_eq!(i, vec![expected_high, expected_low]);

            if expected_low < std::u8::MAX {
                expected_low += 1;
                continue;
            } else {
                expected_low = 0;
                if expected_high < std::u8::MAX {
                    expected_high += 1;
                }
            }
        }
    }

    /// Test iteration from a max construction
    #[test]
    fn iteration_max() {
        let a = ByteIdGenerator::from_max(32);

        let mut expected: u8 = 0;
        for i in &a {
            assert_eq!(i, vec![expected]);
            expected += 1;
        }
    }
}
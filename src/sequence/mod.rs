//! Contains types that relate to storing a sequence.
//! A sequence is constructed using an [Alphabet](crate::alphabet::Alphabet) of symbols.

pub use crate::alphabet::{Alphabet, Complement};

use std::marker::PhantomData;
use crate::alphabet::encoding::{AlphabetEncoder, index_encoder::AsciiIndexEncoder};
use crate::alphabet::encoding::{self, EncodingError};
use std::fmt;

/// A Sequence contains a string constructed from the symbols of the specified
/// [Alphabet](crate::alphabet::Alphabet).
/// Typically this will represent a DNA, RNA or protein sequence.
///
/// # Example
/// ```
/// use biors::alphabet::UnambiguousDnaAlphabet;
/// use biors::sequence::Sequence;
///
/// let alphabet = UnambiguousDnaAlphabet;
/// let mut seq = Sequence::new(&alphabet);
///
/// seq.push("ATTCGGC");
/// ```
///
/// # Defaults
/// By default the Sequence tries to use an [AsciiIndexEncoder] for the alphabet (this should work
/// in the vast majority of cases)
pub struct Sequence<'a, A, E=AsciiIndexEncoder<'a, A>>
where
    A: Alphabet,
    E: AlphabetEncoder<A>
{
    encoder: E,
    /// Determines whether the sequence is circular or not
    pub circular: bool,
    string: Vec<u8>,
    phantom: PhantomData<&'a A>
}

impl<'a, A, E> Sequence<'a, A, E>
where
    A: Alphabet,
    E: AlphabetEncoder<A>
{
    /// Constructs a new sequence using a custom encoder.
    /// The initial Sequence will be empty.
    pub fn from_encoder(encoder: E) -> Self {
        Sequence {
            encoder,
            string: vec![],
            circular: false,
            phantom: PhantomData
        }
    }

    /// Convenience function to set the sequence to be circular during creation.
    /// # Example
    /// ```
    /// use biors::alphabet::UnambiguousDnaAlphabet;
    /// use biors::sequence::Sequence;
    ///
    /// let alphabet = UnambiguousDnaAlphabet;
    /// let seq = Sequence::new(&alphabet).circular(true);
    /// assert!(seq.circular)
    /// ```
    pub fn circular(mut self, circ: bool) -> Self {
        self.circular = circ;
        self
    }

    /// Get a reference to the alphabet that the encoder associated with this Sequence uses.
    pub fn alphabet(&self) -> &A {
        self.encoder.alphabet()
    }

    /// Push a string to the sequence.
    ///
    /// Uses [Alphabet::symbol_size()](crate::alphabet::Alphabet::symbol_size) in order to
    /// determine how to separate the input into the constituent symbols. The first symbol is
    /// assumed to begin with the first character of the input.
    pub fn push<S: AsRef<str>>(&mut self, seq: S) -> encoding::Result<()> {
        let seq = seq.as_ref();
        let symbol_size = self.alphabet().symbol_size();

        // If the number of characters in the string doesn't match the size of the symbols
        if seq.chars().count() % symbol_size != 0 {
            let description = format!(
                "Tried to push sequence with {} characters which is not a multiple of the \
                alphabet's symbol size {}", seq.len(), symbol_size);
            return Err(EncodingError::new(encoding::ErrorKind::InvalidLength, description));
        }

        let split = string_chunks(seq, symbol_size);
        self.string.extend_from_slice(&self.encoder.encode_all(split)?);

        Ok(())
    }

    /// Push a string to the sequence without checking if its length is valid.
    ///
    /// Uses [Alphabet::symbol_size()](crate::alphabet::Alphabet::symbol_size) in order to
    /// determine how to separate the input into the constituent symbols. The first symbol is
    /// assumed to begin with the first character of the input.
    ///
    /// # Notes
    /// This does the same thing as [push()](Sequence::push) except it doesn't check that the sequence
    /// length is a multiple of the alphabets symbol_size.
    /// It is still perfectly safe to use and this just means that any extra characters on the
    /// end will be ignored.
    ///
    /// Benchmarks seem to imply that there is little difference between the two methods but it may
    /// be worth a shot if push is really slow for you for some reason or you simply don't care
    /// about/want the assurance of push.
    pub fn push_unchecked<S: AsRef<str>>(&mut self, seq: S) -> encoding::Result<()> {
        let seq = seq.as_ref();
        let symbol_size = self.alphabet().symbol_size();

        let split = string_chunks(seq, symbol_size);
        self.string.extend_from_slice(&self.encoder.encode_all(split)?);

        Ok(())
    }

    /// Clears the underlying string Vector thus emptying the Sequence.
    pub fn clear(&mut self) {
        self.string.clear();
    }
}

//================================================================================
// Construct using alphabet with AsciiIndexEncoder
//================================================================================

impl<'a, A: Alphabet> Sequence<'a, A, AsciiIndexEncoder<'a, A>> {
    /// Construct a new Sequence from some alphabet using the default AsciiIndexEncoder.
    /// The initial Sequence will be empty.
    pub fn new(alphabet: &'a A) -> Self{
        Sequence {
            encoder: AsciiIndexEncoder::new(alphabet),
            string: vec![],
            circular: false,
            phantom: PhantomData
        }
    }
}

//================================================================================
// Display
//================================================================================

impl<'a, A: Alphabet> fmt::Display for Sequence<'a, A, AsciiIndexEncoder<'a, A>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Decode min(string.len(), 50) characters to their respective symbols
        let num_chars = 50.min(self.string.len());
        let seq_vec = match self.encoder.decode_all(&self.string[0..num_chars]) {
            Err(err) => panic!("Unable to display sequence.\n{}", err),
            Ok(v) => v
        };

        let seq_str = seq_vec.concat();
        write!(f, "Sequence: {}", seq_str)
    }
}

//================================================================================
// Extend
//================================================================================

impl<'a, 'b, A, E> Extend<&'b str> for Sequence<'a, A, E>
where
    A: Alphabet,
    E: AlphabetEncoder<A>
{
    /// Extends a sequence using an iterator of strings.
    ///
    /// # Panics
    /// If any of the strings causes an encoding error this will panic.
    /// If you want to do this without the panic use [push()](Sequence::push)
    fn extend<I: IntoIterator<Item=&'b str>>(&mut self, iter: I) {
        for string in iter {
            match self.push(string) {
                Ok(_) => continue,
                Err(err) => {
                    panic!("Error while trying to extend sequence:\n{}", err);
                }
            }
        }
    }
}

//================================================================================
// Complement
//================================================================================

// Give Sequences that use an alphabet with complement symbols the ability to complement themselves
impl<'a, A: Complement> Sequence<'a, A> {
    pub fn complement(&mut self) {
        unimplemented!()
    }
}

//================================================================================
// Utility Functions
//================================================================================

/// Takes a string and creates an iterator over chunks of chunk_size of that string.
/// All chunks will be exactly chunk_size, any excess in the string will not be included.
/// Works with utf-8 strings.
fn string_chunks(src: &str, chunk_size: usize) -> impl Iterator<Item=&str> {
    src.char_indices()
        .step_by(chunk_size)
        .flat_map(move |(from, _)| {
            src[from..].char_indices()
                .skip(chunk_size - 1)
                .next()
                .map(|(to, c)| {
                    &src[from .. from + to + c.len_utf8()]
                })
        })
}

//================================================================================
// Tests
//================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::encoding;

    struct TestAlphabet;

    impl TestAlphabet {
        const SYMBOLS: [&'static str; 4] = ["AA", "TT", "GG", "CC"];
    }

    impl Alphabet for TestAlphabet {
        #[inline]
        fn symbols(&self) -> &[&str] {
            &TestAlphabet::SYMBOLS
        }

        #[inline]
        fn symbol_size(&self) -> usize { 2 }
    }

    struct TestEncoder<'a, A: Alphabet> {
        alphabet: &'a A,
    }

    impl<'a, A: Alphabet> TestEncoder<'a, A> {
        fn new(alphabet: &'a A) -> Self {
            TestEncoder { alphabet }
        }
    }

    impl<'a, A: Alphabet> AlphabetEncoder<A> for TestEncoder<'a, A> {
        fn encode(&self, _symbol: &str) -> encoding::Result<Vec<u8>> {
            unimplemented!()
        }

        fn decode_all(&self, _symbols: &[u8]) -> encoding::Result<Vec<&str>> {
            unimplemented!()
        }

        fn alphabet(&self) -> &A {
            self.alphabet
        }
    }

    /// Test that the from_encoder method works throw in a sneaky circular to test as well
    #[test]
    fn from_encoder() {
        let a = TestAlphabet;
        let e = TestEncoder::new(&a);

        let s = Sequence::from_encoder(e).circular(true);
        let _a2 = s.alphabet();
    }

    /// Tests the string_chunks function for perfectly sized input
    #[test]
    fn string_chunks_proper_size() {
        let str1 = "ABCDEF";
        let str3 = "AAABBBCCCDDDEEEFFF";
        let str_unicode = "ɑɑBɓɔɔAA";

        let exp1        = vec!["A", "B", "C", "D", "E", "F"];
        let exp3        = vec!["AAA", "BBB", "CCC", "DDD", "EEE", "FFF"];
        let exp_unicode = vec!["ɑɑ", "Bɓ", "ɔɔ", "AA"];

        let res1: Vec<_>        = string_chunks(str1,        1).collect();
        let res3: Vec<_>        = string_chunks(str3,        3).collect();
        let res_unicode: Vec<_> = string_chunks(str_unicode, 2).collect();

        assert_eq!(res1,        exp1);
        assert_eq!(res3,        exp3);
        assert_eq!(res_unicode, exp_unicode);
    }

    /// Tests that the string_chunks function when there are extra characters at the end of input
    #[test]
    fn string_chunks_wrong_size() {
        let str1 = "AABBC";
        let str2 = "ɑɑBɓɔ";

        let exp1 = vec!["AA", "BB"];
        let exp2 = vec!["ɑɑ", "Bɓ"];

        let res1: Vec<_> = string_chunks(str1, 2).collect();
        let res2: Vec<_> = string_chunks(str2, 2).collect();

        assert_eq!(res1, exp1);
        assert_eq!(res2, exp2);
    }

    /// Asserts that the push method on sequence works for exact expected input
    #[test]
    fn push_string() {
        let a = TestAlphabet;
        let mut s = Sequence::new(&a);

        s.push("AATTCCGGCCAA").unwrap();
        let encoding = vec![0, 1, 3, 2, 3, 0];

        assert_eq!(s.string, encoding)
    }

    /// Asserts that hte push method on sequence works for input with extra characters at the end
    #[test]
    fn push_string_extra_characters() {
        let a = TestAlphabet;
        let mut s = Sequence::new(&a);

        match s.push("AATTCCG") {
            Ok(_) => panic!("push allowed badly sized string"),
            Err(err) => {
                assert_eq!(err.kind(), &crate::alphabet::encoding::ErrorKind::InvalidLength);
            }
        };

        s.clear();
        s.push_unchecked("AATTCCG").unwrap();
        let encoding = vec![0, 1, 3];

        assert_eq!(s.string, encoding);
    }

    /// Assert AsciiIndexEncoded sequences display correctly
    #[test]
    fn display() {
        let a = TestAlphabet;
        let mut s = Sequence::new(&a);

        let seq = ["AA"]
            .iter()
            .cycle()
            .take(60)
            .map(|s| *s);

        s.extend(seq);

        let display_res = format!("{}", s);
        let bytes: Vec<u8> = "A".bytes().cycle().take(100).collect();
        let exp_string = std::str::from_utf8(bytes.as_slice()).unwrap();
        let expected = format!("Sequence: {}", exp_string);

        assert_eq!(display_res, expected);
    }

}
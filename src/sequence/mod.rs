//! Contains types that relate to storing a sequence.
//! A sequence is constructed using an [Alphabet](crate::alphabet::Alphabet) of symbols.

pub use crate::alphabet::{Alphabet, Complement};
use crate::alphabet::encoding::{AlphabetEncoder, index_encoder::AsciiIndexEncoder};
use std::marker::PhantomData;

/// A Sequence contains a string constructed from the symbols of the specified
/// [Alphabet](crate::alphabet::Alphabet).
/// Typically this will represent a DNA, RNA or protein sequence.
///
/// # Defaults
/// By default the Sequence tries to use an [AsciiIndexEncoder] for the alphabet (this should work
/// in the vast majority of cases)
pub struct Sequence<'a, A, E=AsciiIndexEncoder<'a, A>>
where
    A: Alphabet,
    E: AlphabetEncoder<A>
{
    alphabet: A,
    encoder: E,
    string: Vec<u8>,
    phantom: PhantomData<&'a A>
}

impl<'a, A, E> Sequence<'a, A, E>
where
    A: Alphabet,
    E: AlphabetEncoder<A>
{
    fn from_encoder(alphabet: A, encoder: E) -> Self {
        Sequence {
            alphabet,
            encoder,
            string: vec![],
            phantom: PhantomData
        }
    }
}

impl<'a, A: Alphabet> Sequence<'a, A, AsciiIndexEncoder<'a, A>> {
    fn new(alphabet: A) -> Self{
        Sequence {
            alphabet,
            encoder: AsciiIndexEncoder::new(&alphabet),
            string: vec![],
            phantom: PhantomData
        }
    }
}

// Give Sequences that use an alphabet with complement symbols the ability to complement themselves
impl<'a, A: Complement> Sequence<'a, A> {
    pub fn complement(&mut self) {
        unimplemented!()
    }
}

//================================================================================
// Tests
//================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::encoding;
    use crate::alphabet::dna::UnambiguousDnaAlphabet;

    struct TestEncoder<'a, A: Alphabet> {
        alphabet: &'a A,
    }

    impl<'a, A: Alphabet> TestEncoder<'a, A> {
        fn new(alphabet: &'a A) -> Self {
            TestEncoder { alphabet }
        }
    }

    impl<'a, A: Alphabet> AlphabetEncoder<A> for TestEncoder<'a, A> {
        fn encode(&self, symbol: &str) -> encoding::Result<Vec<u8>> {
            unimplemented!()
        }

        fn decode_all(&self, symbols: &[u8]) -> encoding::Result<Vec<&str>> {
            unimplemented!()
        }

        fn alphabet(&self) -> &A {
            self.alphabet
        }
    }

    #[test]
    fn from_encoder() {
        let a = UnambiguousDnaAlphabet;
        let e = TestEncoder::new(&a);

        let s = Sequence::from_encoder(a, e);
    }

}
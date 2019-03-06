//! A simple encoder that works for [Alphabets](crate::alphabet::Alphabet) that have a static
//! number of symbols.

pub use super::AlphabetEncoder;

use crate::alphabet::Alphabet;
use bimap::{BiHashMap, Overwritten};
use super::byte_id_generator::ByteIdGenerator;

/// An index encoder takes each symbol of an alphabet and encodes it based on its index in the slice
/// returned from the [Alphabet::symbols()](super::Alphabet::symbols) method.
///
/// # Notes
/// This is the default encoder and should work fine in most cases.
///
/// It makes use of the [Alphabet::max_alphabet_size()](super::Alphabet::max_alphabet_size) method
/// to determine the number of bytes that will be needed to encode all symbols.
///
/// If you expect the number of symbols in your [Alphabet](super::Alphabet) will not be static
/// you may have to create your own encoder.
pub struct IndexEncoder<'a, A: Alphabet> {
    alphabet: &'a A,
    mapping: BiHashMap<&'a str, Vec<u8>>,
}

impl<'a, A: Alphabet> IndexEncoder<'a, A> {
    /// Construct a new [IndexEncoder] from a given alphabet.
    pub fn new(alphabet: &'a A) -> IndexEncoder<'a, A> {
        let mapping = IndexEncoder::construct_mapping(alphabet);

        IndexEncoder { alphabet, mapping }
    }

    /// Recalculates the mapping. Must be called if the Alphabet is ever altered such that a symbol
    /// changes or the number of symbols changes.
    pub fn recalculate_mapping(&mut self) {
        self.mapping = IndexEncoder::construct_mapping(self.alphabet);
    }

    /// Constructs the mapping from symbols in the alphabet to bytes
    fn construct_mapping(alphabet: &A) -> BiHashMap<&str, Vec<u8>> {
        let max_size = alphabet.max_alphabet_size() as u64;
        let generator = ByteIdGenerator::from_max(max_size);

        let mut mapping = BiHashMap::with_capacity(alphabet.symbols().len());

        for (index, symbol) in alphabet.symbols().iter().enumerate() {
            let id = generator
                .get_id(index as u64)
                .expect("Alphabet size inconsistency");

            if mapping.insert(*symbol, id) != Overwritten::Neither {
                panic!("Alphabet with symbols {:?} contains duplicate symbol.", alphabet.symbols());
            }
        }

        mapping
    }
}

impl<'a, A: Alphabet + 'a> AlphabetEncoder<A> for IndexEncoder<'a, A> {
    fn encode(&self, symbol: &str) -> Vec<u8> {
        let res = self.mapping.get_by_left(&symbol);

        // Check if a mapping was found if not determine the error and panic! with useful message
        if let Some(encoded) = res {
            return encoded.to_owned();
        } else {
            let mut error_message = String::from("IndexEncoder failed to encode symbol.");

            // If the symbol is not in the alphabet:
            if !self.alphabet.contains(symbol) {
                let extra = format!(
                    " The input to encode() was a symbol which does not exist in the alphabet: {}",
                    symbol
                );

                error_message.push_str(&extra);
            } else {
                error_message.push_str(
                    " Did you alter the alphabet and forget to call recalculate_mapping()?"
                )
            }

            panic!(error_message);
        };
    }

    fn decode(&self, symbol: &[u8]) -> &str {
        self.mapping
            .get_by_right(&symbol.to_vec())
            .expect("IndexEncoder failed to decode symbol. \
                     Did you alter the alphabet and forget to call recalculate_mapping()?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test alphabet that has the symbols defined so that it is easy to identify what
    /// their mappings will be.
    struct TestAlphabet;

    impl TestAlphabet {
        // The encodings for these will be    0     1     2
        const SYMBOLS: [&'static str; 3] = ["AA", "BB", "CC"];
    }

    impl Alphabet for TestAlphabet {
        #[inline]
        fn symbols(&self) -> &[&str] {
            &TestAlphabet::SYMBOLS
        }

        #[inline]
        fn symbol_size(&self) -> usize {
            2
        }
    }

    #[test]
    fn correct_encode() {
        let a = TestAlphabet;
        let encoder = IndexEncoder::new(&a);

        let seq   = vec!["AA", "BB", "CC", "AA", "BB"];
        let encoded= vec![0, 1, 2, 0, 1];

        assert_eq!(encoder.encode_all(&seq), encoded);
    }

    #[test]
    #[should_panic(expected="IndexEncoder failed to encode symbol. The input to encode() was a symbol which does not exist in the alphabet: A")]
    fn unexpected_symbol() {
        let a = TestAlphabet;
        let encoder = IndexEncoder::new(&a);

        let seq = vec!["AA", "BB", "A"];
        encoder.encode_all(&seq);
    }

    #[test]
    fn recalculate_mappings() {

    }
}
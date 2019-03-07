//! A simple encoder that works for [Alphabets](crate::alphabet::Alphabet) that have a static
//! number of symbols.

pub use super::AlphabetEncoder;

use crate::alphabet::Alphabet;
use crate::alphabet::encoding::{EncodingError, ErrorKind};
use super::EncodingResult;
use bimap::{BiHashMap, Overwritten};

/// An index encoder takes each symbol of an alphabet and encodes it based on its index in the slice
/// returned from the [Alphabet::symbols()](super::Alphabet::symbols) method. The Ascii part of the
/// name derives from the fact that it always uses one byte and can thus only map 256 different symbols.
///
/// # Notes
/// This is the default encoder and should work fine in most cases.
///
/// It makes use of the [Alphabet::max_alphabet_size()](super::Alphabet::max_alphabet_size) method
/// to determine if it can be safely used.
///
/// If you expect the number of symbols in your [Alphabet](super::Alphabet) will not be static
/// you may have to create your own encoder or you can try using the
/// [recalculate_mapping()](AsciiIndexEncoder::recalculate_mapping) method.
pub struct AsciiIndexEncoder<'a, A: Alphabet> {
    alphabet: &'a A,
    mapping: BiHashMap<&'a str, u8>,
}

impl<'a, A: Alphabet> AsciiIndexEncoder<'a, A> {
    /// Construct a new [IndexEncoder] from a given alphabet.
    pub fn new(alphabet: &'a A) -> AsciiIndexEncoder<'a, A> {
        let mapping = AsciiIndexEncoder::construct_mapping(alphabet);

        AsciiIndexEncoder { alphabet, mapping }
    }

    /// Recalculates the mapping. Must be called if the Alphabet is ever altered such that a symbol
    /// changes or the number of symbols changes.
    pub fn recalculate_mapping(&mut self) {
        self.mapping = AsciiIndexEncoder::construct_mapping(self.alphabet);
    }

    /// Constructs the mapping from symbols in the alphabet to bytes
    fn construct_mapping(alphabet: &A) -> BiHashMap<&str, u8> {
        let max_size = alphabet.max_alphabet_size() as u64;
        let num_symbols = alphabet.symbols().len();
        if max_size > 256 || alphabet.symbols().len() > 256 {
            panic!(
                "This alphabet expects to/has more symbols than can the AsciiIndexEncoder is \
                capable of mapping. Try using UnicodeIndexEncoder instead."
            )
        }

        let mut mapping = BiHashMap::with_capacity(num_symbols);

        for (index, symbol) in alphabet.symbols().iter().enumerate() {
            if mapping.insert(*symbol, index as u8) != Overwritten::Neither {
                panic!("Alphabet with symbols {:?} contains duplicate symbol.", alphabet.symbols());
            }
        }

        mapping
    }
}

impl<'a, A: Alphabet> AlphabetEncoder<A> for AsciiIndexEncoder<'a, A> {
    fn encode(&self, symbol: &str) -> EncodingResult<Vec<u8>> {
        let res = self.mapping.get_by_left(&symbol);

        // Check if a mapping was found if not determine the error and panic! with useful message
        if let Some(encoded) = res {
            Ok(vec![*encoded])
        } else {
            let mut error_message
                = String::from("AsciiIndexEncoder failed to encode symbol. ");

            // If the symbol is not in the alphabet:
            let kind = if !self.alphabet.contains(symbol) {
                let extra = format!(
                    "The input to encode() was a symbol which does not exist in the alphabet: {}",
                    symbol
                );

                error_message.push_str(&extra);

                ErrorKind::InvalidSymbol(symbol.to_owned())
            } else { // Symbol is in the Alphabet but wasn't when the mapping was constructed
                error_message.push_str(
                    "Did you alter the alphabet and forget to call recalculate_mapping()?"
                );

                ErrorKind::NoMapping
            };

            Err(EncodingError::new(kind, error_message))
        }
    }

    fn decode(&self, symbol: &[u8]) -> EncodingResult<&str> {
        if symbol.len() != 1 {
            let kind = ErrorKind::InvalidBytes(symbol.to_vec());
            let desc = "AsciiIndexEncoder received more than one byte to decode.";
            return Err(EncodingError::new(kind, desc.to_owned()));
        };

        let res = self.mapping.get_by_right(&symbol[0]);

        if let Some(decoded) = res {
            Ok(decoded)
        } else {
            let kind = ErrorKind::NoMapping;
            let desc = "AsciiIndexEncoder failed to decode symbol. \
            Did you alter the alphabet and forget to call recalculate_mapping()?";
            Err(EncodingError::new(kind, desc.to_owned()))
        }
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
        let encoder = AsciiIndexEncoder::new(&a);

        let seq   = vec!["AA", "BB", "CC", "AA", "BB"];
        let encoded= vec![0, 1, 2, 0, 1];

        assert_eq!(encoder.encode_all(&seq).unwrap(), encoded);
    }

    #[test]
    fn unexpected_symbol() {
        let a = TestAlphabet;
        let encoder = AsciiIndexEncoder::new(&a);

        let seq = vec!["AA", "BB", "A"];
        let res = encoder.encode_all(&seq);

        match res {
            Ok(_) => panic!("Encoding worked when there was an invalid symbol"),
            Err(err) => assert_eq!(*err.kind(), ErrorKind::InvalidSymbol("A".to_owned())),
        }
    }

    #[test]
    fn recalculate_mappings() {

    }
}
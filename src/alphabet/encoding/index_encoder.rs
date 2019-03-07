//! Simple encoders that use the index of the symbol from the call to
//! [Alphabet::symbols()](crate::alphabet::Alphabet::symbols) to encode the symbol.

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
/// If you expect the number of symbols in your [Alphabet](super::Alphabet) will not be static you
/// should look at the [recalculate_mapping()](AsciiIndexEncoder::recalculate_mapping) method.
#[derive(Debug)]
pub struct AsciiIndexEncoder<'a, A: Alphabet> {
    alphabet: &'a A,
    mapping: BiHashMap<&'a str, u8>,
}

impl<'a, A: Alphabet> AsciiIndexEncoder<'a, A> {
    /// Construct a new [AsciiIndexEncoder] from a given alphabet.
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

    fn decode_all(&self, symbols: &[u8]) -> EncodingResult<Vec<&str>> {
        let mut decoded = Vec::with_capacity(symbols.len() / self.size_hint());

        for byte in symbols {
            let next_symbol = self.mapping.get_by_right(byte);

            match next_symbol {
                Some(symbol) => decoded.push(*symbol),
                None => {
                    let kind = ErrorKind::NoMapping;
                    let desc
                        = "AsciiIndexEncoder failed to decode symbol. Did you alter the size of the \
                        alphabet and forget to call recalculate_mapping()?";
                    return Err(EncodingError::new(kind, desc.to_owned()))
                }
            };
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// A simple test alphabet that has the symbols defined so that it is easy to identify what
    /// their mappings will be.
    struct TestAlphabet {
        changed_symbols: RefCell<bool>,
    }

    impl TestAlphabet {
        // Will map to                          0     1      2
        const SYMBOLS_A: [&'static str; 3] = ["AA", "BB", "CC"];
        // Will map to                          0     1      2    3
        const SYMBOLS_B: [&'static str; 4] = ["EE", "BB", "CC", "DD"];

        fn change_symbols(&self) {
            *self.changed_symbols.borrow_mut() = true;
        }
    }

    impl Default for TestAlphabet {
        fn default() -> Self {
            TestAlphabet {
                changed_symbols: RefCell::new(false)
            }
        }
    }

    impl Alphabet for TestAlphabet {
        #[inline]
        fn symbols(&self) -> &[&str] {
            if !*self.changed_symbols.borrow() {
                &TestAlphabet::SYMBOLS_A
            } else {
                &TestAlphabet::SYMBOLS_B
            }
        }

        #[inline]
        fn symbol_size(&self) -> usize {
            2
        }
    }

    /// Tests to make sure that that the AsciiIndexEncoder correctly encodes the symbols
    #[test]
    fn correct_encode() {
        let a = TestAlphabet::default();
        let encoder = AsciiIndexEncoder::new(&a);

        let seq   = vec!["AA", "BB", "CC", "AA", "BB"];
        let encoded= vec![0, 1, 2, 0, 1];

        assert_eq!(encoder.encode_all(&seq).unwrap(), encoded);
    }

    /// Tests that the AsciiIndexEncoder can correctly decode a sequence
    #[test]
    fn correct_decode() {
        let a = TestAlphabet::default();
        let encoder = AsciiIndexEncoder::new(&a);

        let encoded = vec![0, 1, 2, 0, 1];
        let seq    = vec!["AA", "BB", "CC", "AA", "BB"];

        assert_eq!(encoder.decode(&[0]).unwrap(), "AA");
        assert_eq!(encoder.decode_all(&encoded).unwrap(), seq);

    }

    /// Tests the AsciiIndexEncoder's response to an invalid symbol while encoding
    #[test]
    fn unexpected_symbol() {
        let a = TestAlphabet::default();
        let encoder = AsciiIndexEncoder::new(&a);

        let seq = vec!["AA", "BB", "A"];
        let res = encoder.encode_all(&seq);

        match res {
            Ok(_) => panic!("Encoding worked when there was an invalid symbol"),
            Err(err) => assert_eq!(*err.kind(), ErrorKind::InvalidSymbol("A".to_owned())),
        };
    }

    /// Tests that decode will correctly error with more than one symbol
    #[test]
    fn decode_too_many() {
        let a = TestAlphabet::default();
        let encoder = AsciiIndexEncoder::new(&a);

        let bytes = vec![0, 1];
        match encoder.decode(&bytes) {
            Ok(_) => panic!("decode() worked with 2 bytes in AsciiIndexEncoder"),
            Err(err) => assert_eq!(*err.kind(), ErrorKind::InvalidBytes(bytes)),
        }
    }

    /// Tests to make sure that the AsciiIndexEncoder correctly recalculates it's mappings
    #[test]
    fn recalculate_mappings() {
        let a = TestAlphabet::default();
        let mut encoder = AsciiIndexEncoder::new(&a);

        a.change_symbols();
        encoder.recalculate_mapping();

        let seq    = vec!["EE", "CC", "BB", "DD"];
        let encoded = vec![0, 2, 1, 3];

        assert_eq!(encoder.encode_all(&seq).unwrap(), encoded);
    }
}
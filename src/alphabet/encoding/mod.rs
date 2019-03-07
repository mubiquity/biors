//! An encoding takes the symbols of an alphabet and transforms them in some meaningful way
//! in order to increase efficiency and reduce memory usage.
//!
pub mod index_encoder;

pub use super::Alphabet;

pub trait AlphabetEncoder<A: Alphabet> {
    /// Takes in a symbol from the [Alphabet](super::Alphabet) A and turns it into a vector of bytes
    fn encode(&self, symbol: &str) -> Vec<u8>;

    /// Reverses [encode()](AlphabetEncoder::encode())
    fn decode(&self, symbol: &[u8]) -> &str;

    /// Takes a slice of strings and encodes them all using [encode()](AlphabetEncoder::encode()).
    /// Returns a flattened vec of the encoded strings.
    fn encode_all(&self, symbols: &[&str]) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(symbols.len());

        for symbol in symbols {
            let encode = self.encode(*symbol);
            encoded.extend_from_slice(encode.as_slice());
        }

        encoded
    }
}
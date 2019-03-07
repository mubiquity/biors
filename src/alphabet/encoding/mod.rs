//! An encoding takes the symbols of an alphabet and transforms them in some meaningful way
//! in order to increase efficiency and reduce memory usage.
//!
pub mod index_encoder;

pub use super::Alphabet;
use std::error::Error;
use std::fmt;

pub type EncodingResult<T> = Result<T, EncodingError>;

pub trait AlphabetEncoder<A: Alphabet> {
    /// Takes in a symbol from the [Alphabet](super::Alphabet) A and turns it into a vector of bytes
    fn encode(&self, symbol: &str) -> EncodingResult<Vec<u8>>;

    /// Reverses [encode()](AlphabetEncoder::encode())
    fn decode(&self, symbol: &[u8]) -> EncodingResult<&str>;

    /// How many bytes you expect an encoded symbol to take on average.
    /// Does not have to be exact and is purely for extra efficiency in memory allocation.
    ///
    /// # Default
    /// Defaults to 1 byte per encoded symbol.
    #[inline]
    fn size_hint(&self) -> usize {
        1
    }

    /// Takes a slice of strings and encodes them all using [encode()](AlphabetEncoder::encode()).
    /// Returns a flattened vec of the encoded strings on success.
    fn encode_all(&self, symbols: &[&str]) -> EncodingResult<Vec<u8>> {
        // We make the guess that each symbol will take one byte when encoded
        let mut encoded = Vec::with_capacity(symbols.len() * self.size_hint());

        for symbol in symbols {
            let encode = self.encode(*symbol)?;
            encoded.extend_from_slice(encode.as_slice());
        }

        Ok(encoded)
    }
}

/// Represents the kind of error that occurred while encoding or decoding an alphabet symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// The symbol passed to [encode()](AlphabetEncoder::encode) was not in the Alphabets dictionary
    /// or the symbol does not have a mapping in that encoder for some reason.
    InvalidSymbol(String),

    /// The bytes passed to [decode()](AlphabetEncoder::decode) were invalid.
    InvalidBytes(Vec<u8>),

    /// The encoder has no way to map your symbol/bytes. Potentially because the alphabet has
    /// changed. Each encoder will have different ways of handling this and different reasons as to
    /// why it might occur.
    NoMapping,

    /// Some other error occurred (check description/display)
    Other
}

#[derive(Debug, Clone)]
pub struct EncodingError {
    kind: ErrorKind,
    description: String,
}

impl EncodingError {
    /// Construct a new EncodingError from the given ErrorKind and description
    pub fn new(kind: ErrorKind, description: String) -> EncodingError {
        EncodingError { kind, description }
    }

    /// Get the associated ErrorKind for this error
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Get the associated description for this error
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Error for EncodingError {}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encoding error occurred with description:\n\t{}", self.description)
    }
}
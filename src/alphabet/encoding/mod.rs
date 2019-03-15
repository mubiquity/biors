//! An encoding takes the symbols of an alphabet and transforms them in some meaningful way
//! in order to increase efficiency and reduce memory usage.
//!
pub mod index_encoder;

pub use super::Alphabet;
use std::error::Error;
use std::fmt;

/// The type of Results returned from methods that encode or decode an alphabet's symbols.
pub type Result<T> = std::result::Result<T, EncodingError>;

/// Represents a type that can map the symbols in an alphabet to and from valid UTF-8 bytes.
pub trait AlphabetEncoder<A: Alphabet> {
    /// Takes in a symbol from the [Alphabet](super::Alphabet) A and turns it into a vector of bytes
    ///
    /// # Requires
    /// The output bytes MUST be valid UTF-8.
    /// This restriction allows implementation of a variety of efficient string searching algorithms
    /// in a manner that isn't encoder dependant.
    fn encode(&self, symbol: &str) -> Result<Vec<u8>>;

    /// The opposite of [encode_all()](AlphabetEncoder::encode_all). This takes in some bytes that
    /// can be decoded into a collection of alphabet symbols.
    ///
    /// # Notes
    /// Because there is no requirement that each symbol maps to the same number of bytes
    /// it is not possible to create a default implemented decode_all method.
    fn decode_all(&self, symbols: &[u8]) -> Result<Vec<&str>>;

    /// Return a reference to the underlying [Alphabet](super::Alphabet)
    fn alphabet(&self) -> &A;

    /// How many bytes you expect an encoded symbol to take on average.
    /// Does not have to be exact and is purely for extra efficiency in memory allocation.
    ///
    /// # Default
    /// Defaults to 1 byte per encoded symbol.
    #[inline]
    fn size_hint(&self) -> usize {
        1
    }

    /// Decodes a single symbol. Reverses [encode()](AlphabetEncoder::encode).
    fn decode(&self, symbol: &[u8]) -> Result<&str> {
        let decoded = self.decode_all(symbol)?;

        if decoded.len() == 1 {
            Ok(decoded[0])
        } else {
            let kind = ErrorKind::InvalidBytes(symbol.to_vec());
            let description = "Call to decode tried to decode multiple symbols. \
            Use decode_all() instead.";
            Err(EncodingError::new(kind, description.to_owned()))
        }
    }

    /// Takes am iterator of strings and encodes them all using
    /// [encode()](AlphabetEncoder::encode).
    /// Returns a flattened vec of the encoded strings on success.
    fn encode_all<'a, I>(&self, symbols: I) -> Result<Vec<u8>>
    where I: IntoIterator<Item = &'a str>
    {
        let iter = symbols.into_iter();

        // Use size_hint to estimate how much space will be needed to store the result
        let mut encoded = match iter.size_hint() {
            (_, Some(amt)) => Vec::with_capacity(amt * self.size_hint()),
            _ => Vec::with_capacity(self.size_hint()) // Probably room for 1 symbol at least
        };

        for symbol in iter {
            let encode = self.encode(symbol)?;
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

    /// The symbols could not be split because the number of characters was not a multiple of the
    /// [Alphabet::symbol_size()](super::Alphabet::symbol_size).
    InvalidLength,

    /// The encoder has no way to map your symbol/bytes. Potentially because the alphabet has
    /// changed. Each encoder will have different ways of handling this and different reasons as to
    /// why it might occur.
    NoMapping,


    /// Some other error occurred (check description/display)
    Other
}

/// The type of error returned whenever something goes wrong while trying to encode or decode
/// with an [AlphabetEncoder]
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
        write!(f, "Encoding error: {:?}:\n\t{}", self.kind, self.description)
    }
}
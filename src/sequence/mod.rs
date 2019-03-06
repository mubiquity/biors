//! Contains types that relate to storing a sequence.
//! A sequence is constructed using an [Alphabet](crate::alphabet::Alphabet) of symbols.

pub use crate::alphabet::{Alphabet, Complement};

/// A Sequence contains a string constructed from the symbols of the specified
/// [Alphabet](crate::alphabet::Alphabet).
/// Typically this will represent a DNA, RNA or protein sequence.
pub struct Sequence<A: Alphabet> {
    alphabet: A,
    string: Vec<u8>,
}

// Give Sequences that use an alphabet with complement symbols the ability to complement themselves
impl<T: Alphabet + Complement> Sequence<T> {
    pub fn complement(&mut self) {
        unimplemented!()
    }
}
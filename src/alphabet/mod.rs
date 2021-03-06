//! An alphabet is a set of symbols that can be used to construct a sequence.
//! They provide useful utility and compile time convenience when working with sequences.
//! This module provides the traits that most alphabets will implement and the submodules contain
//! pre-implemented common alphabets.

use std::collections::HashMap;

pub use self::dna::{UnambiguousDnaAlphabet, AmbiguousDnaAlphabet};

pub mod encoding;
pub mod dna;

// TODO: Need to decide how I want to handle case sensitivity for now everything is case sensitive
// TODO: Need to get the documentation links to work

/// The alphabet trait is implemented for any type that can be used to construct a sequence.
pub trait Alphabet {
    /// Returns slice containing each valid symbol in the alphabet
    ///
    /// # Requires
    /// Each symbol must only occur once in the returned slice
    ///
    /// # Notes
    /// Symbols are considered to be case sensitive
    fn symbols(&self) -> &[&str];

    /// The size of each symbol in [Alphabet::symbols()].
    ///
    /// # Requires
    /// The size must be greater than 0
    ///
    /// # Default
    /// The default symbol size is 1
    #[inline]
    fn symbol_size(&self) -> usize {
        1
    }

    /// Returns the maximum number of symbols that the Alphabet will ever contain over its entire
    /// lifetime.
    ///
    /// # Default
    /// The default max alphabet size is 256. This allows the assumption that all symbols in the
    /// alphabet can be encoded using a single byte.
    #[inline]
    fn max_alphabet_size(&self) -> usize { 256 }

    /// Returns true if the alphabet contains the symbol "s" else false
    #[inline]
    fn contains<T: AsRef<str>>(&self, s: T) -> bool {
        self.symbols().contains(&s.as_ref())
    }

    /// Checks to see if the Alphabet can construct a sequence of symbols
    #[inline]
    fn is_word<T: AsRef<str>>(&self, word: &[T]) -> bool {
        word.iter().all(|symbol| self.contains(symbol))
    }
}

/// The complement trait is implemented for any [Alphabet](self::Alphabet) that has a mapping from
/// one symbol to another.
pub trait Complement: Alphabet {
    /// Returns a slice of strings where string at position i corresponds to the complement of
    /// the symbol from [self::Alphabet::symbols()] at position i.
    /// The mapping does not need to be one to one.
    ///
    /// # Requires
    /// The length of the returned slice is equal to the length of the slice returned from
    /// [Alphabet::symbols()] and contains only valid symbols from the alphabet.
    /// If these restraints are not met then any calls to the methods from this trait are invalid.
    fn complement_mapping(&self) -> &[&str];

    /// Mutates a slice of strings such that each element becomes its complement as defined in
    /// the [Complement::complement_mapping()] method.
    ///
    /// # Panics
    /// If the [Complement::complement_mapping()] method does not meet the required invariant.
    fn complement<T: AsRef<str>>(&self, input: &[T]) -> Vec<&str> {
        let symbols = self.symbols();
        let complement = self.complement_mapping();

        // Construct a mapping
        // This is not the most efficient way to do it but it is simple and fool proof
        // If optimisation is needed at a later stage it will be done then
        let mapping: HashMap<&&str, &&str> = symbols.iter()
            .zip(complement.iter())
            .collect();

        input.iter().map(|s| *mapping[&s.as_ref()]).collect()
    }
}

//================================================================================
// Tests
//================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Alphabet that can be used to test the default implementations
    struct TestAlphabet;

    impl TestAlphabet {
        const SYMBOLS:    [&'static str; 3] = ["AA", "BB", "CC"];
        const COMPLEMENT: [&'static str; 3] = ["CC", "AA", "BB"];
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

    impl Complement for TestAlphabet {
        #[inline]
        fn complement_mapping(&self) -> &[&str] {
            &TestAlphabet::COMPLEMENT
        }
    }

    /// Test that ensures [Alphabet::contains()] returns true when it should
    #[test]
    fn contains_true() {
        let a = TestAlphabet;

        assert!(a.contains("AA"));
        assert!(a.contains("BB"));
        assert!(a.contains("CC"));
    }

    /// Test that ensures [Alphabet::contains()] returns false when it should
    #[test]
    fn contains_false() {
        let a = TestAlphabet;

        assert!(!a.contains("B"));
        assert!(!a.contains("D"));
    }

    /// Tests that the is_word method is working as expected
    #[test]
    fn is_word() {
        let a = TestAlphabet;

        assert!( a.is_word(&["AA", "BB", "AA", "CC"]));
        assert!(!a.is_word(&["AA", "BB", "AA","C"]))
    }

    /// Tests that the complement method works as expected when the invariant is met
    #[test]
    fn complement_valid() {
        let a = TestAlphabet;

        let seq      = ["AA", "BB", "CC", "CC", "BB", "AA", "AA"];
        let seq_comp = ["CC", "AA", "BB", "BB", "AA", "CC", "CC"];

        assert_eq!(seq_comp, a.complement(&seq).as_slice());
    }
}
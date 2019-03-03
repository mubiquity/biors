//! An alphabet is a set of symbols that can be used to construct a sequence.
//! They provide useful utility and compile time convenience when working with sequences.
//! This module provides the traits that most alphabets will implement and the submodules contain
//! pre-implemented common alphabets.

use std::collections::HashMap;

pub mod dna;

// TODO: Need to decide how I want to handle case sensitivity for now everything is case sensitive
// TODO: Need to get the documentation links to work

/// The alphabet trait is implemented for any type that can be used to construct a sequence
pub trait Alphabet {
    /// Returns slice containing each valid symbol in the alphabet
    ///
    /// # Requires
    /// Each symbol must only occur once in the returned slice
    ///
    /// # Notes
    /// Symbols are considered to be case sensitive
    fn symbols(&self) -> &[&str];

    /// If each symbol in symbols has the same size then this returns that size otherwise None.
    ///
    /// # Notes
    /// <ls>
    /// <li>It cannot be assumed the size will be constant and thus the default implementation
    /// recalculates it each call.</li>
    /// <li>If the symbol size is constant then consider explicitly defining size.</li>
    /// <li>If [`Alphabet::symbols()`] is empty then the size will be 0</li>
    /// </ls>
    fn size(&self) -> Option<usize> {
        // Get the minimum symbol length
        let symbols = self.symbols();
        let min = symbols.iter().min_by_key(|s| s.len());

        // If min is None then the vector is empty
        if min.is_none() {
            return Some(0);
        }

        // Get the maximum symbol length
        let max = symbols.iter().max_by_key(|s| s.len()).unwrap().len();
        let min = min.unwrap().len();

        if min == max {
            return Some(min);
        }

        None
    }

    /// Returns true if the alphabet contains the symbol "s" else false
    fn contains<T: AsRef<str>>(&self, s: T) -> bool {
        self.symbols().contains(&s.as_ref())
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

    /// What the size of the constructed alphabet should be
    enum TestSize {
        Zero,
        One,
        Two,
        None
    }

    impl TestSize {
        fn value(&self) -> Option<usize> {
            match *self {
                TestSize::Zero => Some(0),
                TestSize::One  => Some(1),
                TestSize::Two  => Some(2),
                TestSize::None => None,
            }
        }
    }

    /// Alphabet that can be used to test the default implementations
    struct TestAlphabet {
        size: TestSize,
        symbols: Vec<&'static str>,
        complement: Vec<&'static str>,
    }

    impl TestAlphabet {
        // Short and simple function to change the properties of the alphabet for testing
        fn new(size: TestSize) -> TestAlphabet {
            let (symbols, complement) = match size {
                TestSize::Zero    => (vec![], vec![]),
                TestSize::One     => (vec!["A", "B", "C"], vec!["C", "A", "B"]),
                TestSize::Two     => (vec!["AA", "BB", "CC"], vec!["CC", "AA", "BB"]),
                TestSize::None    => (vec!["A", "BB", "CC"], vec!["CC", "A", "BB"]),
            };

            TestAlphabet { size, symbols, complement }
        }

    }

    impl Alphabet for TestAlphabet {
        fn symbols(&self) -> &[&str] {
            &self.symbols
        }
    }

    impl Complement for TestAlphabet {
        fn complement_mapping(&self) -> &[&str] {
            &self.complement
        }
    }

    /// A collection of each type of alphabet that is more convenient to use
    struct TestAlphabetCollection(TestAlphabet, TestAlphabet, TestAlphabet, TestAlphabet);

    impl TestAlphabetCollection {
        fn new() -> TestAlphabetCollection {
            TestAlphabetCollection (
                TestAlphabet::new(TestSize::Zero),
                TestAlphabet::new(TestSize::One),
                TestAlphabet::new(TestSize::Two),
                TestAlphabet::new(TestSize::None),
            )
        }

        /// Runs a function on each TestAlphabet in order to check for panics
        fn test_all<F: Fn(&TestAlphabet)>(&self, f: F) {
            f(&self.0);
            f(&self.1);
            f(&self.2);
            f(&self.3);
        }
    }

    /// Test that ensures the default implementation of [Alphabet::size()] works as intended
    #[test]
    fn correct_size() {
        let alphabets = TestAlphabetCollection::new();

        // Goes through each TestAlphabet and checks that the size function returns expected value
        alphabets.test_all(|a| assert_eq!(a.size(), a.size.value()));
    }

    /// Test that ensures [Alphabet::contains()] returns true when it should
    #[test]
    fn contains_true() {
        let none = TestAlphabet::new(TestSize::None);

        assert!(none.contains("A"));
        assert!(none.contains("BB"));
        assert!(none.contains("CC"));
    }

    /// Test that ensures [Alphabet::contains()] returns false when it should
    #[test]
    fn contains_false() {
        let none = TestAlphabet::new(TestSize::None);

        assert!(!none.contains("B"));
        assert!(!none.contains("D"));
    }

    /// Tests that the complement method works as expected when the invariant is met
    #[test]
    fn complement_valid() {
        let a1 = TestAlphabet::new(TestSize::One);
        let a2 = TestAlphabet::new(TestSize::None);

        let seq1 = ["A", "B", "C", "C", "A", "B", "B"];
        let seq1_comp = ["C", "A", "B", "B", "C", "A", "A"];

        let seq2 = ["A", "BB", "CC", "CC", "BB", "A", "A"];
        let seq2_comp = ["CC", "A", "BB", "BB", "A", "CC", "CC"];

        assert_eq!(seq1_comp, a1.complement(&seq1).as_slice());
        assert_eq!(seq2_comp, a2.complement(&seq2).as_slice());
    }
}
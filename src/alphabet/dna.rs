//! Defines multiple DNA alphabets for varying common situations

pub use super::{Alphabet, Complement};
use std::fmt;

/// An alphabet that contains the symbols ACTG
/// # Symbol Meaning
/// <table>
///   <tr>
///     <th>Symbol</th>
///     <th>Meaning</th>
///     <th>Complement</th>
///   </tr>
///   <tr>
///     <td>A</td>
///     <td>Adenine</td>
///     <td>T</td>
///   </tr>
///   <tr>
///     <td>C</td>
///     <td>Cytosine</td>
///     <td>G</td>
///   </tr>
///   <tr>
///     <td>T</td>
///     <td>Thymine</td>
///     <td>A</td>
///   </tr>
///   <tr>
///     <td>G</td>
///     <td>Guanine</td>
///     <td>C</td>
///   </tr>
/// </table>
pub struct UnambiguousDnaAlphabet;

impl UnambiguousDnaAlphabet {
    const SYMBOLS:    [&'static str; 4] = ["A", "C", "T", "G"];
    const COMPLEMENT: [&'static str; 4] = ["T", "G", "A", "C"];
}

impl Alphabet for UnambiguousDnaAlphabet {
    #[inline]
    fn symbols(&self) -> &[&str] {
        &UnambiguousDnaAlphabet::SYMBOLS
    }

    #[inline]
    fn size(&self) -> Option<usize> {
        Some(1)
    }
}

impl Complement for UnambiguousDnaAlphabet {
    #[inline]
    fn complement_mapping(&self) -> &[&str] {
        &UnambiguousDnaAlphabet::COMPLEMENT
    }
}

impl fmt::Display for UnambiguousDnaAlphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unambiguous DNA Alphabet containing symbols: {:?}", self.symbols())
    }
}

/// An alphabet that contains the symbols AGCTYRWSKMDVHBN
/// # Symbol Meaning
/// <table>
///   <tr>
///     <th>Symbol</th>
///     <th>Meaning</th>
///     <th>Complement</th>
///   </tr>
///   <tr>
///     <td>A</td>
///     <td>Adenine</td>
///     <td>T</td>
///   </tr>
///   <tr>
///     <td>G</td>
///     <td>Guanine</td>
///     <td>C</td>
///   </tr>
///   <tr>
///     <td>C</td>
///     <td>Cytosine</td>
///     <td>G</td>
///   </tr>
///   <tr>
///     <td>T</td>
///     <td>Thymine</td>
///     <td>A</td>
///   </tr>
///   <tr>
///     <td>Y</td>
///     <td>Pyrimidine (C or T)</td>
///     <td>R</td>
///   </tr>
///   <tr>
///     <td>R</td>
///     <td>Purine (A or G)</td>
///     <td>Y</td>
///   </tr>
///   <tr>
///     <td>W</td>
///     <td>weak (A or T)</td>
///     <td>W</td>
///   </tr>
///   <tr>
///     <td>S</td>
///     <td>strong (G or C)</td>
///     <td>S</td>
///   </tr>
///   <tr>
///     <td>K</td>
///     <td>keto (T or G)</td>
///     <td>M</td>
///   </tr>
///   <tr>
///     <td>M</td>
///     <td>amino (C or A)</td>
///     <td>K</td>
///   </tr>
///   <tr>
///     <td>D</td>
///     <td>A, G, T (not C)</td>
///     <td>H</td>
///   </tr>
///   <tr>
///     <td>V</td>
///     <td>A, C, G (not T)</td>
///     <td>B</td>
///   </tr>
///   <tr>
///     <td>H</td>
///     <td>A, C, T (not G)</td>
///     <td>D</td>
///   </tr>
///   <tr>
///     <td>B</td>
///     <td>C, G, T (not A)</td>
///     <td>V</td>
///   </tr>
///   <tr>
///     <td>N</td>
///     <td>Any base</td>
///     <td>N</td>
///   </tr>
/// </table>
pub struct AmbiguousDnaAlphabet;

impl AmbiguousDnaAlphabet {
    const SYMBOLS: [&'static str; 15]
                    = ["A", "G", "C", "T", "Y", "R", "W", "S", "K", "M", "D", "V", "H", "B", "N"];

    const COMPLEMENT: [&'static str; 15]
                    = ["T", "C", "G", "A", "R", "Y", "W", "S", "M", "K", "H", "B", "D", "V", "N"];
}

impl Alphabet for AmbiguousDnaAlphabet {
    #[inline]
    fn symbols(&self) -> &[&str] {
        &AmbiguousDnaAlphabet::SYMBOLS
    }

    #[inline]
    fn size(&self) -> Option<usize> {
        Some(1)
    }
}

impl Complement for AmbiguousDnaAlphabet {
    #[inline]
    fn complement_mapping(&self) -> &[&str] {
        &AmbiguousDnaAlphabet::COMPLEMENT
    }
}

impl fmt::Display for AmbiguousDnaAlphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ambiguous DNA Alphabet containing symbols: {:?}", self.symbols())
    }
}

// TODO: Some tests could be made automatic/macroised for all Alphabet implementers
#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that UnambiguousDnaAlphabet returns the correct symbols
    #[test]
    fn unambiguous_symbols() {
        let a = UnambiguousDnaAlphabet;
        assert_eq!(a.symbols(), ["A", "C", "T", "G"])
    }

    /// Ensures that AmbiguousDnaAlphabet returns the correct symbols
    #[test]
    fn ambiguous_symbols() {
        let a = AmbiguousDnaAlphabet;
        assert_eq!(a.symbols(),
                   ["A", "G", "C", "T", "Y", "R", "W", "S", "K", "M", "D", "V", "H", "B", "N"])
    }

    /// Ensures that there are the same number of symbols as there are complements
    #[test]
    fn symbol_complement_size() {
        let a1 = AmbiguousDnaAlphabet;
        let a2 = UnambiguousDnaAlphabet;

        assert_eq!(a1.complement_mapping().len(), a1.symbols().len());
        assert_eq!(a2.complement_mapping().len(), a2.symbols().len());
    }

    /// Ensures that all symbols in the complement are also in the alphabet
    #[test]
    fn complements_in_alphabet() {
        let a1 = AmbiguousDnaAlphabet;
        let a2 = UnambiguousDnaAlphabet;

        for symbol in a1.complement_mapping() {
            assert!(a1.contains(symbol));
        }

        for symbol in a2.complement_mapping() {
            assert!(a2.contains(symbol));
        }
    }

    /// Ensures that the complement function works correctly for UnambiguousDnaAlphabet
    #[test]
    fn unambiguous_complement() {
        let a = UnambiguousDnaAlphabet;

        let seq = ["A", "C", "T", "G", "G", "C", "A", "T"];
        let comp = ["T", "G", "A", "C", "C", "G", "T", "A"];

        assert_eq!(comp, a.complement(&seq).as_slice());
    }

    /// Ensures that the complement function works correctly for AmbiguousDnaAlphabet
    #[test]
    fn ambiguous_complement() {
        let a = AmbiguousDnaAlphabet;

        let seq = ["Y", "H", "K", "R", "T", "V", "B", "A", "D", "G", "W", "N", "S", "M", "C"];
        let comp = ["R", "D", "M", "Y", "A", "B", "V", "T", "H", "C", "W", "N", "S", "K", "G"];

        assert_eq!(comp, a.complement(&seq).as_slice());
    }
}
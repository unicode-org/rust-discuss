//! # The ECMA 402 API surface proposal for rust
//!
//! This proposal contains traits declarations for a rust-flavored [ECMA402
//! implementation](https://www.ecma-international.org/publications/standards/Ecma-402.htm).
//!
//! All ECMA402 functions require specifying at least one, if not multiple locales
//! for which they will then return appropriate results. For this reason, this proposal shows a
//! minimal type signature required for something like that to be feasible.
//!
//! The idea of this proposal is to define common API surface that would admit different
//! implementations of ECMA 402 inspired libraries.  The existence of a common standard would allow
//! drop-in replacement of say [ICU-based]() implementation for
//! [Unic-based](https://crates.io/crates/unic) implementation at some future, which is relevant
//! for users that need the Unicode support functionality today, and are not prepared to wait until
//! Unic conquers the rust world.
//!
//! See [LanguageIdentifier] for an example of such a trait.
//!
//! ## A note about presentation
//!
//! This proposal is deliberately written in the form of compilable rust code, and is perhaps best
//! consumed by looking at the output of the command `cargo doc --open` ran at the top level
//! directory.  It's not quite [literate
//! programming](https://en.wikipedia.org/wiki/Literate_programming) but should be close enough for
//! our purpose.  And our purpose here is to present the API alongside a glimpse of how it would be
//! used.
//!
//! The proposed APIs are quickly tested with implementations given in the `mod tests` section of
//! the source code.  I originally put those into doc-tests, but doc-tests were difficult to write
//! efficiently so I rolled them into a separate test module, as is customary in rust.
//!
//! # Part 1: Language identifiers and BCP 47 representation
//!
//! This proposal contains the following traits:
//!
//! * [AsBCP47]: A single-method trait for converting an object into a BCP 47 serialized form.
//!   This is a minimum required to be able to define ECMA402 compatible APIs, which take arrays
//!   of locales and friends.
//! * [LanguageIdentifier]: Adds immutable getters for language identifier components.

/// Represents an immutable language identifier.
///
/// This trait can be passed into functions that are not expected to be able to mutate the
/// identifier.  The `language` property must be defined, or equal to the literal string `und` if
/// it is left unspecified.  Other properties are optional.  See [weird::Variants] for the
/// obviously missing treatment of variants subtags.
pub trait LanguageIdentifier {
    /// Returns the language subtag of the `language::Identifier`.  If the
    /// language subtag is empty, the returned value is `und`.
    fn language(&self) -> &str;

    /// Returns the region subtag of the `language::Identifier`, if one is set.
    fn region(&self) -> Option<&str>;

    /// Returns the script subtag of the `language::Identifier`, if one is set.
    fn script(&self) -> Option<&str>;
}

/// Allows representing the item (a locale object or a language identifier) in the form compatible
/// with the [BCP 47 representation](https://tools.ietf.org/html/bcp47).
pub trait AsBCP47 {
    /// Returns a BCP 47 representation of the object.  This represents a canonical serialization
    /// of all properties of a language identifier or a locale into a string.  Some objects, like
    /// full-blown locales have extensions that are required to be serialized in a very specific
    /// way.  Follow BCP 47 practices to do so when implementing this trait.
    fn as_bcp47(&self) -> &str;
}

/// Traits that ended up being unusual or weird because of issues unrelated to their structure.
/// Specifically [weird::Variants] departs from what it should have been because of issues with
/// defining a lifetime of an iterator.
pub mod weird {

    /// Allows access to variants.  Variants are guaranteed to be valid.
    ///
    /// What I had wanted originally is something that returns an iterator; but it turns out that
    /// it's quite involved to do so in rust today.  One would probably want to use an
    /// [ExactSizeIterator] for this purpose, but it turns out that it is very involved to define
    /// specifically a trait that establishes the lifetime relationships between the elements, the
    /// iterator itself and the [Variants].  So I didn't, and instead provided the needed functions
    /// here.  An `has_variants` predicate is absent because it's equivalent to
    /// `num_variants()==0`, and calling `num_variants()` should not require counting.
    pub trait Variants {
        /// Returns an integer representing the number of variants defined in this language
        /// identifier.
        fn num_variants(&self) -> usize;

        /// Calls `for_each` on each variant defined, and passes each one in turn
        /// to it.  Iteration order is random.  An example use is given below.  Care
        /// must be taken not to rely on any specific iteration order.
        ///
        /// ``` ignore
        /// let mut variants = HashSet::new();
        /// id.for_each_variant(|s| {
        ///     variants.insert(s.to_string());
        /// });
        /// ```
        fn for_each_variant(&self, for_each: impl FnMut(&str));
    }
}

#[cfg(test)]
mod tests {
    use crate::weird::Variants;
    use crate::LanguageIdentifier;
    use std::collections::HashSet;

    /// This is a sample implementation of the [Identifier] trait.  The static
    /// lifetimes of the components are chosen because it makes the tests short,
    /// but any internal implementation works.
    struct TestID {
        language: &'static str,
        region: Option<&'static str>,
        script: Option<&'static str>,
        variants: Vec<&'static str>,
    }
    impl LanguageIdentifier for TestID {
        fn language(&self) -> &str {
            self.language
        }
        fn region(&self) -> Option<&str> {
            self.region
        }
        fn script(&self) -> Option<&str> {
            self.script
        }
    }

    impl Variants for TestID {
        fn num_variants(&self) -> usize {
            self.variants.len()
        }

        fn for_each_variant(&self, mut for_each: impl FnMut(&str)) {
            self.variants.iter().map(|s| for_each(s)).for_each(drop);
        }
    }

    #[test]
    fn return_components() {
        let id = TestID {
            language: "en",
            region: Some("US"),
            script: None,
            // Note the variants in this example are not valid.
            variants: vec!["east_coast", "west_coast"],
        };
        assert_eq!(id.language(), "en");
        assert_eq!(id.region(), Some("US"));
        assert_eq!(id.script(), None);

        let mut variants = HashSet::new();
        id.for_each_variant(|s| {
            variants.insert(s.to_string());
        });

        // Iteration order is unspecified.
        let expected: HashSet<String> = ["west_coast", "east_coast"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(variants, expected);
    }
}

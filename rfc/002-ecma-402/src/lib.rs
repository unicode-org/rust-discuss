//! # I18n Concept RFC 002: ECMA-402 API surface for rust case study on `Intl.Locale`
//!
//! This RFC presents a rust take on ECMA-402 API.  See the [ECMA-402
//! specification](https://www.ecma-international.org/publications/standards/Ecma-402.htm) for more
//! detail on the spec itself.  For our purposes, the documentation of the [ECMA-402 API
//! surface](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl)
//! is perhaps more immediately useful.
//!
//! This RFC is presented as a rust crate, so that all presented concepts can be readily tested.
//!
//! ## Audience
//!
//! All people interested in the issues of internationalization (I18N) and localization (L10N).
//!
//! ## Scope
//!
//! Proposal for a rust API fullfilling basic internationalization needs, inspired by ECMA-402.
//!
//! # References
//!
//! - https://github.com/rust-lang/rfcs/issues/858
//! - https://github.com/i18n-concept/rust-discuss/issues/14
//!

/// `Option` is used to specify overriding options for the constructor methods.
///
/// Some of the Opts here could take `enum`s instead, to be less stringly typed.
pub enum Opt {
    Script(String),
    Region(String),
    HourCycle(String),
    Calendar(String),
}

/// Implements ECMA-402 `Intl.Locale`.
///
/// This is an exercise only.
pub trait Locale<'a>: Sized {
    type Error;

    /// Constructor method.
    fn new(tag: &'a str, options: &[Opt]) -> Result<Self, Self::Error>;

    fn language() -> Option<&'a str>;

    fn region() -> Option<&'a str>;

    fn script() -> Option<&'a str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LocImpl {
        // ...
    }

    impl<'a> Locale<'a> for LocImpl {
        fn script() -> Option<&'a str> { unimplemented!() }
        fn new(_: &'a str, _: &[Opt]) -> Result<Self, Self::Error> { unimplemented!() }
        type Error = u32;
        fn language() -> Option<&'a str> { unimplemented!() }
        fn region() -> Option<&'a str> { unimplemented!() }
    }
}

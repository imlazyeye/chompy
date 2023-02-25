use std::fmt::Display;

use crate::utils::{Located, Location};

/// A structure combining your individual token type and a [Location].
///
/// It should be notated that the [PartialEq] implementation of this struct *ignore* the contained
/// [Location]. This is for the sake of testing, which often does not want to include those
/// comparisons within their assertions. For a complete comparison, use [Eq].
#[derive(Debug, Eq, Clone, Copy)]
pub struct Tok<K: TokenKind + Display> {
    kind: K,
    location: Location,
}

impl<K: TokenKind + Display> Tok<K> {
    /// Creates a new Tok with the provided kind and location.
    pub fn new(kind: K, location: Location) -> Self {
        Self { kind, location }
    }

    /// Returns a reference to the inner TokenKind of this Tok.
    pub fn kind(&self) -> &K {
        &self.kind
    }
}

impl<K: TokenKind + Display> PartialEq<Tok<K>> for Tok<K> {
    fn eq(&self, other: &Tok<K>) -> bool {
        self.kind == other.kind
    }
}

impl<K: TokenKind + Display> Token<K> for Tok<K> {
    fn kind(self) -> K {
        self.kind
    }
}

impl<K: TokenKind + Display> Located for Tok<K> {
    fn location(&self) -> Location {
        self.location
    }
}

impl<K: TokenKind + Display> Display for Tok<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl<K: TokenKind + Display> From<Tok<K>> for String {
    fn from(val: Tok<K>) -> Self {
        val.to_string()
    }
}

/// A Token is a singular piece taken from source code to be later used in parsing the language's
/// grammer. For example, `&&`, `match` and `"hello!"` are all individual tokens in Rust.
///
/// This trait is used for the structure that contains the type which covers each kind of token in
/// your language (implementing [TokenKind]). Zap provides [Tok] as a basic implementor this trait.
pub trait Token<K>: Located + Sized {
    /// Takes the token and returns its kind the "kind" assosciated with this Tok.
    fn kind(self) -> K;
}

/// A [TokenKind] covers individual piece of your language that fits into a [Token].
pub trait TokenKind: PartialEq + Sized {}

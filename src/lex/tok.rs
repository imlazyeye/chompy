use std::fmt::Display;

use crate::utils::{Located, Location};

/// A structure combining your individual token type and a [Location].
///
/// It should be notated that the [PartialEq] implementation of this struct *ignore* the contained
/// [Location]. This is for the sake of testing, which often does not want to include those
/// comparisons within their assertions. For a complete comparison, use [Eq].
#[derive(Debug, Eq, Clone, Copy)]
pub struct Tok<K: TokenKind> {
    /// The inner kind of this Tok.
    pub kind: K,
    /// The [Location] this Tok comes from.
    pub location: Location,
}

impl<K: TokenKind> Tok<K> {
    /// Creates a new Tok with the provided kind and location.
    pub fn new(kind: K, location: Location) -> Self {
        Self { kind, location }
    }
}

impl<K: TokenKind> PartialEq<Tok<K>> for Tok<K> {
    fn eq(&self, other: &Tok<K>) -> bool {
        self.kind == other.kind
    }
}

impl<K: TokenKind> Token<K> for Tok<K> {
    fn kind(self) -> K {
        self.kind
    }

    fn kind_ref(&self) -> &K {
        &self.kind
    }
}

impl<K: TokenKind> Located for Tok<K> {
    fn location(&self) -> Location {
        self.location
    }
}

impl<K: TokenKind> Display for Tok<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.kind, f)
    }
}

impl<K: TokenKind> From<Tok<K>> for String {
    fn from(val: Tok<K>) -> Self {
        val.to_string()
    }
}

/// A Token is a singular piece taken from source code to be later used in parsing the language's
/// grammer. For example, `&&`, `match` and `"hello!"` are all individual tokens in Rust.
///
/// This trait is used for the structure that contains the type which covers each kind of token in
/// your language (implementing [TokenKind]). Zap provides [Tok] as a basic implementor this trait.
pub trait Token<K: TokenKind>: std::fmt::Debug + Located + Sized {
    /// Takes the token and returns its kind the "kind" assosciated with this Tok.
    fn kind(self) -> K;

    /// Returns a ref to the kind associated with this Tok.
    fn kind_ref(&self) -> &K;
}

/// A [TokenKind] covers individual piece of your language that fits into a [Token].
pub trait TokenKind: std::fmt::Debug + PartialEq + Sized + Display + Clone + Send + Sync {}

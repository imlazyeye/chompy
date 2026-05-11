//! chompy provides a series of utilities used to create parsers. This crate is primarily developed
//! to assist with the specific needs of my own projects but aspires to be robust enough to serve
//! any user interested in creating a parser quickly.

/// Tools for creating tokens and lexers.
pub mod lex {
    mod char_stream;
    mod errors;
    mod lex;
    mod tok;
    pub use char_stream::*;
    pub use errors::*;
    pub use lex::*;
    pub use tok::*;
}

/// Tools for creating parsers.
pub mod parse {
    mod errors;
    mod parse;
    pub use errors::*;
    pub use parse::*;
}

/// Common utilities shared across the different elements of chompy.
pub mod utils {
    mod files;
    mod location;
    mod unescape;
    pub use files::*;
    pub use location::*;
    pub use unescape::*;
}

/// Handles the creation and management of user-driven errors with chompy.
pub mod diagnostics {
    mod diag;
    mod macros;
    mod utils;
    pub use diag::*;
    pub use utils::*;
}

#[cfg(test)]
mod tests {
    mod lex;
    mod utils;
}

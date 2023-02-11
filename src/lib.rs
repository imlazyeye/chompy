//! Zap provides a series of utilities used to create parsers. This crate is primarily developed to
//! assist with the specific needs of my own projects but aspires to be robust enough to serve any
//! user interested in creating a parser quickly.

#![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::undocumented_unsafe_blocks)]

/// Tools for creating tokens and lexers.
pub mod lex {
    mod lexer;
    mod tok;
    pub use lexer::*;
    pub use tok::*;
}

/// Common utilities shared across the different elements of zap.
pub mod utils {
    mod files;
    mod location;
    pub use files::*;
    pub use location::*;
}

/// Handles the creation and management of user-driven errors with zap.
pub mod diagnostics {
    mod diag;
    mod macros;
    mod utils;
    pub use diag::*;
    pub use macros::*;
    pub use utils::*;
}

//! chompy provides a series of utilities used to create parsers. This crate is primarily developed
//! to assist with the specific needs of my own projects but aspires to be robust enough to serve
//! any user interested in creating a parser quickly.

#![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_inception)]

/// Tools for creating tokens and lexers.
pub mod lex {
    mod char_stream;
    mod lex;
    mod tok;
    pub use char_stream::*;
    pub use lex::*;
    pub use tok::*;
}

/// Common utilities shared across the different elements of chompy.
pub mod utils {
    mod files;
    mod location;
    pub use files::*;
    pub use location::*;
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

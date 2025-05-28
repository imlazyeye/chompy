use crate::{
    define_error,
    diagnostics::Builder,
    utils::{Located, Location},
};

/// Lexing error for a string that is never closed.
pub struct UnterminatedString(pub Location);
define_error!(
    UnterminatedString {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.0.primary("this string was never terminated"),)
        }

        fn location(&self) -> Location {
            self.0
        }
    }
);

/// Lexing error for a hexidecimal input that is invalid (i.e.: 0xffz)
pub struct InvalidHex(pub Location);

define_error!(
    InvalidHex {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.0.primary("this is not a valid hexidecimal value"))
        }

        fn location(&self) -> Location {
            self.0.location()
        }
    }
);

/// Lexing error for a character that no rule handeled.
pub struct UnexpectedChar(pub Location);

define_error!(
    UnexpectedChar {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.0.primary("did not expect this character in this position"))
        }

        fn location(&self) -> Location {
            self.0
        }
    }
);

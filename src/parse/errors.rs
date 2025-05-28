use crate::{
    define_error,
    diagnostics::Builder,
    lex::{Tok, TokenKind},
    utils::{Located, Location},
};

/// Parsing error when the end of a file is reached earlier than expected.
pub struct UnexpectedEnd(pub Location);
define_error!(
    UnexpectedEnd {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.0.primary("reached the end of the file in the middle of parsing a statement"),)
        }

        fn location(&self) -> Location {
            self.0
        }
    }
);

/// Parsing error w hen a token is encountered that was unexpected.
pub struct UnexpectedToken<K: TokenKind>(pub Tok<K>);

define_error!(
    UnexpectedToken<K: TokenKind> {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.0.primary(format!("'{}' is not valid in this position", self.0)))
        }

        fn location(&self) -> Location {
            self.0.location()
        }
    }
);

/// Parsing error when anything except for a particular token was encountered.
pub struct ExpectedToken<K: TokenKind>(pub K, pub Location);

define_error!(
    ExpectedToken<K: TokenKind> {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.1.primary(format!("expected this to be '{}'", self.0)))
        }

        fn location(&self) -> Location {
            self.1
        }
    }
);

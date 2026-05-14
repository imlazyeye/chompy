use crate::{
    define_error,
    diagnostics::Builder,
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

/// Parsing error when anything except for a particular token was encountered.
pub struct ExpectedToken(pub String, pub Location);

define_error!(
    ExpectedToken {
        fn build(&self, builder: Builder) -> Builder {
            builder.label(self.1.primary(format!("expected this to be '{}'", self.0)))
        }

        fn location(&self) -> Location {
            self.1
        }
    }
);

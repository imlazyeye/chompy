/// Provides the boilerplate for implementing [super::Diag] on a struct with
/// [super::Severity::Error].
///
/// Example usage:
/// ```
/// use zap::diagnostics::*;
/// use zap::utils::*;
/// use zap::define_error;
///
/// #[derive(Debug)]
/// struct MissingSemiColon(Location);
///
/// define_error!(
///     MissingSemiColon {
///         fn build(&self, builder: Builder) -> Builder {
///             builder
///                 .title("you're missing a semicolon!")
///                 .label(self.0.primary("should be right here!"))
///         }
///
///         fn location(&self) -> Location {
///             self.0
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_error {
    ($ty:ty { $build_fn:item $location_fn:item }) => {
        zap::define_diag!(
            zap::diagnostics::Severity::Error => $ty {
                $build_fn
                $location_fn
            }
        );
    };
}

/// Provides the boilerplate for implementing [super::Diag] on a struct with
/// [super::Severity::Warning].
///
/// Example usage:
/// ```
/// use zap::diagnostics::*;
/// use zap::utils::*;
/// use zap::define_warning;
///
/// #[derive(Debug)]
/// struct NotSoGood(Location);
///
/// define_warning!(
///     NotSoGood {
///         fn build(&self, builder: Builder) -> Builder {
///             builder
///                 .title("this isn't very good!")
///                 .label(self.0.primary("maybe change this!"))
///         }
///
///         fn location(&self) -> Location {
///             self.0
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_warning {
    ($ty:ty { $build_fn:item $location_fn:item }) => {
        zap::define_diag!(
            zap::diagnostics::Severity::Error => $ty {
                $build_fn
                $location_fn
            }
        )
    };
}

/// Provides the boilerplate for implementing [super::Diag] on a struct with [super::Severity::Bug].
///
/// Example usage:
/// ```
/// use zap::diagnostics::*;
/// use zap::utils::*;
/// use zap::define_bug;
///
/// #[derive(Debug)]
/// struct Unstable(Location);
///
/// define_bug!(
///     Unstable {
///         fn build(&self, builder: Builder) -> Builder {
///             builder.title("we became unstable!")
///         }
///
///         fn location(&self) -> Location {
///             self.0
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_bug {
    ($ty:ty { $build_fn:item $location_fn:item }) => {
        zap::define_diag!(
            zap::diagnostics::Severity::Error => $ty {
                $build_fn
                $location_fn
            }
        )
    };
}

/// Used to provide a [super::Severity] directly when defining a [super::Diag]. This is used
/// internally, and you should consider using [define_error], [define_warning], or [define_bug].
#[macro_export]
macro_rules! define_diag {
    ($severity:expr => $ty:ty { $build_fn:item $location_fn:item }) => {

        impl zap::diagnostics::Diag for $ty {
            fn severity(&self) -> zap::diagnostics::Severity {
                $severity
            }

            $build_fn
        }

        impl zap::utils::Located for $ty {
            $location_fn
        }
    };
}

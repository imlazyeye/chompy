use super::*;
use static_assertions::assert_obj_safe;
use std::fmt::Debug;

assert_obj_safe!(Diag);
/// The core trait used to create user-facing diagnostics in chompy. The macros
/// [crate::define_error], [crate::define_warning], and [crate::define_bug] can be utilized to
/// implement this to your types.
pub trait Diag: Debug + Send + Sync {
    /// Returns the [Severity] of this Diag.
    fn severity(&self) -> Severity;
    /// Returns the [Builder] used to assemble the information presented to the user.
    fn build(&self, builder: Builder) -> Builder;
}

/// Alias for a Result that returns DiagBox's.
pub type Result<'d, T> = std::result::Result<T, DiagBox<'d>>;

/// Container for dynamically dispatched implementers of [Diag].
pub struct DiagBox<'d>(Box<dyn Diag + Send + Sync + 'd>);

impl std::fmt::Debug for DiagBox<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'d, E> From<E> for DiagBox<'d>
where
    E: Diag + 'd,
{
    fn from(value: E) -> Self {
        DiagBox(Box::new(value))
    }
}

impl<'d> std::ops::Deref for DiagBox<'d> {
    type Target = Box<dyn Diag + Send + Sync + 'd>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

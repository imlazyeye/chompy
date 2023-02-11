use crate::utils::FileId;
use codespan_reporting::*;

/// Alias for [codespan_reporting]'s Severity for the sake of not requiring users to have codespan
/// as a dependency.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Severity {
    /// A bug within your parser, not the user's input.
    Bug,
    /// An error created by the user's input.
    Error,
    /// A problem with the user's code that they should be made aware of, but is not considered a
    /// hard error.
    Warning,
}

impl std::ops::Deref for Severity {
    type Target = codespan_reporting::diagnostic::Severity;

    fn deref(&self) -> &Self::Target {
        match self {
            Severity::Bug => &diagnostic::Severity::Bug,
            Severity::Error => &diagnostic::Severity::Error,
            Severity::Warning => &diagnostic::Severity::Warning,
        }
    }
}

impl From<Severity> for diagnostic::Severity {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Bug => diagnostic::Severity::Bug,
            Severity::Error => diagnostic::Severity::Error,
            Severity::Warning => diagnostic::Severity::Warning,
        }
    }
}

/// A wrapper for [diagnostic::Diagnostic] to assist in writing diagnostics laconically.
pub struct Builder(diagnostic::Diagnostic<FileId>);
impl Builder {
    /// Creates a new Builder with the provided [Severity].
    pub fn new(severity: Severity) -> Self {
        Self(diagnostic::Diagnostic::new(severity.into()))
    }

    /// Places the primary title associated with this diagnostic.
    pub fn title(mut self, name: impl Into<String>) -> Self {
        self.0.message = name.into();
        self
    }

    /// Adds a [Label] to this diagnostic to provide information about a specific location
    pub fn label(mut self, label: Label) -> Self {
        self.0.labels.push(label.into());
        self
    }

    /// Adds a note to this diagnostic to provide additional information to the user.
    pub fn note(mut self, message: impl Into<String>) -> Self {
        self.0.notes.push(message.into());
        self
    }
}

impl std::ops::Deref for Builder {
    type Target = diagnostic::Diagnostic<FileId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Builder> for diagnostic::Diagnostic<FileId> {
    fn from(value: Builder) -> Self {
        value.0
    }
}

/// A wrapper around [codespan_reporting::diagnostic::Label] as to not require users to require
/// codespan as a dependency.
pub struct Label(diagnostic::Label<FileId>);
impl Label {
    pub(crate) fn new(label: diagnostic::Label<FileId>) -> Self {
        Self(label)
    }
}

impl From<Label> for diagnostic::Label<FileId> {
    fn from(value: Label) -> Self {
        value.0
    }
}

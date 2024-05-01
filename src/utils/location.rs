use crate::diagnostics::Label;

use super::FileId;
use std::ops::Range;

/// A location for something in fog, combining a span and a file id.
#[derive(Debug, PartialEq, Eq, Default, Copy, Clone, Hash)]
pub struct Location {
    file_id: FileId,
    span: Span,
}
impl Location {
    /// Creates a new location with the given [FileId] and [Span].
    pub fn new(file_id: FileId, span: impl Into<Span>) -> Self {
        Self {
            file_id,
            span: span.into(),
        }
    }

    /// Returns the [Span] within this Location.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Returns the [FileId] for this Location.
    pub fn file_id(&self) -> FileId {
        self.file_id
    }
}

impl Located for Location {
    fn location(&self) -> Location {
        *self
    }
}

/// A Span is used to notate a specific range of characters in source code to later inform users the
/// exact location of diagnostics.
#[derive(Debug, PartialEq, Eq, Default, Copy, Clone, Hash)]
pub struct Span(usize, usize);
impl Span {
    /// Creates a new span.
    pub fn new(start: usize, end: usize) -> Self {
        // assert!(
        //     start <= end,
        //     "Attempted to make a Span with an ending lower than its start ({start} -> {end})"
        // );
        Self(start, end)
    }

    /// Returns the start of the span.
    pub fn start(&self) -> usize {
        self.0
    }

    /// Returns the end of the span.
    pub fn end(&self) -> usize {
        self.1
    }

    /// Creates a new span between this one's start and the provided one's end.
    pub fn until(&self, other: Self) -> Self {
        Self::new(self.0, other.1)
    }
}
impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.0..span.1
    }
}
impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Span::new(value.start, value.end)
    }
}

/// Implements utilities for types who have a [Location] attached to them.
pub trait Located {
    /// Returns the Location this item is from.
    fn location(&self) -> Location;

    /// Returns the span this item originates from.
    fn span(&self) -> Span {
        self.location().span
    }
    /// Returns the file id this item originates from.
    fn file_id(&self) -> FileId {
        self.location().file_id
    }

    /// Creates a primary label for diagnostics with this item's information
    fn primary(&self, message: impl Into<String>) -> Label {
        Label::new(
            codespan_reporting::diagnostic::Label::primary(self.file_id(), self.span())
                .with_message(message),
        )
    }

    /// Creates a secondary label for diagnostics with this item's information
    fn secondary(&self, message: impl Into<String>) -> Label {
        Label::new(
            codespan_reporting::diagnostic::Label::secondary(self.file_id(), self.span())
                .with_message(message),
        )
    }
}

use peekmore::{PeekMore, PeekMoreIterator};
use std::{ops::RangeBounds, str::Chars};

/// A wrapper of the needed iterators for [Lex]'s char stream.
pub struct CharStream {
    source: &'static str,
    iter: PeekMoreIterator<Chars<'static>>,
    peek_cursor: usize,
    true_cursor: usize,
}
impl CharStream {
    /// Creates a new CharStream out of the provided source material.
    pub fn new(source: &'static str) -> Self {
        Self {
            source,
            iter: source.chars().peekmore(),
            peek_cursor: 0,
            true_cursor: 0,
        }
    }

    /// Returns the next char in the stream and advances forward.
    pub fn chomp(&mut self) -> Option<char> {
        let Some(next) = self.iter.next() else {
            return None;
        };
        self.true_cursor += 1;
        self.peek_cursor = self.true_cursor;
        Some(next)
    }

    /// Chomps the next char if it matches the expected one.
    pub fn match_chomp(&mut self, expected: char) -> bool {
        if self.match_peek_with(|c| c == expected) {
            self.chomp().is_some()
        } else {
            false
        }
    }

    /// Chomps the next char if it satisfies the provided closure.
    pub fn match_chomp_with<F: FnOnce(char) -> bool>(&mut self, f: F) -> bool {
        if self.match_peek_with(f) {
            self.chomp().is_some()
        } else {
            false
        }
    }

    /// Returns the next char in the stream without moving the cursor.
    pub fn peek(&mut self) -> Option<char> {
        self.iter.peek().copied()
    }

    /// Returns if the next char if it matches the provided one.
    pub fn match_peek(&mut self, expected: char) -> bool {
        self.match_peek_with(|c| c == expected)
    }

    /// Returns if the next char if it satisfies the provided closure.
    pub fn match_peek_with<F: FnOnce(char) -> bool>(&mut self, f: F) -> bool {
        let Some(c) = self.peek() else { return false };
        if f(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Peeks, then advances the peek cursor by one.
    ///
    /// The peek cursor is separate from the true cursor of the stream. Whereas the true cursor will
    /// always represent the position of the next [chomp], the peek cursor can be moved around to
    /// inspect chars beyond the true cursor.
    pub fn peek_move(&mut self) -> Option<char> {
        let peek = self.peek();
        self.advance();
        peek
    }

    /// Chomps every char between the true cursor and peek cursor, returning a slice of the chomped
    /// chars. You can think of this as "fast forwarding" the true cursor to the position of the
    /// peek cursor.
    pub fn chomp_peeks(&mut self) -> &'static str {
        let slice = self.inspect_peeks();
        self.iter.truncate_iterator_to_cursor();
        self.true_cursor = self.peek_cursor;
        slice
    }

    /// Resets the peek cursor after returning a slice of everything between the true cursor and
    /// peek cursor.
    pub fn empty_peeks(&mut self) -> &'static str {
        let slice = self.inspect_peeks();
        self.reset_peeks();
        slice
    }

    /// Returns a slice of everything between the true cursor and peek cursor.
    pub fn inspect_peeks(&mut self) -> &'static str {
        let forward_position = std::cmp::min(self.source.len(), self.peek_cursor);
        self.slice(self.true_cursor..forward_position)
    }

    /// Resets the peek cursor to the position of the true cursor.
    pub fn reset_peeks(&mut self) {
        self.iter.reset_cursor();
        self.peek_cursor = self.true_cursor;
    }

    /// Advances the peek cursor forward by one.
    pub fn advance(&mut self) {
        self.peek_cursor += 1;
        self.iter.advance_cursor();
    }

    /// Returns the position of the true cursor.
    pub fn position(&self) -> usize {
        self.true_cursor
    }

    /// Returns the position of the peek cursor.
    pub fn peek_position(&self) -> usize {
        self.peek_cursor
    }

    /// Returns if the char stream has reached the end with nothing more to return.
    pub fn at_end(&self) -> bool {
        self.position() == self.source.len()
    }

    /// Returns a slice of the source within the provided range.
    pub fn slice<R>(&self, range: R) -> &'static str
    where
        R: RangeBounds<usize> + std::slice::SliceIndex<str, Output = str>,
    {
        &self.source[range]
    }

    /// Returns a reference to the CharStream's source.
    pub fn source(&self) -> &str {
        self.source
    }
}

/// Utility for keeping track of the current position of a [Lexer].
#[derive(Debug, Default)]
pub struct Cursor(usize);
impl Cursor {
    /// Returns the current position of the cursor.
    pub fn position(&self) -> usize {
        self.0
    }

    /// Returns the next position the cursor will encounter.
    pub fn next(&self) -> usize {
        self.0 + 1
    }
}

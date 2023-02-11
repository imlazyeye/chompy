use super::Token;
use std::{iter::Peekable, ops::RangeBounds, str::Chars};

/// A set of utilities to construct a lexer. After providing the neccesary references, the bulk of your lexer is written within the `lex` function.
pub trait Lex<T: Token<K>, K> {
    /// Returns a reference to the source code this lexer is consuming.
    fn source(&self) -> &'static str;

    /// Returns a reference to the iterator over the characters of the source.
    fn char_stream(&mut self) -> &mut Peekable<Chars<'static>>;

    /// Returns a reference to the [Cursor] this lexer uses.
    fn cursor(&mut self) -> &mut Cursor;

    /// The primary function for lexxing the next token in the stream.
    fn lex(&mut self) -> Option<T>;

    /// Returns the next char in the stream without advancing the lexer.
    fn peek(&mut self) -> Option<char> {
        self.char_stream().peek().copied()
    }

    /// Consumes the next char if it matches the one provided.
    fn match_chomp(&mut self, chr: char) -> bool {
        self.peek()
            .filter(|v| v == &chr)
            .and_then(|_| self.chomp())
            .is_some()
    }

    /// Consumes and returns the next char in the stream if any remain.
    fn chomp(&mut self) -> Option<char> {
        self.cursor().0 += 1;
        self.char_stream().next()
    }

    /// Returns a slice of the source within the provided range.
    fn slice<R>(&self, range: R) -> &'static str
    where
        R: RangeBounds<usize> + std::slice::SliceIndex<str, Output = str>,
    {
        &self.source()[range]
    }

    /// Chomps every char that fulfills the provided closure, then returns a slice of the chars
    /// from the provided start position to the final position reached.
    ///
    /// An oddity of the consume functions in Lex are their need to provide the `start` parameters.
    /// This does not actually affect where the lexer starts from, as it is working off of an
    /// iterator, but instead let's us define where the returned *slice* starts. This is become many
    /// lexers operate by matching over the value returned by chomp [chomp], making their cursor
    /// already shifted forward.
    ///
    /// Until this is improved, you should avoid using [peek] prior to using a consume function and
    /// instead should [chomp_take].
    fn consume<F>(&mut self, start: usize, mut f: F) -> &'static str
    where
        F: FnMut(char) -> bool,
    {
        while self.peek().map_or(false, &mut f) {
            self.chomp();
        }
        let pos = self.cursor().position();
        self.slice(start..pos)
    }

    /// Chomps every char that is either alphanumberic or an underscore, returning the resulting
    /// slice.
    ///
    /// This is provided as a utility for users working within common syntaxes, but for more
    /// specified control one should use [consume].
    fn consume_ident(&mut self, start: usize) -> &'static str {
        self.consume(start, |c| c.is_alphanumeric() || c == '_')
    }

    /// Chomps every char which is a digit, or optionally an underscore, returning the resulting
    /// slice.
    ///
    /// This is provided as a utility for users working within common syntaxes, but for more
    /// specified control one should use [consume].
    fn consume_number(&mut self, start: usize, allow_underscore: bool) -> &'static str {
        self.consume(start, |c| {
            c.is_ascii_digit() || c == '_' && allow_underscore
        })
    }

    /// Chomps every char that is commonly acceptable for a hex in an `0xffa` format, returning the
    /// resulting slice.
    ///
    /// This is provided as a utility for users working within common syntaxes, but for more
    /// specified control one should use [consume].
    fn consume_hex(&mut self, start: usize) -> &'static str {
        self.consume(start, |c| matches!(c, 'A'..='F' | 'a'..='f' | '0'..='9'))
    }

    /// Chomps every char until a newline is reached, returning the resulting slice.
    ///
    /// This is provided as a utility for users working within common syntaxes, but for more
    /// specified control one should use [consume].
    fn consume_line(&mut self, start: usize) -> &'static str {
        self.consume(start, |c| !matches!(c, '\n' | '\r'))
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

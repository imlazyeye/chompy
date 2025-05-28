use std::iter::Peekable;

use crate::{
    diagnostics::DiagBox,
    lex::{Lex, Token, TokenKind},
    utils::{FileId, Location, Span},
};

use super::errors::{ExpectedToken, UnexpectedEnd};

/// A set of utilities to create a parser.
pub trait Parse<L, T, K>
where
    L: Lex<T, K> + Iterator<Item = Result<T, DiagBox>> + 'static,
    T: Token<K>,
    K: TokenKind + 'static,
{
    /// Returns a mutable reference to the internal lexer.
    fn lexer(&mut self) -> &mut Peekable<L>;

    /// Returns the current [FileId].
    fn file_id(&self) -> FileId;

    /// Returns the current cursor (position) tracked by the parser.
    fn cursor(&self) -> usize;

    /// Mutates the cursor to the provided value. Used by the default implementations of this trait
    /// to navigate your lexer.
    fn set_cursor(&mut self, target: usize);

    /// Consumes and returns the next tok if it is the given type.
    fn match_take(&mut self, tok_kind: K) -> Option<T> {
        match self.peek() {
            Ok(peek) if peek.kind_ref() == &tok_kind => Some(self.take().unwrap()),
            Err(_) => None,
            _ => None,
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not of the required type.
    fn require(&mut self, expected_type: K) -> Result<T, DiagBox> {
        let found_tok = self.take()?;
        if found_tok.kind_ref() == &expected_type {
            Ok(found_tok)
        } else {
            Err(ExpectedToken(expected_type, found_tok.location()).into())
        }
    }

    /// Get the fog toks's cursor.
    fn next_tok_boundary(&mut self) -> usize {
        let cursor = self.cursor();
        self.lexer().peek().map_or(cursor, |tok| {
            tok.as_ref().map_or(cursor, |tok| tok.span().start())
        })
    }

    /// Returns the the next Token (or an error if there is none) without advancing the cursor.
    fn peek(&mut self) -> Result<&T, DiagBox> {
        let next = self.next_tok_boundary();
        let location = self.location(next);

        if self.lexer().peek().is_some_and(|v| v.is_err()) {
            return Err(self.take().unwrap_err());
        }

        match self.lexer().peek() {
            Some(Ok(tok)) => Ok(tok),
            None => Err(UnexpectedEnd(location).into()),
            _ => unreachable!(),
        }
    }

    /// Returns the next Token or None without advancing the cursor.
    fn soft_peek(&mut self) -> Result<Option<&T>, DiagBox> {
        if self.lexer().peek().is_some_and(|v| v.is_err()) {
            return Err(self.take().unwrap_err());
        }

        match self.lexer().peek() {
            Some(Ok(tok)) => Ok(Some(tok)),
            None => Ok(None),
            _ => unreachable!(),
        }
    }

    /// Returns the next Token, returning an error if there is none.
    fn take(&mut self) -> Result<T, DiagBox> {
        let next = self.next_tok_boundary();
        let location = self.location(next);
        match self.lexer().next() {
            Some(Ok(tok)) => {
                let boundary = self.next_tok_boundary();
                self.set_cursor(boundary);
                Ok(tok)
            }
            Some(Err(err)) => Err(err),
            None => Err(UnexpectedEnd(location).into()),
        }
    }

    /// Creates a [Span] from the given position up until the pilot's current position.
    fn span(&self, start: usize) -> Span {
        Span::new(start, self.cursor())
    }

    /// Creates a [Location] from the given position up until our current position.
    fn location(&self, start: usize) -> Location {
        Location::new(self.file_id(), self.span(start))
    }
}

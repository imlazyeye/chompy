use super::{CharStream, Token};

/// A set of utilities to construct a lexer. After providing the neccesary references, the bulk of
/// your lexer is written within the `lex` function.
pub trait Lex<T: Token<K>, K> {
    /// Returns a reference to the source code this lexer is consuming.
    fn source(&self) -> &'static str;

    /// Returns a reference to the iterator over the characters of the source.
    fn char_stream(&mut self) -> &mut CharStream;

    /// The primary function for lexxing the next token in the stream.
    fn lex(&mut self) -> Result<Option<T>, LexError>;

    // todo: this should all really be done via deref

    /// Returns the next char in the stream without advancing the lexer.
    fn peek(&mut self) -> Option<char> {
        self.char_stream().peek()
    }

    /// Chomps the next char if it matches the one provided.
    fn match_chomp(&mut self, chr: char) -> Option<char> {
        self.char_stream().match_chomp(chr)
    }

    /// Consumes and returns the next char in the stream if any remain.
    fn chomp(&mut self) -> Option<char> {
        self.char_stream().chomp()
    }

    /// Chomps every char that is either alphanumeric or an underscore, returning the resulting
    /// slice. If the first char found is not alphanumeric, None is returned.
    fn construct_ident(&mut self) -> Option<&'static str> {
        let test = |c: char| -> bool { c.is_alphanumeric() || c == '_' };
        self.char_stream()
            .match_peek_with(test)
            .then(|| self.construct(test))
    }

    /// Chomps every char which is a digit. Optionally allows underscores to appear within the
    /// number (though they cannot be the first character).
    fn construct_integer(&mut self, allow_underscore: bool) -> Option<i64> {
        let test = |c: char| -> bool { c.is_ascii_digit() || c == '_' && allow_underscore };
        self.peek()
            .map_or(false, |c| c.is_ascii_digit())
            .then(|| self.construct(test))
            .and_then(|v| v.replace('_', "").parse::<i64>().ok())
    }

    /// Attempts to construct an integer twice with a period inbetween the two.
    fn construct_float(&mut self, allow_underscore: bool) -> Option<f64> {
        let stream = self.char_stream();
        if !stream.match_peek_with(|c| c.is_ascii_digit()) {
            None
        } else {
            let mut found_dot = false;
            let mut found_trailing_digit = false;
            let slice = loop {
                match stream.peek_move() {
                    Some('_') if allow_underscore => {}
                    Some('.') if !found_dot => {
                        found_dot = true;
                    }
                    Some(chr) if chr.is_ascii_digit() => {
                        if found_dot {
                            found_trailing_digit = true;
                        }
                    }
                    _ if found_dot && found_trailing_digit => {
                        break Some(stream.chomp_peeks());
                    }
                    _ => break None,
                }
            };

            if let Some(float) = slice.and_then(|v| v.replace('_', "").parse::<f64>().ok()) {
                Some(float)
            } else {
                self.char_stream().reset_peeks();
                None
            }
        }
    }

    /// Chomps chars to create a string literal. You can provide the char's you allow to open/close
    /// a string, as well as well as any chars that can be used to escape your delimiters.
    ///
    /// If you support multiple delimiters then only the opening char can be matched as the closing
    /// char. For example, `'hello"` would not qualify as a valid string even if you support both
    /// `'` and `"`.
    ///
    /// If the first char found is not a quote, None is returned. If the end of the stream is
    /// reached before a closing quote is found, an error is returned within a Some().
    ///
    /// The returned slice includes the opening and closing quotations.
    fn construct_string(
        &mut self,
        quote_chars: &[char],
        escape_chars: &[char],
    ) -> Option<Result<&'static str, LexError>> {
        let stream = self.char_stream();
        let Some(opening_delim) = stream.peek_move() else { return None };
        if !quote_chars.contains(&opening_delim) {
            stream.reset_peeks();
            None
        } else {
            let mut in_escape = false;
            loop {
                match stream.peek_move() {
                    None => {
                        let slice = stream.empty_peeks();
                        break Some(Err(LexError::UnterminatedString(slice)));
                    }
                    Some(chr) if chr == opening_delim && !in_escape => {
                        break Some(Ok(stream.chomp_peeks()));
                    }
                    Some(chr) if escape_chars.contains(&chr) && !in_escape => in_escape = true,
                    _ => {}
                }
            }
        }
    }

    /// Chomps every char that is a digit or A through F (case agnostic). You can provide a prefix
    /// that must be matched at the start of the pattern.
    ///
    /// If the prefix is not fulfilled None is returned. If the first character following the prefix
    /// is not valid hex, an error is returned within the Some().
    fn construct_hex(&mut self, prefix: &str) -> Option<Result<&'static str, LexError>> {
        let stream = self.char_stream();
        if !prefix.chars().all(|p| stream.match_peek(p)) {
            stream.reset_peeks();
            None
        } else {
            while stream.match_peek_with(|c| c.is_ascii_hexdigit()) {}
            let slice = stream.chomp_peeks();
            if slice.is_empty() {
                Some(Err(LexError::InvalidHex(slice)))
            } else {
                Some(Ok(slice))
            }
        }
    }

    /// Chomps every char that follows the prefix. This will continue across multiple lines, meaning
    /// if you call this with two lines of comments ahead of you, both will be returned in this
    /// one call.
    fn construct_comment(&mut self, prefix: &str) -> Option<&'static str> {
        let start = self.char_stream().position();
        let mut found_any = false;
        loop {
            if !prefix.chars().all(|p| self.char_stream().match_peek(p)) {
                self.char_stream().reset_peeks();
                break;
            } else {
                found_any = true;
                self.chomp_line();
            }
        }
        if found_any {
            let stream = self.char_stream();
            let forward_position = std::cmp::min(stream.source().len(), stream.position());
            Some(stream.slice(start..forward_position))
        } else {
            None
        }
    }

    /// Chomps every char until a newline is reached, returning the resulting slice.
    fn chomp_line(&mut self) -> &'static str {
        let start = self.char_stream().position();
        loop {
            match self.chomp() {
                Some(c) if matches!(c, '\n' | '\r') => {
                    break;
                }
                Some(_) => {}
                None => break,
            }
        }
        let stream = self.char_stream();
        let forward_position = std::cmp::min(stream.source().len(), stream.position());
        stream.slice(start..forward_position)
    }

    /// Chomps every char that fulfills the provided closure, then returns a slice of the chars
    /// from the provided start position to the final position reached.
    fn construct<F>(&mut self, mut f: F) -> &'static str
    where
        F: FnMut(char) -> bool,
    {
        let stream = self.char_stream();
        while stream.match_peek_with(&mut f) {}
        stream.chomp_peeks()
    }
}

/// Errors that can be encountered from the in-built utilities in [Lex].
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    /// A string that was never closed before the end of the file.
    UnterminatedString(&'static str),
    /// A series of chars that begun with the hex-prefix but did not subsequently provide valid hex
    /// chars (i.e.: `0xQRS`).
    InvalidHex(&'static str),
    /// A char which satsified no rules within the lexer.
    UnexpectedChar(char),
}

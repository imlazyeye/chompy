use crate::{
    diagnostics::Result,
    utils::{FileId, Location, Span},
};

use super::{
    CharStream, Token, TokenKind,
    errors::{InvalidHex, UnterminatedString},
};

/// A set of utilities to construct a lexer. After providing the neccesary references, the bulk of
/// your lexer is written within the `lex` function.
pub trait Lex<'s, T: Token<K>, K: TokenKind> {
    /// Returns a reference to the source code this lexer is consuming.
    fn source(&self) -> &'s str;

    /// Returns a reference to the iterator over the characters of the source.
    fn char_stream(&mut self) -> &mut CharStream<'s>;

    /// Returns the [crate::utils::FileId] of the file this lexer is working within.
    fn file_id(&self) -> FileId;

    /// The primary function for lexxing the next token in the stream.
    fn lex(&mut self) -> Result<'s, Option<T>>;

    // todo: this should all really be done via deref

    /// Returns the next char in the stream without advancing the lexer.
    fn peek(&mut self) -> Option<char> {
        self.char_stream().peek()
    }

    /// Chomps the next char if it matches the one provided.
    fn match_chomp(&mut self, chr: char) -> bool {
        self.char_stream().match_chomp(chr)
    }

    /// Consumes and returns the next char in the stream if any remain.
    fn chomp(&mut self) -> Option<char> {
        self.char_stream().chomp()
    }

    // todo: all the bools on the below functions should be replaced with a bitflag configuration
    // struct that can be gotten with `self.config()`

    /// Chomps every char that is either alphanumeric or an underscore, returning the resulting
    /// slice. If the first char found is not alphanumeric, None is returned.
    fn construct_ident(&mut self) -> Option<&'s str> {
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
            .is_some_and(|c| c.is_ascii_digit())
            .then(|| self.construct(test))
            .and_then(|v| v.replace('_', "").parse::<i64>().ok())
    }

    /// Attempts to construct an integer twice with a period inbetween the two. You can optionally
    /// allow underscores to appear (which will discard them), and you can optionally allow
    /// scientific notation in the form `e<digits>`, `e+<digits>`, or `e-<digits>`. For example,
    /// with scientific notation enabled, '1.5e+02' returns 150.0 and '1.5e-02' returns 0.015.
    ///
    /// In the future the scientific-notation marker may become customizable but for now it is
    /// hardcoded to `e`.
    fn construct_float(
        &mut self,
        allow_underscore: bool,
        allow_scientific_notation: bool,
    ) -> Option<f64> {
        let stream = self.char_stream();
        if !stream.match_peek_with(|c| c.is_ascii_digit()) {
            return None;
        }

        let mut found_dot = false;
        let mut found_trailing_digit = false;
        let mantissa_slice = loop {
            match stream.peek() {
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
                    break stream.chomp_peeks();
                }
                _ => {
                    stream.reset_peeks();
                    return None;
                }
            }
            stream.advance();
        };

        let mantissa = mantissa_slice.replace('_', "").parse::<f64>().ok()?;

        if allow_scientific_notation && stream.peek() == Some('e') {
            stream.advance();

            let negative = match stream.peek() {
                Some('+') => {
                    stream.advance();
                    false
                }
                Some('-') => {
                    stream.advance();
                    true
                }
                _ => false,
            };

            let mut found_exp_digit = false;
            while stream.match_peek_with(|c| c.is_ascii_digit()) {
                found_exp_digit = true;
                stream.advance();
            }

            if found_exp_digit {
                // chomp_peeks yields "e[+|-]?<digits>"; rely on f64's parser to handle the whole
                // thing in one shot
                let suffix = stream.chomp_peeks();
                if let Ok(exp) = suffix.trim_start_matches(['e', '+', '-']).parse::<i32>() {
                    let signed = if negative { -exp } else { exp };
                    return Some(mantissa * 10f64.powi(signed));
                }
            }

            // Saw `e` but no valid exponent followed — discard the trial peeks
            // and fall through with just the mantissa
            stream.reset_peeks();
        }

        Some(mantissa)
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
    ///
    /// ## Caveats
    ///
    /// Because chompy is designed to only return source-borrowed/copy results, this does _not_ use your
    /// provided escape chars to escape anything _except_ your quotations. This means that
    /// characters like '\n' and '\r' will be retained as two individual characters in the string
    /// returned. We instead provide a utility, [utils::unescape], to filter those escapes into a newly
    /// allocated String.
    fn construct_string(
        &mut self,
        quote_chars: &[char],
        escape_chars: &[char],
    ) -> Option<Result<'s, &'s str>> {
        let file_id = self.file_id();
        let stream = self.char_stream();
        let start = stream.position();
        let opening_delim = stream.peek_move()?;
        if !quote_chars.contains(&opening_delim) {
            stream.reset_peeks();
            None
        } else {
            let mut in_escape = false;
            loop {
                match stream.peek_move() {
                    None => {
                        let location =
                            Location::new(file_id, Span::new(start, stream.peek_position()));
                        break Some(Err(UnterminatedString(location).into()));
                    }
                    Some(chr) if chr == opening_delim && !in_escape => {
                        let slice = stream.slice((start + 1)..stream.peek_position() - 1);
                        stream.chomp_peeks();
                        break Some(Ok(slice));
                    }
                    Some(chr) if escape_chars.contains(&chr) && !in_escape => in_escape = true,
                    _ => in_escape = false,
                }
            }
        }
    }

    /// Chomps every char that is a digit or A through F (case agnostic). You can provide a prefix
    /// that must be matched at the start of the pattern.
    ///
    /// If the prefix is not fulfilled None is returned. If the first character following the prefix
    /// is not valid hex, an error is returned within the Some().
    fn construct_hex(&mut self, prefix: &str) -> Option<Result<'s, &'s str>> {
        let file_id = self.file_id();
        let stream = self.char_stream();
        let start = stream.position();
        if !prefix.chars().all(|p| stream.match_peek(p)) {
            stream.reset_peeks();
            None
        } else {
            while stream.match_peek_with(|c| c.is_ascii_hexdigit()) {}
            let slice = stream.slice((start + 2)..stream.peek_position());
            stream.chomp_peeks();
            if slice.is_empty() {
                let location = Location::new(file_id, Span::new(start, stream.peek_position()));
                Some(Err(InvalidHex(location).into()))
            } else {
                Some(Ok(slice))
            }
        }
    }

    /// Chomps every char that follows the prefix. This will continue across multiple lines, meaning
    /// if you call this with two lines of comments ahead of you, both will be returned in this
    /// one call.
    fn construct_comment(&mut self, prefixes: &[&str]) -> Option<&'s str> {
        let start = self.char_stream().position();
        let mut found_any = false;
        loop {
            if !prefixes
                .iter()
                .any(|prefix| prefix.chars().all(|p| self.char_stream().match_peek(p)))
            {
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
    fn chomp_line(&mut self) -> &'s str {
        let start = self.char_stream().position();
        loop {
            match self.chomp() {
                Some('\n' | '\r') => {
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

    /// Chomps the next chars if they continously fulfill the pattern str provided.
    fn chomp_pattern(&mut self, pattern: &str) -> bool {
        let stream = self.char_stream();
        if pattern.chars().all(|v| stream.match_peek(v)) {
            stream.chomp_peeks();
            true
        } else {
            stream.reset_peeks();
            false
        }
    }

    /// Chomps every char that fulfills the provided closure, then returns a slice of the chars
    /// from the provided start position to the final position reached.
    fn construct<F>(&mut self, mut f: F) -> &'s str
    where
        F: FnMut(char) -> bool,
    {
        let stream = self.char_stream();
        while stream.match_peek_with(&mut f) {}
        stream.chomp_peeks()
    }
}

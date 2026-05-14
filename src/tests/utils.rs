use crate::{
    diagnostics::Result,
    lex::{CharStream, Lex, Tok, TokenKind, UnexpectedChar},
    utils::{Location, Span},
};

/// Test macro for token lexing.
#[macro_export]
macro_rules! test_tok_match {
    ($name:ident: $src:expr => $($should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            use $crate::lex::Lex;
            let expected = vec![$($should_be, )*];
            let mut lexer = $crate::tests::utils::Lexer::new($src);
            let mut outputed = vec![];
            while let Some(tok) = lexer.lex().unwrap() {
                outputed.push(tok.kind().clone());
            }
            pretty_assertions::assert_eq!(expected, *outputed);
        }
    };
}

/// Test macro for token mismatches.
#[macro_export]
macro_rules! test_tok_mismatch {
    ($name:ident: $src:expr => $($should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        #[should_panic]
        fn $name() {
            use $crate::lex::Lex;
            let expected = vec![$($should_be, )*];
            let mut lexer = $crate::tests::utils::Lexer::new($src);
            let mut outputed = vec![];
            while let Some(tok) = lexer.lex().unwrap() {
                outputed.push(tok.kind().clone());
            }
            pretty_assertions::assert_eq!(expected, *outputed);
        }
    };
}

/// Macro test for specific lex errors. We're just checking the primary label which is a bit lazy,
/// but DiagBox is not PartialEq.
#[macro_export]
macro_rules! test_lex_err {
    ($name:ident: $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            use $crate::{
                diagnostics::{Builder, Diag},
                lex::Lex,
            };
            let mut lexer = $crate::tests::utils::Lexer::new($src);
            pretty_assertions::assert_eq!(
                Err($should_be.build(Builder::new($should_be.severity()))),
                lexer
                    .lex()
                    .map_err(|e| e.as_ref().build(Builder::new(e.severity())))
            );
        }
    };
}

pub struct Lexer<'s> {
    source: &'s str,
    char_stream: CharStream<'s>,
    file_id: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            char_stream: CharStream::new(source),
            file_id: 0,
        }
    }
}

impl<'s> Lex<'s, Tok<TokKind<'s>>, TokKind<'s>> for Lexer<'s> {
    fn source(&self) -> &'s str {
        self.source
    }

    fn char_stream(&mut self) -> &mut CharStream<'s> {
        &mut self.char_stream
    }

    fn lex(&mut self) -> Result<Option<Tok<TokKind<'s>>>> {
        let start_pos = self.char_stream.position();
        if self.char_stream.match_chomp_with(|c| c.is_whitespace()) {
            return self.lex();
        }
        let kind = if let Some(hex) = self.construct_hex("0x") {
            TokKind::Hex(hex?)
        } else if let Some(float) = self.construct_float(true, true) {
            TokKind::Float(float)
        } else if let Some(int) = self.construct_integer(true) {
            TokKind::Int(int)
        } else if let Some(string) = self.construct_string(&['"', '\''], &['\\']) {
            TokKind::String(string?)
        } else if let Some(string) = self.construct_comment(&["//"]) {
            TokKind::Comment(string)
        } else if let Some(ident) = self.construct_ident() {
            match ident {
                "let" => TokKind::Let,
                "const" => TokKind::Const,
                ident => TokKind::Ident(ident),
            }
        } else if let Some(chr) = self.chomp() {
            match chr {
                '=' => {
                    if self.match_chomp('=') {
                        TokKind::DoubleEqual
                    } else {
                        TokKind::Equal
                    }
                }
                ';' => TokKind::SemiColon,
                _ => {
                    return Err(UnexpectedChar(Location::new(
                        self.file_id,
                        Span::new(start_pos, self.char_stream.position()),
                    ))
                    .into());
                }
            }
        } else {
            return Ok(None);
        };

        Ok(Some(Tok::new(
            kind,
            Location::new(0, start_pos..self.char_stream.position()),
        )))
    }

    fn file_id(&self) -> crate::utils::FileId {
        self.file_id
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokKind<'s> {
    Let,
    Const,
    Equal,
    DoubleEqual,
    SemiColon,
    Ident(&'s str),
    Int(i64),
    Float(f64),
    String(&'s str),
    Hex(&'s str),
    Comment(&'s str),
}

impl TokenKind for TokKind<'_> {}

impl std::fmt::Display for TokKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokKind::Let => f.pad("let"),
            TokKind::Const => f.pad("const"),
            TokKind::Equal => f.pad("="),
            TokKind::DoubleEqual => f.pad("=="),
            TokKind::SemiColon => f.pad(";"),
            TokKind::Ident(iden) => f.pad(iden),
            TokKind::Int(r) => f.pad(&r.to_string()),
            TokKind::Float(r) => f.pad(&r.to_string()),
            TokKind::String(s) => f.pad(&format!("\"{s}\"")),
            TokKind::Hex(hex) => f.pad(hex),
            TokKind::Comment(s) => f.pad(s),
        }
    }
}

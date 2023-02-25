use crate::{
    lex::{CharStream, Lex, LexError, Tok, TokenKind},
    utils::Location,
};

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

#[macro_export]
macro_rules! test_lex_err {
    ($name:ident: $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            use $crate::lex::Lex;
            let mut lexer = $crate::tests::utils::Lexer::new($src);
            pretty_assertions::assert_eq!(Err($should_be), lexer.lex());
        }
    };
}

pub struct Lexer {
    source: &'static str,
    char_stream: CharStream,
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Self {
            source,
            char_stream: CharStream::new(source),
        }
    }
}

impl Lex<Tok<TokKind>, TokKind> for Lexer {
    fn source(&self) -> &'static str {
        self.source
    }

    fn char_stream(&mut self) -> &mut CharStream {
        &mut self.char_stream
    }

    fn lex(&mut self) -> Result<Option<Tok<TokKind>>, LexError> {
        let start_pos = self.char_stream.position();
        if self
            .char_stream
            .match_chomp_with(|c| c.is_whitespace())
            .is_some()
        {
            return self.lex();
        }
        let kind = if let Some(hex) = self.construct_hex("0x") {
            TokKind::Hex(hex?)
        } else if let Some(float) = self.construct_float(true) {
            TokKind::Float(float)
        } else if let Some(int) = self.construct_integer(true) {
            TokKind::Int(int)
        } else if let Some(string) = self.construct_string(&['"', '\''], &['\\']) {
            TokKind::String(string?)
        } else if let Some(string) = self.construct_comment("//") {
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
                    if self.match_chomp('=').is_some() {
                        TokKind::DoubleEqual
                    } else {
                        TokKind::Equal
                    }
                }
                ';' => TokKind::SemiColon,
                _ => return Err(LexError::UnexpectedChar(chr)),
            }
        } else {
            return Ok(None);
        };
        Ok(Some(Tok::new(
            kind,
            Location::new(0, start_pos..self.char_stream.position()),
        )))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokKind {
    Let,
    Const,
    Equal,
    DoubleEqual,
    SemiColon,
    Ident(&'static str),
    Int(i64),
    Float(f64),
    String(&'static str),
    Hex(&'static str),
    Comment(&'static str),
}

impl TokenKind for TokKind {}

impl std::fmt::Display for TokKind {
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

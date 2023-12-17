use crate::{
    lex::{LexError::*, Token},
    test_lex_err, test_tok_match, test_tok_mismatch,
    tests::utils::TokKind::*,
    utils::Location,
};

test_tok_match!(semicolon: ";" => SemiColon);
test_tok_match!(equal: "=" => Equal);
test_tok_match!(double_equal: "==" => DoubleEqual);
test_tok_match!(ident: "foo" => Ident("foo"));
test_tok_match!(const_ident: "const" => Const);
test_tok_match!(let_ident: "let" => Let);
test_tok_match!(let_bind: "let foo" => Let, Ident("foo"));
test_tok_match!(int: "1" => Int(1));
test_tok_match!(int_series: "1 2 3" => Int(1), Int(2), Int(3));
test_tok_match!(int_with_underscores: "1_000_000" => Int(1_000_000));
test_tok_match!(float: "1.0" => Float(1.0));
test_tok_match!(float_with_underscores: "1_0.0_0" => Float(10.0));
test_tok_match!(hex: " 0xfff" => Hex("fff"));
test_tok_match!(string_single_quotes: "'hi'" => String("hi"));
test_tok_match!(string_double_quotes: "\"hi\"" => String("hi"));
test_tok_match!(comment: "// hello" => Comment("// hello"));
test_tok_match!(
    comment_after_other:
    "0 // hello" => Int(0), Comment("// hello")
);
test_tok_match!(
    comment_after_whitespace:
    "   // hello" => Comment("// hello")
);
test_tok_match!(
    multiline_comment:
    "// hello\n// there" => Comment("// hello\n// there")
);
test_tok_match!(
    doc_comment:
    "/// hello" => Comment("/// hello")
);
test_tok_match!(
    multiline_comment_mix:
    "// hello\n/// there\n// !" => Comment("// hello\n/// there\n// !")
);

test_lex_err!(unknown: "$" => UnexpectedChar(Location::new(0, 0..1)));
test_lex_err!(unterminated_string: "'foo" => UnterminatedString(Location::new(0, 0..5)));
test_lex_err!(mismatch_string_delimiter: "'foo\"" => UnterminatedString(Location::new(0, 0..6)));
test_tok_mismatch!(underscore_first_int: "_0" => Int(0));
test_tok_mismatch!(underscore_first_float: "_0.0" => Float(0.0));

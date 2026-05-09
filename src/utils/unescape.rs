/// Translates strings that have their escapes laid out as individual characters into their true,
/// single char references.
///
/// Handles \n, \r, \t, and \0.
///
/// See [Lex::construct_string] for more information.
pub fn unescape(input: &str, escape_chars: &[char], quote_chars: &[char]) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if !escape_chars.contains(&c) {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('0') => out.push('\0'),
            Some(c) if escape_chars.contains(&c) => out.push(c),
            Some(c) if quote_chars.contains(&c) => out.push(c),
            Some(other) => {
                out.push(c);
                out.push(other);
            }
            None => out.push(c),
        }
    }
    out
}

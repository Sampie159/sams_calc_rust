pub enum TokenKind {
    END = 0,
    ADD = '+',
    SUB = '-',
    MUL = '*',
    DIV = '/',
    EXP_BEGIN = '(',
    EXP_END = ')',
    WHITESPACE = ' ',
    NUMBER,
}

pub struct Token {
    kind: TokenKind,
    text: &str,
    size: u64,
}

#[repr(u8)]
pub enum TokenKind {
    END = 0,
    ADD = b'+',
    SUB = b'-',
    MUL = b'*',
    DIV = b'/',
    EXPBEGIN = b'(',
    EXPEND = b')',
}

pub struct Token {
    kind: TokenKind,
    text: String,
    size: u64,
}

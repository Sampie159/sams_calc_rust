pub enum TokenKind {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    EXPBEGIN,
    EXPEND,
    NUMBER,
}

pub struct Token {
    kind: TokenKind,
    text: String,
}

impl Token {
    pub fn new_token(kind: TokenKind, text: String) -> Self {
        Token { kind, text }
    }
}

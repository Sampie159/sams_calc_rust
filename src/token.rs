#[derive(Debug)]
pub enum TokenKind {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    NUMBER,
    EXP_BEGIN,
    EXP_END,
    INVALID,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: Option<String>,
}

impl TokenKind {
    pub fn get_token(ch: char) -> Self {
        return match ch {
            '+' => Self::ADD,
            '-' => Self::SUB,
            '*' => Self::MUL,
            '/' => Self::DIV,
            '^' => Self::POW,
            _ => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
        };
    }
}

impl Token {
    pub fn new_token(kind: TokenKind) -> Self {
        Token { kind, text: None }
    }

    pub fn new_full(kind: TokenKind, text: String) -> Self {
        Token {
            kind,
            text: Some(text),
        }
    }
}

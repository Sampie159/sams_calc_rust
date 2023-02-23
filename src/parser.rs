use crate::{
    calculator::{Operation, TreeOperations},
    token::{Token, TokenKind},
};

fn create_token_aux(tk: TokenKind, element: String) -> Token {
    let token = Token::new_token(tk, element);
    token
}

fn check_if_number(element: String) -> Token {
    for e in element.chars() {
        if !(e.is_digit(10) || e == '.') {
            panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        }
    }
    return create_token_aux(TokenKind::NUMBER, element);
}

fn create_token(element: String) -> Token {
    match element.as_str() {
        "(" => return create_token_aux(TokenKind::EXPBEGIN, element),
        ")" => return create_token_aux(TokenKind::EXPEND, element),
        "+" => return create_token_aux(TokenKind::ADD, element),
        "-" => return create_token_aux(TokenKind::SUB, element),
        "*" => return create_token_aux(TokenKind::MUL, element),
        "/" => return create_token_aux(TokenKind::DIV, element),
        "^" => return create_token_aux(TokenKind::POW, element),
        _ => return check_if_number(element),
    }
}

pub fn parse(buf: String) -> TreeOperations {
    let op: Operation = Operation::new_op(TokenKind::ADD);
    let mut head: TreeOperations = TreeOperations::new_tree(op);

    let buf = buf.split_whitespace();
    let mut token_list: Vec<Token> = Vec::new();

    for element in buf {
        token_list.push(create_token(String::from(element)));
    }

    for token in token_list {
        //TODO
    }

    head
}

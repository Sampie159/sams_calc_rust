use crate::{
    calculator::{Operation, TreeOperations},
    token::{Token, TokenKind},
};

fn is_operator(ch: char) -> bool {
    let operators = vec!['-', '+', '*', '/', '^'];
    operators.contains(&ch)
}

fn tokenize(buf: String) -> Vec<Token> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut buf = buf.chars().peekable();

    let mut aux = String::from("");

    let mut building_number = false;
    let mut idx = 0;

    while let Some(ch) = buf.next() {
        if ch.is_digit(10) {
            aux.push(ch);
            if !building_number {
                building_number = true;
            }
        } else if building_number {
            if ch == '.' || ch == ',' {
                aux.push(ch);
            } else {
                token_list.push(Token::new_full(TokenKind::NUMBER, aux.clone()));
                aux = String::from("");
                building_number = false;
            }
        }
        if is_operator(ch) {
            println!("is operator");
            token_list.push(Token::new_token(TokenKind::get_token(ch)))
        } else if ch == '(' {
            token_list.push(Token::new_token(TokenKind::EXP_BEGIN));
        } else if ch == ')' {
            token_list.push(Token::new_token(TokenKind::EXP_END));
        }

        println!("{idx}, {:?}", Some(ch));
        idx += 1;
    }

    if building_number {
        token_list.push(Token::new_full(TokenKind::NUMBER, aux));
    }

    for token in &token_list {
        println!("{:?}", token);
    }

    token_list.reverse();
    token_list
}

fn create_ast_aux(token_list: Vec<Token>, is_left: bool, idx: i8) {}

fn create_ast(token_list: Vec<Token>) -> TreeOperations {
    let mut op: Operation = Operation::new_op(TokenKind::INVALID);

    let mut is_left = true;

    for token in token_list {
        match token.kind {
            TokenKind::NUMBER => {
                if is_left {
                    op.arg1 = Some(token.text.unwrap().parse::<f64>().unwrap());
                    is_left = false;
                } else {
                    op.arg2 = Some(token.text.unwrap().parse::<f64>().unwrap());
                    is_left = true;
                }
            }
            TokenKind::EXP_BEGIN => continue, //TODO
            TokenKind::EXP_END => continue,   //TODO
            _ => op.op = token.kind,
        }
    }

    println!("{:?}", op.op);
    let to = TreeOperations::new_tree(op);
    to
}

pub fn parse(buf: &str) {
    let buf = buf.trim();
    let buf = buf.replace(" ", "");
    println!("{buf}");

    let token_list: Vec<Token>;
    token_list = tokenize(buf);

    let mut to = create_ast(token_list);

    println!("{}", to.solve_tree());
}

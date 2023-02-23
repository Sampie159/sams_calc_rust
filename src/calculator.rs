use crate::token::TokenKind;

pub struct Operation {
    op: TokenKind,
    arg1: Option<f64>,
    arg2: Option<f64>,
}

pub struct TreeOperations {
    op: Operation,
    left: Option<Box<TreeOperations>>,
    right: Option<Box<TreeOperations>>,
}

impl Operation {
    pub fn new_op(op: TokenKind) -> Self {
        Operation {
            op,
            arg1: None,
            arg2: None,
        }
    }

    pub fn calculate<T>(&self, func: T) -> f64
    where
        T: Fn(f64, f64) -> f64,
    {
        func(self.arg1.unwrap(), self.arg2.unwrap())
    }

    pub fn solve_operation(&self) -> f64 {
        let add = |a, b| a + b;
        let sub = |a, b| a - b;
        let mul = |a, b| a * b;
        let div = |a, b| a / b;
        let pow = |a: f64, b: f64| a.powf(b);
        match self.op {
            TokenKind::ADD => self.calculate(add),
            TokenKind::SUB => self.calculate(sub),
            TokenKind::MUL => self.calculate(mul),
            TokenKind::DIV => self.calculate(div),
            TokenKind::POW => self.calculate(pow),
            _ => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
        }
    }
}

impl TreeOperations {
    pub fn new_tree(op: Operation) -> Self {
        TreeOperations {
            op,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: TreeOperations) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: TreeOperations) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn solve_tree(&mut self) -> f64 {
        if self.left.is_some() {
            self.op.arg1 = Some(self.left.as_mut().unwrap().solve_tree());
        }
        if self.right.is_some() {
            self.op.arg2 = Some(self.right.as_mut().unwrap().solve_tree());
        }

        self.op.solve_operation()
    }
}

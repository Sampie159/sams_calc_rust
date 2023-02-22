use crate::token::TokenKind;

pub struct Operation {
    op: char,
    arg1: f64,
    arg2: f64,
}

pub struct TreeOperations {
    op: Operation,
    left: Option<Box<TreeOperations>>,
    right: Option<Box<TreeOperations>>,
}

impl Operation {
    fn calculate<T>(&self, func: T) -> f64
    where
        T: Fn(f64, f64) -> f64,
    {
        func(self.arg1, self.arg2)
    }

    pub fn solve_operation(&self) -> f64 {
        let add = |a, b| a + b;
        let sub = |a, b| a - b;
        let mul = |a, b| a * b;
        let div = |a, b| a / b;
        match self.op {
            TokenKind::ADD => self.calculate(add),
            TokenKind::SUB => self.calculate(sub),
            TokenKind::MUL => self.calculate(mul),
            TokenKind::DIV => self.calculate(div),
        }
    }
}

impl TreeOperations {
    pub fn solve_tree(&self) -> f64 {
        if self.left.is_some() {
            self.op.arg1 = self.left.solve_tree();
        }
        if self.right.is_some() {
            self.op.arg2 = self.right.solve_tree();
        }
        self.op.solve_operation()
    }
}

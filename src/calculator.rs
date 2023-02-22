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
    pub fn new_op(op: char, arg1: f64, arg2: f64) -> Self {
        Operation { op, arg1, arg2 }
    }

    pub fn calculate<T>(&self, func: T) -> f64
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
            '+' => self.calculate(add),
            '-' => self.calculate(sub),
            '*' => self.calculate(mul),
            '/' => self.calculate(div),
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
            self.op.arg1 = self.left.unwrap().solve_tree();
        }
        if self.right.is_some() {
            self.op.arg2 = self.right.unwrap().solve_tree();
        }

        self.op.solve_operation()
    }
}

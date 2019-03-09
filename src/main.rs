mod expr {
    use Expr::*;

    #[derive(Debug)]
    pub enum Expr {
        Val(i32),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
        Mul(Box<Expr>, Box<Expr>),
        Div(Box<Expr>, Box<Expr>),
    }

    pub fn val(value: i32) -> Box<Expr> {
        Box::new(Val(value))
    }

    pub fn add(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Add(left, right))
    }

    #[allow(dead_code)]
    pub fn sub(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Sub(left, right))
    }

    pub fn mul(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Mul(left, right))
    }

    pub fn div(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Div(left, right))
    }
}

fn main() {
    use expr::*;
    let e = div(mul(val(2), add(val(3), val(4))), val(2));
    println!("e={:?}", e)
}

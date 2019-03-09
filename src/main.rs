#[derive(Debug)]
enum Expr {
    Val(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn val(value: i32) -> Box<Expr> {
    Box::new(Expr::Val(value))
}

fn add(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Add(left, right))
}

#[allow(dead_code)]
fn sub(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Sub(left, right))
}

fn mul(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Mul(left, right))
}

fn div(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Div(left, right))
}

fn main() {
    let e = div(mul(val(2), add(val(3), val(4))), val(2));
    println!("e={:?}", e)
}

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

fn eval(e: Box<expr::Expr>) -> Option<i32> {
    use expr::Expr::*;
    match *e {
        Val(value) => Some(value),
        Add(left, right) => {
            let l = eval(left)?;
            let r = eval(right)?;
            Some(l + r)
        }
        Sub(left, right) => {
            let l = eval(left)?;
            let r = eval(right)?;
            Some(l - r)
        }
        Mul(left, right) => {
            let l = eval(left)?;
            let r = eval(right)?;
            Some(l * r)
        }
        Div(left, right) => {
            let r = eval(right)?;
            if r == 0 { return None }
            let l = eval(left)?;
            Some(l / r)
        }
    }
}

fn main() {
    use expr::*;
    {
        let e = div(mul(val(2), add(val(3), val(4))), val(2));
        println!("e={:?}", eval(e)) // Divide by 2: displays "Some(7)"
    }
    {
        let e = div(mul(val(2), add(val(3), val(4))), val(0));
        println!("e={:?}", eval(e)) // Divide by 0: displays "None"
    }
}

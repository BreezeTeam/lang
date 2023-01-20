use std::cell::{Cell, RefCell};
use std::process::id;
use std::rc::Rc;
use crate::ast::{Expr, Identifier, Infix, Literal, Prefix, Program, Stmt};
use crate::object::environment::Environment;
use crate::object::objects::Object;
use crate::object::objects::Object::Return;
use crate::object::objects::Object::NULL;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}


impl Evaluator {
    /// 对象转bool类型
    pub fn otb(&mut self, object: Object) -> Result<bool, Object> {
        match object {
            Object::Boolean(b) => Ok(b),
            Object::Error(s) => Err(Object::Error(s)),
            b => Err(Object::Error(format!("{} is not a bool", b))),
        }
    }

    pub fn oti(&mut self, object: Object) -> Result<i64, Object> {
        match object {
            Object::Integer(i) => Ok(i),
            Object::Error(s) => Err(Object::Error(s)),
            i => Err(Object::Error(format!("{} is not an integer", i))),
        }
    }

    pub fn otfn(&mut self, object: Object) -> Object {
        match object {
            Object::Function(_, _, _) | Object::Builtin(_, _, _) => object,
            Object::Error(s) => Object::Error(s),
            f => Object::Error(format!("{} is not a valid function", f)),
        }
    }

    pub fn othash(&mut self, object: Object) -> Object {
        match object {
            Object::Integer(i) => Object::Integer(i),
            Object::Boolean(b) => Object::Boolean(b),
            Object::String(s) => Object::String(s),
            Object::Error(s) => Object::Error(s),
            x => Object::Error(format!("{} is not hashable", x)),
        }
    }
}


impl Evaluator {
    /// new a evaluator and init
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    /// evaluation for Program
    pub fn evaluation(&mut self, program: Program) -> Object {
        self.eval_statements(program)
    }

    /// evaluation statements
    fn eval_statements(&mut self, statements: Vec<Stmt>) -> Object {
        let mut result = NULL;
        for stmt in statements {
            result = match self.eval_stmt(stmt) {
                Return(x) => *x,
                obj => obj,
            };
        };
        result
    }

    /// evaluation statement
    fn eval_stmt(&mut self, stmt: Stmt) -> Object {
        match stmt {
            Stmt::ExprStmt(expr) => self.eval_expr(expr),
            Stmt::ReturnStmt(expr) => Return(Box::new(self.eval_expr(expr))),
            Stmt::LetStmt(Identifier(ident), expr) => {
                let object = self.eval_expr(expr);
                self.env.borrow_mut().set(&ident, object.clone());
                object
            }
        }
    }
    /// evaluation expr
    fn eval_expr(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::IdentExpr(Identifier(ident)) => {
                match self.env.borrow().get(&ident) {
                    Some(o) => o,
                    None => Object::Error(format!("identifier not found: {}", ident)),
                }
            }
            Expr::LiteralExpr(l) => self.eval_literal(l),
            Expr::PrefixExpr(prefix, right) => self.eval_prefix(&prefix, *right),
            Expr::InfixExpr(infix, left, right) => self.eval_infix(&infix, *left, *right),

            //     Expr::IfExpr()
            // Expr::FnExpr()
            //     Expr::ArrayExpr()
            // Expr::HashExpr()
            //     Expr::InfixExpr()
            // Expr::CallExpr()
            //     Expr::IndexExpr()
            _ => Object::Error("todo".to_string())
        }
    }


    /// evaluation for literal
    fn eval_literal(&self, literal: Literal) -> Object {
        match literal {
            Literal::IntLiteral(i) => Object::Integer(i),
            Literal::BoolLiteral(b) => Object::Boolean(b),
            Literal::StringLiteral(s) => Object::String(s),
        }
    }
    /// evaluation for prefix (!,-,+)
    fn eval_prefix(&mut self, prefix: &Prefix, right: Expr) -> Object {
        let right = self.eval_expr(right);
        match prefix {
            Prefix::Plus => match self.oti(right) {
                Ok(i) => Object::Integer(i),
                Err(err) => err
            }
            Prefix::Minus => match self.oti(right) {
                Ok(i) => Object::Integer(-i),
                Err(err) => err
            }
            Prefix::Not => match self.otb(right) {
                Ok(i) => Object::Boolean(!i),
                Err(err) => err
            }
        }
    }
    fn eval_infix(&self, infix: &Infix, p1: Expr, p2: Expr) -> Object {
        todo!()
    }
}
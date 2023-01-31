use std::cell::{Cell, RefCell};
use std::process::id;
use std::rc::Rc;
use crate::ast::{BlockStatement, Expr, Identifier, Infix, Literal, Prefix, Program, Stmt};
use crate::object::environment::Environment;
use crate::object::objects::{BuiltinFunction, Object};
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
        match self.eval_statements(program) {
            Return(x) => *x,
            o => o
        }
    }

    /// evaluation statements
    fn eval_statements(&mut self, statements: Vec<Stmt>) -> Object {
        let mut result = NULL;
        for stmt in statements {
            let stmt_obj = self.eval_stmt(stmt);
            result = match stmt_obj {
                Return(_) => {
                    result = stmt_obj;
                    break;
                }
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
            Expr::IfExpr {
                cond, consequence, alternative
            } => self.eval_if(cond, consequence, alternative),
            Expr::FnExpr { parameters, body } => self.eval_fn(parameters, body),
            Expr::ArrayExpr(item_exprs) => self.eval_array(item_exprs),
            Expr::HashExpr(hash_pair_exprs) => self.eval_hash(hash_pair_exprs),
            Expr::CallExpr {
                function, arguments
            } => self.eval_call(function, arguments),
            Expr::IndexExpr {
                left, index
            } => self.eval_index(left, index),
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
    /// evaluation for prefix `(!,-,+)`
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
    /// evaluation for infix `(+,-,*,/,==,!=,>=,<=,>,<)`
    fn eval_infix(&mut self, infix: &Infix, left: Expr, right: Expr) -> Object {
        let left_obj = self.eval_expr(left);
        let right_obj = self.eval_expr(right);
        match infix {
            Infix::Plus => {
                match (left_obj, right_obj) {
                    (Object::Integer(i1), Object::Integer(i2)) => Object::Integer(i1 + i2),
                    (Object::String(s1), Object::String(s2)) => Object::String(s1 + &s2),
                    (Object::Error(s), _) | (_, Object::Error(s)) => Object::Error(s),
                    (x, y) => Object::Error(format!("{:?} and {:?} are not addable", x, y)),
                }
            }
            Infix::Minus => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Integer(i1 - i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
            Infix::Divide => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Integer(i1 / i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
            Infix::Multiply => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Integer(i1 * i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
            Infix::Equal => Object::Boolean(left_obj == right_obj),
            Infix::NotEqual => Object::Boolean(left_obj != right_obj),
            Infix::GreaterThanEqual => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Boolean(i1 >= i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
            Infix::LessThanEqual => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Boolean(i1 <= i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
            Infix::GreaterThan => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Boolean(i1 > i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
            Infix::LessThan => {
                match (self.oti(left_obj), self.oti(right_obj)) {
                    (Ok(i1), Ok(i2)) => Object::Boolean(i1 < i2),
                    (Err(err), _) | (_, Err(err)) => err,
                }
            }
        }
    }
    /// evaluation for `if {block} else {block}`
    fn eval_if(&mut self, cond: Box<Expr>, consequence: BlockStatement, alternative: Option<BlockStatement>) -> Object {
        let cond_obj = self.eval_expr(*cond);
        match self.otb(cond_obj) {
            Ok(b) => {
                if b {
                    self.eval_statements(consequence)
                } else {
                    match alternative {
                        None => Object::NULL,
                        Some(block) => self.eval_statements(block),
                    }
                }
            }
            Err(err) => err
        }
    }
    /// evaluation for `env:{ fn(parameters){body} }`
    fn eval_fn(&self, parameters: Vec<Identifier>, body: BlockStatement) -> Object {
        Object::Function(parameters, body, self.env.clone())
    }

    /// evaluation for array `[item,item,..]`
    fn eval_array(&mut self, items: Vec<Expr>) -> Object {
        Object::Array(items.into_iter().map(|item| self.eval_expr(item)).collect())
    }
    /// evaluation for hash `{literal:expr,...}`
    fn eval_hash(&mut self, hash_pairs: Vec<(Literal, Expr)>) -> Object {
        Object::Hash(hash_pairs.into_iter()
            .map(|(l, e)|
                (self.othash(self.eval_literal(l)), self.eval_expr(e))
            ).collect())
    }

    /// evaluation for call `(func_expr)([arguments])`
    fn eval_call(&mut self, function: Box<Expr>, arguments: Vec<Expr>) -> Object {
        let func = self.eval_expr(*function);
        match self.otfn(func) {
            Object::Function(params, body, f_env) => {
                self.eval_fn_call(arguments, params, body, f_env)
            }
            Object::Builtin(_, num_params, b_fn) => {
                self.eval_builtin_call(arguments, num_params, b_fn)
            }
            o => o,
        }
    }

    /// evaluation for index `(left_expr)[index_expr]`
    fn eval_index(&mut self, left: Box<Expr>, index: Box<Expr>) -> Object {
        let index = self.eval_expr(*index);
        match self.eval_expr(*left) {
            Object::Array(arr) => match self.oti(index) {
                Ok(index_number) => arr
                    .into_iter()
                    .nth(index_number as usize)
                    .unwrap_or(Object::NULL).clone(),
                Err(err) => err,
            },
            Object::Hash(hash) => {
                let name = self.othash(index);
                match name {
                    Object::Error(_) => name,
                    _ => hash.get(&name).unwrap_or(&Object::NULL).clone()
                }
            }
            o => Object::Error(format!("unexpected index target: {}", o)),
        }
    }
    fn eval_fn_call(&mut self, arguments: Vec<Expr>, params: Vec<Identifier>, body: BlockStatement, env: Rc<RefCell<Environment>>) -> Object {
        if arguments.len() != params.len() {
            Object::Error(format!(
                "wrong number of arguments: {} expected but {} given",
                params.len(),
                arguments.len()
            ))
        } else {
            let args = arguments.into_iter().map(|arg| self.eval_expr(arg)).collect::<Vec<_>>();
            let current_env = self.env.clone();

            // set function env from arguments and params
            let mut function_env = Environment::new_with_outer(env.clone());
            for (_, (Identifier(ident), obj)) in params.into_iter().zip(args).enumerate() {
                function_env.set(&ident, obj);
            }
            // evaluation body with function env
            self.env = Rc::new(RefCell::new(function_env));
            let result = self.eval_statements(body);
            // reset env
            self.env = current_env;
            match result {
                Return(v) => *v,
                o => o,
            }
        }
    }
    fn eval_builtin_call(&mut self, arguments: Vec<Expr>, num_params: usize, b_fn: BuiltinFunction) -> Object {
        if arguments.len() != num_params {
            Object::Error(format!(
                "wrong number of arguments: {} expected but {} given",
                num_params,
                arguments.len()
            ))
        } else {
            let args = arguments.into_iter().map(|arg| self.eval_expr(arg)).collect::<Vec<_>>();
            b_fn(args).unwrap_or_else(Object::Error)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::evaluator::Evaluator;
    use crate::lexer::Lexer;
    use crate::object::objects::Object;
    use crate::parser::Parser;
    use crate::token::Tokens;

    fn compare(input: &[u8], object: Object) {
        let (_, r) = Lexer::lexing(input).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result_parse) = Parser::parsing(tokens).unwrap();
        let mut evaluator = Evaluator::new();
        let eval = evaluator.evaluation(result_parse);
        assert_eq!(eval, object);
    }

    #[test]
    fn test_simple() {
        // ints
        compare("5".as_bytes(), Object::Integer(5));
        compare("10".as_bytes(), Object::Integer(10));
        // bools
        compare("true".as_bytes(), Object::Boolean(true));
        compare("false".as_bytes(), Object::Boolean(false));
    }

    #[test]
    fn test_prefix() {
        // bang operator
        compare("!false".as_bytes(), Object::Boolean(true));
        compare("!true".as_bytes(), Object::Boolean(false));
        compare("!!false".as_bytes(), Object::Boolean(false));
        compare("!!true".as_bytes(), Object::Boolean(true));

        compare(
            "!5".as_bytes(),
            Object::Error("5 is not a bool".to_string()),
        );
        compare(
            "!1".as_bytes(),
            Object::Error("1 is not a bool".to_string()),
        );
        compare(
            "!0".as_bytes(),
            Object::Error("0 is not a bool".to_string()),
        );
        compare(
            "!!1".as_bytes(),
            Object::Error("1 is not a bool".to_string()),
        );
        compare(
            "!!0".as_bytes(),
            Object::Error("0 is not a bool".to_string()),
        );
        // the prefix +
        compare("+1".as_bytes(), Object::Integer(1));
        compare("+5".as_bytes(), Object::Integer(5));
        compare("+20".as_bytes(), Object::Integer(20));
        compare(
            "+true".as_bytes(),
            Object::Error("true is not an integer".to_string()),
        );
        compare(
            "+false".as_bytes(),
            Object::Error("false is not an integer".to_string()),
        );
        // the prefix -
        compare("-1".as_bytes(), Object::Integer(-1));
        compare("-5".as_bytes(), Object::Integer(-5));
        compare("-20".as_bytes(), Object::Integer(-20));
        compare(
            "-true".as_bytes(),
            Object::Error("true is not an integer".to_string()),
        );
        compare(
            "-false".as_bytes(),
            Object::Error("false is not an integer".to_string()),
        );
    }

    #[test]
    fn test_infix_op() {
        // algebra
        compare("5 + 5 + 5 + 5 - 10".as_bytes(), Object::Integer(10));
        compare("2 * 2 * 2 * 2 * 2".as_bytes(), Object::Integer(32));
        compare("-50 + 100 + -50".as_bytes(), Object::Integer(0));
        compare("5 * 2 + 10".as_bytes(), Object::Integer(20));
        compare("5 + 2 * 10".as_bytes(), Object::Integer(25));
        compare("20 + 2 * -10".as_bytes(), Object::Integer(0));
        compare("50 / 2 * 2 + 10".as_bytes(), Object::Integer(60));
        compare("2 * (5 + 10)".as_bytes(), Object::Integer(30));
        compare("3 * 3 * 3 + 10".as_bytes(), Object::Integer(37));
        compare("3 * (3 * 3) + 10".as_bytes(), Object::Integer(37));
        compare(
            "(5 + 10 * 2 + 15 / 3) * 2 + -10".as_bytes(),
            Object::Integer(50),
        );
        // logic algebra
        compare("1 < 2".as_bytes(), Object::Boolean(true));
        compare("1 > 2".as_bytes(), Object::Boolean(false));
        compare("1 < 1".as_bytes(), Object::Boolean(false));
        compare("1 > 1".as_bytes(), Object::Boolean(false));
        compare("1 <= 2".as_bytes(), Object::Boolean(true));
        compare("1 >= 2".as_bytes(), Object::Boolean(false));
        compare("1 <= 1".as_bytes(), Object::Boolean(true));
        compare("1 >= 1".as_bytes(), Object::Boolean(true));
        compare("1 == 1".as_bytes(), Object::Boolean(true));
        compare("1 != 1".as_bytes(), Object::Boolean(false));
        compare("1 == 2".as_bytes(), Object::Boolean(false));
        compare("1 != 2".as_bytes(), Object::Boolean(true));
        // combination
        compare("(1 < 2) == true".as_bytes(), Object::Boolean(true));
        compare("(1 < 2) == false".as_bytes(), Object::Boolean(false));
        compare("(1 > 2) == true".as_bytes(), Object::Boolean(false));
        compare("(1 > 2) == false".as_bytes(), Object::Boolean(true));
    }

    #[test]
    fn test_conditional() {
        compare("if (true) { 10 }".as_bytes(), Object::Integer(10));
        compare("if (false) { 10 }".as_bytes(), Object::NULL);
        compare(
            "if (1) { 10 }".as_bytes(),
            Object::Error("1 is not a bool".to_string()),
        );
        compare("if (1 < 2) { 10 }".as_bytes(), Object::Integer(10));
        compare("if (1 > 2) { 10 }".as_bytes(), Object::NULL);
        compare(
            "if (1 < 2) { 10 } else { 20 }".as_bytes(),
            Object::Integer(10),
        );
        compare(
            "if (1 > 2) { 10 } else { 20 }".as_bytes(),
            Object::Integer(20),
        );
    }

    #[test]
    fn test_return() {
        // compare("return 10".as_bytes(), Object::Integer(10));
        // compare("return 10; 9".as_bytes(), Object::Integer(10));
        // compare("return 2 * 5; 9".as_bytes(), Object::Integer(10));
        // compare("9; return 2 * 5; 9".as_bytes(), Object::Integer(10));

        let input = "if (10 > 1) {\
                 if (10 > 1) {\
                     return 10;\
                 }\
                 return 1;\
             }\
            "
            .as_bytes();
        compare(input, Object::Integer(10));
    }

    #[test]
    fn test_bindings() {
        compare("let a = 5; a;".as_bytes(), Object::Integer(5));
        compare("let a = 5 * 5; a;".as_bytes(), Object::Integer(25));
        compare("let a = 5; let b = a; b;".as_bytes(), Object::Integer(5));
        compare(
            "let a = 5; let b = a; let c = a + b + 5; c;".as_bytes(),
            Object::Integer(15),
        );
        compare(
            "foobar".as_bytes(),
            Object::Error("identifier not found: foobar".to_string()),
        );
    }

    #[test]
    fn test_strings() {
        compare(
            "\"foobar\"".as_bytes(),
            Object::String("foobar".to_string()),
        );
        compare(
            "\"foo\" + \"bar\"".as_bytes(),
            Object::String("foobar".to_string()),
        );
        compare(
            "\"foo\" + \" \" + \"bar\"".as_bytes(),
            Object::String("foo bar".to_string()),
        );
        compare(
            "\"foo\" - \"bar\"".as_bytes(),
            Object::Error("foo is not an integer".to_string()),
        );
    }

    #[test]
    fn test_fn() {
        compare(
            "let identity = fn(x) { x; }; identity(5);".as_bytes(),
            Object::Integer(5),
        );
        compare(
            "let identity = fn(x) { return x; }; identity(5);".as_bytes(),
            Object::Integer(5),
        );
        compare(
            "let double = fn(x) { x * 2; }; double(5);".as_bytes(),
            Object::Integer(10),
        );
        compare(
            "let add = fn(x, y) { x + y; }; add(5, 5);".as_bytes(),
            Object::Integer(10),
        );
        compare(
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".as_bytes(),
            Object::Integer(20),
        );
        compare("fn(x) { x; }(5)".as_bytes(), Object::Integer(5));
        compare(
            "5();".as_bytes(),
            Object::Error("5 is not a valid function".to_string()),
        );
        compare(
            "false();".as_bytes(),
            Object::Error("false is not a valid function".to_string()),
        );
        compare(
            "let add = fn(x, y) { x + y; }; add(1);".as_bytes(),
            Object::Error("wrong number of arguments: 2 expected but 1 given".to_string()),
        );
        compare(
            "let a = 10; let x = fn () { a; }; x();".as_bytes(),
            Object::Integer(10),
        );
        compare(
            "let x = fn () { a; }; let a = 10; x();".as_bytes(),
            Object::Integer(10),
        );

        let fn_input1 = "let add = fn(a, b, c, d) { return a + b + c + d; };\
             add(1, 2, 3, 4);\
            "
            .as_bytes();

        let fn_input2 = "let addThree = fn(x) { return x + 3 };\
             addThree(3);\
            "
            .as_bytes();

        let fn_input3 = "let max = fn(x, y) { if (x > y) { x } else { y } };\
             max(5, 10)\
            "
            .as_bytes();

        let fn_input4 = "let factorial = fn(n) {\
                if (n == 0) {\
                    1\
                } else {\
                    n * factorial(n - 1)\
                }\
             }\
             factorial(5)\
            "
            .as_bytes();

        let fn_input5 = "let addThree = fn(x) { return x + 3 };\
             let callTwoTimes = fn(x, f) { f(f(x)) }\
             callTwoTimes(3, addThree);\
            "
            .as_bytes();

        let fn_input6 = "let callTwoTimes = fn(x, f) { f(f(x)) }\
             callTwoTimes(3, fn(x) { x + 1 });\
            "
            .as_bytes();

        let fn_input7 = "let newAdder = fn(x) { fn(n) { x + n } };\
             let addTwo = newAdder(2);\
             addTwo(2);\
            "
            .as_bytes();

        compare(fn_input1, Object::Integer(10));
        compare(fn_input2, Object::Integer(6));
        compare(fn_input3, Object::Integer(10));
        compare(fn_input4, Object::Integer(120));
        compare(fn_input5, Object::Integer(9));
        compare(fn_input6, Object::Integer(5));
        compare(fn_input7, Object::Integer(4));
    }

    #[test]
    fn test_array() {
        compare(
            "[1, 2, 3, 4]".as_bytes(),
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(4),
            ]),
        );

        compare(
            "let double = fn(x) { x * 2 };[1, double(2), 3 * 3, 4 - 3]".as_bytes(),
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(4),
                Object::Integer(9),
                Object::Integer(1),
            ]),
        );

        compare("[1, 2, 3][0]".as_bytes(), Object::Integer(1));
        compare("[1, 2, 3][1]".as_bytes(), Object::Integer(2));
        compare("[1, 2, 3][2]".as_bytes(), Object::Integer(3));
        compare("let i = 0; [1][i];".as_bytes(), Object::Integer(1));
        compare("[1, 2, 3][1 + 1];".as_bytes(), Object::Integer(3));
        compare(
            "let myArray = [1, 2, 3]; myArray[2];".as_bytes(),
            Object::Integer(3),
        );
        compare(
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];".as_bytes(),
            Object::Integer(6),
        );
        compare(
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i];".as_bytes(),
            Object::Integer(2),
        );
        compare("[1, 2, 3][3]".as_bytes(), Object::NULL);
        compare("[1, 2, 3][-1]".as_bytes(), Object::NULL);
    }

    #[test]
    fn test_hash() {
        let input_beg = "let double = fn(x) {
           x * 2;
         };
         let arr = [1, 2, 3, 4];
         let h = {
           \"one\": 10 - 9,
           \"two\": 8 / 4,
           3: arr[2],
           4: double(2),
           true: if (10 > 8) { true } else { false },
           false: \"hello\" == \"world\"
         };
        "
            .to_string();

        compare(
            (input_beg.clone() + "h[\"one\"]").as_bytes(),
            Object::Integer(1),
        );
        compare(
            (input_beg.clone() + "let s = \"two\"; h[s]").as_bytes(),
            Object::Integer(2),
        );
        compare(
            (input_beg.clone() + "h[3]").as_bytes(),
            Object::Integer(3),
        );
        compare(
            (input_beg.clone() + "h[2 + 2]").as_bytes(),
            Object::Integer(4),
        );
        compare(
            (input_beg.clone() + "h[true]").as_bytes(),
            Object::Boolean(true),
        );
        compare(
            (input_beg.clone() + "h[5 < 1]").as_bytes(),
            Object::Boolean(false),
        );
        compare(
            (input_beg.clone() + "h[100]").as_bytes(),
            Object::NULL,
        );
        compare(
            (input_beg.clone() + "h[[]]").as_bytes(),
            Object::Error("[] is not hashable".to_string()),
        );
        compare(
            (input_beg + "3[true];").as_bytes(),
            Object::Error("unexpected index target: 3".to_string()),
        );
    }

    #[test]
    fn test_builtins() {
        // len
        compare("len(\"hello world!\")".as_bytes(), Object::Integer(12));
        compare("len(\"\")".as_bytes(), Object::Integer(0));
        compare(
            "len(\"Hey Bob, how ya doin?\")".as_bytes(),
            Object::Integer(21),
        );
        compare(
            "len(3)".as_bytes(),
            Object::Error("invalid arguments for len".to_string()),
        );
        compare(
            "len(\"hello\", \"world\")".as_bytes(),
            Object::Error("wrong number of arguments: 1 expected but 2 given".to_string()),
        );
        compare("len([])".as_bytes(), Object::Integer(0));
        compare("len([1, 2, 3, 4])".as_bytes(), Object::Integer(4));
        // head
        compare("head([1])".as_bytes(), Object::Integer(1));
        compare("head([1, 2, 3, 4])".as_bytes(), Object::Integer(1));
        compare(
            "head([])".as_bytes(),
            Object::Error("empty array".to_string()),
        );
        // tail
        compare("tail([1])".as_bytes(), Object::Array(vec![]));
        compare(
            "tail([1, 2, 3, 4])".as_bytes(),
            Object::Array(vec![
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(4),
            ]),
        );
        compare(
            "tail([])".as_bytes(),
            Object::Error("empty array".to_string()),
        );
        // cons
        compare(
            "cons(1, [])".as_bytes(),
            Object::Array(vec![Object::Integer(1)]),
        );
        compare(
            "cons(1, [2, 3, 4])".as_bytes(),
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
                Object::Integer(4),
            ]),
        );
        // map reduce
        let map_decl = "let map = fn(f, arr) {\
              if (len(arr) == 0) {\
                []\
              } else {\
                let h = head(arr);\
                cons(f(h), map(f, tail(arr)));\
              }\
            };\
            "
            .to_string();

        let reduce_decl = "let reduce = fn(f, init, arr) {\
                if (len(arr) == 0) {\
                    init\
                } else {\
                    let newInit = f(init, head(arr));\
                    reduce(f, newInit, tail(arr));\
                }\
            };\
            "
            .to_string();

        compare(
            (map_decl + "let double = fn(x) { x * 2 }; map(double, [1, 2, 3, 4])").as_bytes(),
            Object::Array(vec![
                Object::Integer(2),
                Object::Integer(4),
                Object::Integer(6),
                Object::Integer(8),
            ]),
        );

        compare(
            (reduce_decl + "let add = fn(x, y) { x + y }; reduce(add, 0, [1, 2, 3, 4, 5])")
                .as_bytes(),
            Object::Integer(15),
        );
    }
}

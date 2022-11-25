use std::fmt::Debug;
// 泛型约束

// 打印泛型对象，该对象必须实现 Debug Trait
fn print_debug_data<T: Debug>(t: &T) {
    println!("{:?}", t);
}
#[derive(Debug)]
struct xx {
    a: i64,
    b: i64,
}
struct yy {
    a: f64,
    b: f64,
}

fn main() {
    let x = xx { a: 1, b: 2 };
    print_debug_data(&x);
    let y = yy { a: 1.0, b: 2.1 };
    // print_debug_data(&y);
    // y 没有 实现 Debug trait，会报错
}

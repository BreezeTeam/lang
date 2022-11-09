// 闭包：也叫做lambda 表达式。是一类可以捕获周围作用域中变量的函数

// |var|var+x ; 该闭包可以捕获x变量

fn main() {
    // 一个典型的函数如下
    fn function(i: i32) -> i32 {
        1 + i
    }

    // 闭包：
    // 由于闭包是匿名的，所以需要绑定到一个引用
    // 完整的闭包包括 |入参:变量标注|->返回变量标注 {函数体}，
    let closure_1 = |i: i32| -> i32 { i + 1 };
    // 省略 类型标注
    let closure_2 = |i| i + 1;
    // 省略 函数体 的 作用域 {}
    let closure_3 = |i| i + 1;

    // 调用函数和闭包
    let i = 1;
    println!("function:{}", function(i));
    println!("closure_1:{}", closure_1(i));
    println!("closure_2:{}", closure_2(i));
    println!("closure_3:{}", closure_3(i));

    //无参闭包,返回一个 1
    let one = || 1;
    println!("one:{}", one());
}

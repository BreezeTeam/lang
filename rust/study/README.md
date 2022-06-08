# reference

## rust-by-example

https://rustwiki.org/zh-CN/rust-by-example/hello.html

## hello_world

```rust
fn main() {
    // 这是一个宏
    println!("Hello, world!");
}
```

```bash 
#rustc 从源程序生成可执行程序
rustc .\src\main.rs 
# 执行可执行文件
.\main.exe
```

## 注释

```rust
fn main() {
    //  注释
    /*
    多行注释
     */
    println!("Hello, world!");

    /*
    块注释在 语法分析时，应该会被去除，从而能将 被块注释分割的表达式合并
     */
    /// 文档注释
    //! 为注释所属于的项（译注：如 crate、模块或函数）生成帮助文档。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
```

## 打印

```rust
fn main() {
    //  注释
    /*
    多行注释
     */
    println!("Hello, world!");

    /*
    块注释在 语法分析时，应该会被去除，从而能将 被块注释分割的表达式合并
     */
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // 使用{} 来替换变量的内容
    println!("{} days", 12);
    // 如果 12 不加后缀，那么会转为i32 ，可以加后缀
    println!("{} days", 12i64);

    // 使用命名参数替换变量的位置
    println!("subject:{subject}", subject = "test");

    //也可以在:号后面指定特殊的格式,:b 指二进制
    println!("特殊的格式: 10：{}; 2：{:b}; 8:{:o}; 16：{:x}", 19, 19, 19, 19);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");


    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    // 打印 浮点数
    // https://rustwiki.org/zh-CN/std/fmt/
    let pi = 3.14159;
    println!("{:.2}", pi);
}
```

## fmt Debug

所有的类型，若想用 std::fmt 的格式化打印，都要求实现至少一个可打印的 traits。 自动的实现只为一些类型提供，比如 std 库中的类型。所有其他类型都必须手动实现。 使用`#[derive(Debug)]`
自动实现非std对象的trait 实现

使用`{:?}` 进行打印 使用`{:#?}` 进行美化打印:

```bash
Person {
    name: "Peter",
    age: 27,
}

```

源代码:

```rust

// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);


// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", DebugPrintable(3));
    let unprint = UnPrintable(3);
    // println!("Now {:?} will print!", unprint);

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));


    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}

```

## fmt Display

自己实现Display 和 Debug 进行比较

```rust
extern crate core;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Test(i32, i32);

impl Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.real, self.imag)
    }
}

fn main() {
    let test = Test(1, 2);
    println!("Debug {:?}", test);
    println!("Display {}", test);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Debug {:?}", complex);
    println!("Display {}", complex);
}
```

## 创建宏

Rust 提供了一个强大的宏系统，可进行元编程（metaprogramming）。 宏看起来和函数很像，只不过名称末尾有一个感叹号 ! 。 宏并不产 生函数调用，而是展开成源码，并和程序的其余部分一起被编译。 Rust 又有一点和 C
以及其他语言都不同，那就是 Rust 的宏会展开为抽象语法树（AST，abstract syntax tree）， 而不是像字符串预处理那样直接替换成代码，这样就不会产生无法预料的优先权 错误。

```rust
macro_rules! macro_test {
    () => (
        println!("表示此宏不接受任何参数；此宏将会展开成这个代码块里面的内容。!");
    )
}

fn main() {
    // 这个调用将会展开成 `println("Hello");`!
    macro_test!();
}
```




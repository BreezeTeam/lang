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

## fmt Display( {} )

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

## ?宏 和 try！宏

？操作符：对于某个语句进行尝试，观察是否出错，如果发生错误，返回相应错误，否则继续执行后面的语句

```rust
use std::fmt::{Display, Formatter};

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?
            }
            write!(f, "{}", v)?
        }
        write!(f, "]")
    }
}

fn main() {
    let x = List(vec![1, 2, 3, 4]);
    println!("{}", x)
}
```

## 字面量

```rust
fn main() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要

    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);

    println!("string as {}", "xxx")
}

```

## 元组

```rust
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// 编写display
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})\n", self.0, self.1)?;
        write!(f, "({} {})\n", self.2, self.3)
    }
}

fn transpose(mat: Matrix) -> Matrix {
    let (a, b, c, d) = (mat.0, mat.1, mat.2, mat.3);
    Matrix(a, c, b, d)
}

fn main() {
    let mat = Matrix(1.0, 2.0, 3.0, 4.0);

    println!("{:?}", mat);
    println!("{}", mat);
    println!("{}", transpose(mat))
}

```

## 数组和切片

```rust
use std::mem;

// &[i32] 借用一个slcie
fn analyze_slice(slice: &[i32]) {
    println!("slice:{:?}", slice);
    println!("sliceSize:{}", slice.len());
}

fn main() {
    // 自动类型推导，不需要进行类型标记
    //定长数组
    let a1 = [1, 2, 3, 4, 5];
    // 所有元素初始化为相同的值
    let a2 = [true; 500];
    println!("定长数组:{:?}", a1);
    println!("定长数组:{:?}", a2);

    // 获取数组长度
    println!("数组长度：{}", a1.len());

    //数组在栈中分配
    // mem:size_of_val 返回指向值的大小(以字节为单位)。
    println!("{}", mem::size_of_val(&a2));
    // 以字节为单位返回类型的大小。
    println!("{}", mem::size_of::<bool>());

    //slice
    //数组自动被借用为slice
    analyze_slice(&a1);
    analyze_slice(&a1[1..4]);
    analyze_slice(&a1[0..1]);
    //下标越界时会panic
}
```

## 结构体

```rust
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//单元结构体
struct Unit;

//元组结构体
#[derive(Debug)]
struct Pair(i32, i32);

//结构体可以作为另一个结构体的字段
struct Rectangle {
    top: Point,
    bottom: Point,
}

fn main() {
    // 初始化字段并创建结构体
    let name = String::from("test");
    let age = 27;
    let person = Person { name, age };
    println!("Hello, {}!", person.name);
    println!("{}", person);

    // 实例化结构体 point
    let point: Point = Point { x: 10.0, y: 20.0 };
    println!("{}", point);

    // 使用结构体更新语法创建新的point
    let new_point = Point { x: 5.2, ..point };
    println!("{}", new_point);

    //使用let 绑定 解构 point
    let Point { x: x1, y: y1 } = new_point;
    println!("({},{})", x1, y1);

    //在结构体的实例化中使用结构体的实例化
    let _rectangle = Rectangle {
        top: Point { x: 10.0, y: 20.0 },
        bottom: Point { x: 20.0, y: 30.0 },
    };

    // 实力一个单元结构体
    let _unit = Unit;

    // 使用元组
    let pair = Pair(1, 2);
    println!("{:?}", pair);
    println!("{:?} and {:?}", pair.0, pair.1);

    // 解构一个 元组结构体
    let Pair(first, second) = pair;
    println!("{:?} , {:?}", first, second);
}

```

## enum

```rust
use std::fmt::{Display, Formatter};

use crate::Status::Poor;

// 该属性用于隐藏
#[allow(dead_code)]

// 1. 创建一个Event 对事件进行分类
enum WebEvent {
    // 单元结构体
    PageLoad,
    PageUnload,

    // 元组结构体
    KeyPress(char),
    Paste(String),
    // 普通结构体
    Click { x: i64, y: i64 },
}

// 2. 创建一个类型别名
enum TooLongEnumName {
    ADD,
    SUB,
}

// 创建一个短一点的类型别名
type operations = TooLongEnumName;

// 3. 使用use
enum Status {
    Rich,
    Poor,
}

// 为枚举实现 Display
impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Rich => write!(f, "Rich"),
            Status::Poor => write!(f, "Poor"),
        }
    }
}

enum Work {
    _996,
    _007,
}

// 4.C风格，具有显式辨别值

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 为枚举实现 Display
impl Display for Work {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Work::_007 => write!(f, "work_status: 996"),
            Work::_996 => write!(f, "work_status: 007"),
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('X');

    let pasted = WebEvent::Paste(String::from("hello"));

    // 从一个字符串切边中创建一个具有所有权的String
    let pasted2 = WebEvent::Paste("hello world".to_owned());

    let click = WebEvent::Click { x: 1, y: 2 };

    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    // 打印
    inspect(pressed);
    inspect(pasted);
    inspect(pasted2);
    inspect(click);
    inspect(load);
    inspect(unload);

    let opt = operations::ADD;
    match opt {
        operations::ADD | operations::SUB => {
            println!("使用别名")
        }
    }

    use Work::*;
    println!("{}", _007);
    println!("{}", _996);

    // 好像不用use 也可以

    let poor = Poor;
    println!("{}", poor);

    use Status::Rich;
    let rich = Rich;
    println!("{}", rich);

    // 使用use
    use Color::*;
    println!("Color Red is {}", Red as i32);
    println!("Color Red is {}", Green as i32);
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page not Load"),
        WebEvent::PageUnload => println!("page not Unload"),
        // 解构
        WebEvent::KeyPress(c) => println!("key pressed {}", c),
        WebEvent::Paste(s) => println!("paste {}", s),
        // 将 click 解构
        WebEvent::Click { x, y } => println!("click x={},y={}", x, y),
    }
}


```

## 常量
const：常量，没有固定的内存地址，将会在编译时被内联
即不管在哪里使用，在使用时都是直接拷贝这段数据到相关上下文中使用

```rust
use std::collections::HashMap;


// 不可改变的值
const LANGUAGE: i32 = 1;
//须显式指定数据类型。类型必须具有 'static生存期：程序初始化器(initializer)中的任何引用都必须具有 'static生存期。
const LANGUAGE_2: &str = "this is a string";
const LANGUAGE_3: &'static str = "this is a other string";
// 以上两种写法是一样的
const CONST4:i32 =1 + LANGUAGE;

// const array
const LEFT: [&'static str; 3] = ["Hello", "World", "!"];
// or
const LEFT2: &'static [&'static str] = &["Hello", "World", "!"];


// static 类似于 const ，但是它在程序中标识一个精确的内存位置
// 静态中的调用仅限于常量函数、元组结构和元组变体
lazy_static::lazy_static!{

    // 使用 ref 关键字的原因 是 ref的语义符合 lazy_static 的实际情况
    static ref STRING_CONST: String = String::from("STRING_CONST");
    static ref STRING_CONST2: String = String::from("STRING_CONST2");
    static ref MAP: HashMap<u32, u32> = HashMap::new();
}
static  STRING_CONST3: &str = "STRING_CONST";
static  STRING_CONST4: &'static str = "STRING_CONST2";
// 尝试使用 String ，但是不可行，因为String 在堆上
// static STRING_CONST3: &String = STRING_CONST.add_assign(STRING_CONST);

// 测试一下vec
static VEC:Vec<u8> = Vec::new();

// 不能使用
// static MAP: HashMap<u32, u32> = HashMap::new();



// 具有static生命周期的，可以是可变的变量（但是需要使用 static mut）
static ARRAY: [i32; 2] = [0; 2];

/// #### 离开一下
/// 
/// 
fn main() {
    println!("LANGUAGE：{}", LANGUAGE);
    println!("LANGUAGE_2:{}", LANGUAGE_2);
    println!("LANGUAGE_3:{}", LANGUAGE_3);
    println!("CONST4:{}", CONST4);
    println!("LEFT{:?}", LEFT);
    println!("LEFT2{:?}", LEFT2);
    println!("LEFT2{:?}", LEFT2[0]);

    // 不能修改变量
    // StringConst.push_str("Hello");
    println!("STRING_CONST：{}", *STRING_CONST);
    println!("STRING_CONST2：{}", *STRING_CONST2);
    println!("STRING_CONST3{}", STRING_CONST3);
    println!("STRING_CONST4{}", STRING_CONST4);

    println!("ARRAY：{:?}", ARRAY);
    println!("VEC{:?}", VEC);
    println!("MAP{:?}", *MAP);
}

```

## 变量遮蔽和作用域

```rust
fn main() {
    let v1 = 1;
    //先声明，再初始化
    let v3;

    //作用域冻结
    let mut mutable_var = 1212;
    {
        let v2 = 2;
        println!("v1:{},v2:{}", v1, v2);
        let v1 = 3;
        println!("v1:{},v2:{}", v1, v2);
        //初始化
        v3 = 4;

        // 可变变量被不可变变量遮蔽
        let mutable_var = mutable_var;
        // 报错，被冻结
        // mutable_var = 11;
    }
    // v2 被遮蔽
    // println!("v1:{},v2:{}", v1, v2);

    // 这里的值的1，因为 3被遮蔽了
    println!("v1:{}", v1);

    // 重新绑定
    let v1 = 'a';
    println!("v1:{}", v1);
    let v1 = 'b';
    println!("v1:{}", v1);

    println!("v3:{}", v3);

    // 冻结解除
    mutable_var = 1212;
    println!("mutable_var:{}", mutable_var);
}



```

## 类型别名

```rust
// `NanoSecond` 是 `u64` 的新名字。
type NanoSecond = u64;
type Inch = u64;

// 下面的属性可以屏蔽警告
// #[allow(non_camel_case_types)]
type u64_t = u64;
// 试一试 ^ 移除上面那个属性

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

```

## 类型转换

```rust
extern crate core;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Number2 {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

// TryFrom 用于易出错的转换
impl TryFrom<i32> for Number2 {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(())
        } else {
            Ok(Number2 { value })
        }
    }
}

// 实现该 trait 可以将 任何类型转换为字符串
impl ToString for Number2 {
    fn to_string(&self) -> String {
        format!("value:{}", self.value)
    }
}

fn main() {
    let num = Number::from(55);
    println!("{:?}", num);

    let intvar = 65;
    let number: Number = intvar.into();
    println!("{:?}", number);

    // TryFrom
    let oknum = Number2 { value: 9 };

    assert_eq!(Number2::try_from(9), Ok(oknum));
    assert_eq!(Number2::try_from(-1), Err(()));

    //TryInto
    let result: Result<Number2, ()> = 8i32.try_into();
    assert_eq!(result, Ok(Number2 { value: 8 }));
    let result: Result<Number2, ()> = (-32i32).try_into();
    assert_eq!(result, Err(()));

    // ToString
    let number3 = Number2 { value: 3 };
    println!("{}", number3.to_string());

    // parse
    let data: i32 = "5".parse().unwrap();
    println!("{}", data);
    let data2 = "121".parse::<i64>().unwrap();
    println!("{}", data2);
}

```

## 表达式

```rust

fn main() {
    let x = 1;
    // 代码块也是表达式，所以可以用作赋值，代码块中的最后一个表达式将赋值给适当的表达式，例如局部变量等
    // 但是如果代码块的最后一个表达式结尾有分号，那么返回值为()
    let y = {
        let local = 2;
        x + local
    };
    let z = {
        let local = 3;
        x + local;
    };
    println!("x:{},y:{},z:{:?}", x, y, z);
}

```

## if-else

```rust
fn main() {
    // if-else 是一个表达式，每个分支都需要返回相同的类型
    let n = 5;
    if n > 0 {
        println!("n>0");
    } else if n == 0 {
        println!("n==0");
    } else {
        println!("n>0");
    }

    let new_n = {
        if n == 0 {
            1
        } else {
            // 所有分支必须返回一样的类型
            n + 10
        }; // 这里不能加分号，交了的话，那么 该代码块中最后一个表达式返回为()
    }; // 这是一个表达式，需要加;
    println!("new_n:{:?}", new_n);
}

```

## loop

```rust
#![allow(unreachable_code)]

fn main() {
    let receiveInner = 'outer: loop {
        println!("Outer Hello World!");

        loop {
            println!("inner hello world");

            // 这里直接写break 是中断inner loop
            // break; // break inner loop

            // 但是如果再写一次就会报错，代码不可达，所以需要使用label 加标签
            // break 直接跳跃到 outer，并且将值返回给outer处的表达式
            break 'outer "loop return Value"; // break outer loop
        }
        println!("Outer Hello End!");
    };

    println!("End loop,receiveInner:{:?}", receiveInner);
}

```

## while

```rust

fn main() {
    let o = 15;

    let mut n = 1;
    while n < o + 1 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

```

## clap 命令行开发

```rust

use clap::Parser;

// 这个文档将作为 help的description
/// Simple program to greet a person
#[derive(Parser, Debug)]
// version 将打印版本号
// bin_name 可以修改显示的软件名称
// long_about = None 表示不显示长的 about
// struct 中的成员都会是 参数
// arg 标识为 option 参数
// 没标的是argument 参数
#[command(author, bin_name = "foo", version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

```

## 信号接收

```rust
use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<()> {
    // 接受ctrlc 信号
    let ctrl_c_events = ctrl_channel()?;
    // 周期性发出信号
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

```

### lazy_static
使用这个宏可以创建在运行时初始化的静态

可以使用外部crate 声明
```rust
#[macro_use]
extern crate lazy_static;
```
这样会将外部的标识符绑定到当前extern 所在的作用域，
如果放在根mod中，会自动出现在所有作用域
其中`#[macro_use]`是复用宏

```rust
use lazy_static;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
```


## iter and for in

```rust
fn main() {
    // n 的值将取[0,..15],开区间
    for n in 0..16 {
        if (n % 15) == 0 {
            println!("fizzbuzz")
        } else if (n % 3) == 0 {
            println!("fizz")
        } else if (n % 5) == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }
    }

    // n 的值将取[0,..16],闭区间
    for n in 0..=16 {
        if (n % 15) == 0 {
            println!("fizzbuzz")
        } else if (n % 3) == 0 {
            println!("fizz")
        } else if (n % 5) == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }
    }

    // for in 解构对于集合类型将会调用给into_iter 函数
    // 将其转换为一个迭代器，其他的方法有 iter 和iter_mut 函数。
    let vec_1: [&str; 3] = ["hello", "world", "!"];
    for v in vec_1.iter() {
        // iter 将会每次迭代时进行借用，集合不会改变所有权
        println!("item:{}", v);
    }
    println!("{:?}", vec_1);

    // into_iter 会消耗集合，每次迭代时，会提供集合本身的数据
    // 集合 内的数据会被move ，移动所有权到 iter
    // **array 不会被move**
    for v in vec_1.into_iter() {
        println!("item:{}", v);
        match v {
            "!" => println!("move !"),
            _ => println!("{}", v),
        }
    }
    println!("{:?}", vec_1);
    // 这里还能正常使用
    for v in vec_1.iter() {
        // iter 将会每次迭代时进行借用，集合不会改变所有权
        println!("item:{}", v);
    }

    // 对于这种类型为啥 会被move呢，想不通啊

    let names = vec!["Bob", "Frank", "Ferris"];
    for v in names.iter() {
        match v {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello :{}", v),
        }
    }
    for v in names.into_iter() {
        // 这里 用不用 & 都会被move
        match &v {

            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello :{}", v),
        }
    }

    // 此处的所有元素都被move了
    for v in names.iter() {
        match v {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello :{}", v),
        }
    }
}

```

## match enum

```rust
#[allow(dead_code)]
enum Color {
    Name,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

fn main() {
    let number = 13;

    // match 第一个匹配的分支被比对
    // 并且所有可能的值都必须被覆盖
    match number {
        // 匹配单个值
        1 => println!("One"),
        // 匹配多个值
        2 | 3 | 4 => println!("other:{}", number),
        // 匹配一个范围
        13..=19 => println!("区间:{}", number),
        // 处理别的情况
        // 该部分不能省略，也就是说，必须覆盖所有分支
        _ => println!("{}", number),
    }

    // match 解构元组
    let tuple = (0, -2, 3);
    match tuple {
        // 解构部分变量
        (0, x, y) => println!("{},{}", x, y),
        //省略部分变量
        // (.., -2, ..) => println!("1 start"),
        // 在 tuple中只能使用一次
        (-2, ..) => println!("1 start"),
        _ => println!("other"),
    }
    let rgb = Color::RGB(10, 10, 10);

    // match 枚举取值
    match rgb {
        Color::Name => println!("name"),
        Color::RGB(r, g, b) => println!("r:{},g:{},b:{}", r, g, b),
        Color::HSV(h, s, v) => println!("h:{},s:{},v:{}", h, s, v),
    }

}

```
## match ,ref mut & 
```rust
struct Refer<T1>(T1);
fn main() {
    println!("dereference &val");
    // match 解引用
    // 这里获得一个 &i32 类型，&标识取引用
    let refer1 = Refer(&4);
    match refer1.0 {
        // 以下两个都能匹配到
        val => println!("val{:?}", *val),
        &val => println!("&{:?}", val),
        _ => println!("_{:?}", refer1.0),
    }
    println!("dereference &val before  match");
    // 匹配前解引用
    // 那么就能匹配到 i32
    match *refer1.0 {
        val => println!("val{:?}", val),
        _ => println!("_{:?}", refer1.0),
    }
    println!("val not quote");
    // 一开始就不使用 引用
    let refer1 = Refer(4);
    match refer1.0 {
        // 以下两个都能匹配到
        val => println!("val{:?}", val),
        // 对于值来说，可以通过 ref val 获取 &i32
        ref val => println!("&{:?}", val),
        _ => println!("_{:?}", refer1.0),
    }
    println!("val use ref ");
    // 如果还想使用原来的代码，那么需要手动创建引用
    let ref data = 4;
    let refer1 = Refer(data);
    match refer1.0 {
        val => println!("val{:?}", val),
        _ => println!("_{:?}", refer1.0),
    }
    println!("val use ref  and *");
    // 匹配前解引用
    match *refer1.0 {
        val => println!("val{:?}", val),
        _ => println!("_{:?}", refer1.0),
    }

    println!("val use mut ");
    // 如果还想使用原来的代码，那么需要手动创建引用
    let mut data = 4;
    match data {
        //此处创建获取了引用，需要先解引用,并且这样写，就能修改到值
        ref mut m1 => {
            *m1 += 10;
            println!("ref mut val:{:?}", m1)
        }
        // match 时获取了值,但是这样写不会修改data
        mut m => {
            m += 10;
            println!(" mut val:{:?}", m)
        }
        val => println!("val{:?}", val),
        _ => println!("_{:?}", data),
    }
    println!("finish mut match:{:?}", data);
}

```


## 卫语句

```rust
fn main() {
    let pair = (2, -3);

    match pair {
        // if 条件部分是 卫语句 guard 用来过滤分支。
        (x, y) if x + y == 0 => println!("x:{},y:{}", x, y),
        _ => println!("{:?}", pair),
    }
}

```


## match 时进行绑定

```rust
fn main() {
    let age = 19;

    match age {
        0 => println!("error age"),
        // 范围匹配时 使用 s..=n 语法，使用这种方法进行的匹配无法
        // 没办法知道年龄
        1..=12 => println!("children"),
        //
        n @ 13..=18 => println!("age is {}", n),
        n => println!("@ match var for n ,value is {}", n),
        // _=> is  unreachable code
        _ => println!("unknow age"),
    }

    // 使用 绑定 解构

    // 解构 struct
    struct Refer<T1>(T1);
    let some = Refer(4);
    match some {
        Refer(a @ 4) => println!(" @ for {}", a),
        _ => println!("other"),
    }

    // 可以解构 enum
    fn some_number() -> Option<u32> {
        Some(42)
    }
    let some = some_number();
    match some {
        Some(a @ 42) => println!(" @ for {}", a),
        _ => println!("other"),
    }

    //再试试别的
    #[derive(Debug)]
    struct OtherStruct {
        x: i32,
        y: i32,
    }

    let s = OtherStruct { x: 3, y: 3 };
    println!("{:?}", s);
    match s {
        // 这样 可以匹配成功
        OtherStruct { x: 1, y: 2 } => println!("{:?}", s),

        // 并且可以省略呢
        OtherStruct { y: 2, .. } => println!("{:?}", s),

        // 再试试 绑定
        OtherStruct { y: m @ 3, .. } => println!("{:?}", m),
        _ => println!("other"),
    }

    println!("match tuple");
    // match 解构元组
    let tuple = (-3, -2, 3);
    match tuple {
        // 解构部分变量
        (0, x, y) => println!("{},{}", x, y),
        //省略部分变量
        // (.., -2, ..) => println!("1 start"),
        // 在 tuple中只能使用一次
        (-2, ..) => println!("1 start"),
        (x @ -3, ..) => println!("use @ start :{}", x),
        _ => println!("other"),
    }

    #[allow(dead_code)]
    enum Color {
        Name,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
    }
    let rgb = Color::RGB(11, 10, 10);

    // match 枚举取值
    match rgb {
        Color::Name => println!("name"),
        // 这里使用match 要求 r 需要是 11
        Color::RGB(r @ 11, g, b) => println!("@ r:{},g:{},b:{}", r, g, b),
        Color::RGB(r , g, b) => println!("r:{},g:{},b:{}", r, g, b),
        Color::HSV(h, s, v) => println!("h:{},s:{},v:{}", h, s, v),
    }
}

```


## if let 

```rust
fn main() {
    #[derive(PartialEq)]
    enum Foo {
        Bar,
        Car,
    }

    let a = Foo::Bar;

    // 这个枚举 没有 实现 #[derive(PartialEq)]，所以 == 时会出错
    if a == Foo::Bar {
        println!("match Bar");
    }

    #[derive(Debug)]
    enum Foo2 {
        Bar,
        Car,
    }

    let a = Foo2::Bar;
    // 这个枚举 没有 实现 #[derive(PartialEq)]，所以 == 时会出错
    // 如果 变量在前，就变成赋值了
    if let a = Foo2::Car {
        println!("match Bar {:?}", a);
    } else {
        println!("not match");
    }

    // 使用 if let 匹配成功
    let b = Foo2::Bar;
    if let Foo2::Car = b {
        println!("match Bar {:?}", b);
    } else {
        println!("not match {:?}", b);
    }

    // 试试结构体
    #[derive(Debug)]
    struct OtherStruct {
        x: i32,
        y: i32,
    }
    let s = OtherStruct { x: 3, y: 3 };

    // a @ 是 一个 pattern ，这么用会报错
    // 所以应该这么干
    if let OtherStruct { x: 4, y } = s {
        // 这里的x 是 无法使用的
        // println!("0st {:?}.x:{}", s, x);
        println!("0st {:?}", s);
    } else if let OtherStruct { x: a @ 3, y } = s {
        println!("1st {:?}.x :{}", s, a);
        // 这个为啥能用
    } else if let OtherStruct { y: 3, .. } = s {
        println!("2st {:?}", s);
    } else {
        println!("not match");
    }
}

```


## while let

```rust

fn main() {
    let mut optional = Some(1);

    while let Some(i) = optional {
        if i >= 9 {
            println!(">9, break");
            break;
        } else {
            optional = Some(i + 1);
            println!("other")
        }
    }
}


```

## 方法
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 方法是依附于对象的函数
// 这些函数通过关键字self来访问对象中的数据
// 方法在 impl代码块中定义
impl Point {
    // 这是一个静态方法，静态方法不需要被实例调用
    // 直接 对象::静态方法即可，一般用于 constructor
    fn constructor() -> Point {
        Point { x: 0, y: 0 }
    }

    // 另外一个new 函数，不知道有没有 重载
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn destroy(self){
        let Point { x, y } = self;
        println!("x:{},y:{}",x,y);
    }

    fn print(&self){
        let Point { x, y } = self;
        println!("x:{},y:{}",x,y);
    }
}

fn main() {
    let point = Point::new(1, 1);
    println!("{:?}", point);
    point.print();
    point.destroy();
    // 后面所有的point 都无法使用，因为所有权被move了
    // println!("{:?}", point);
}

```


## 闭包

```rust

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

```

## 闭包的捕获

```rust

use std::mem;

fn main() {
    // 闭包很灵活，可以自动适应用例
    //既可以 move又可以borrow
    // 闭包可以通过 {引用 &T}{可变引用 &mut T}{值 T}来捕获变量

    // 闭包优先通过引用捕获
    let color = String::from("red");
    println!("{}", color);
    // color = String::from("green");

    // 捕获变量的引用
    let print = || println!("{}", color);
    // 使用借用进行调用
    print();

    // 被不可变引用
    let reborrow = &color;
    print();
    println!("{}", reborrow);

    // 这里修改了变量的绑定，但是不影响哦
    let color = String::from("green");
    print();
    println!("{}", color);

    // 使用可变借用的闭包
    let mut count = 0;
    // 这里可以修改
    count = 50;
    // 当 inc 变化时，意味着闭包的内部发生变化
    let mut inc = || {
        count += 1;
        println!("{}", count);
    };
    inc();
    // 下一句：不能去修改count了，因为这里是一个借用
    // count = 100;
    inc();
    // 闭包中捕获了这个变量

    // 这时可以重新修改这个数据，因为后面没有再使用闭包了
    // 即闭包不再借用 &mut count 了
    count = 100;
    println!("{}", count);

    // rust 中，默认所有的值都是栈分配
    // 通过使用Box<T>,可以把这个值进行装箱，使其在堆上分配。类似于 cpp的
    // 智能析构指针；
    // 创建一个不可复制类型
    let moveable = Box::new(0);

    // 此时 这里面是指针
    // 对象不用move
    let consume = || {
        println!("moveable:{}", *moveable);
    };

    // 可以多次调用，因为现在闭包没有要求move
    consume();
    consume();
    // 此时 这里面是指针
    // 对象不用move
    let consume2 = || {
        println!("moveable:{}", *moveable);
        mem::drop(moveable);
    };

    // 可以多次调用，因为现在闭包没有要求move
    consume2();
    // 变量被move到闭包，并且被消耗了
    // consume2();

    // 对于之前的闭包，如果使用move ，可以强制闭包获取所有权

    let color = String::from("red");
    println!("{}", color);
    // color = String::from("green");

    // 捕获变量的引用
    let print = move || println!("{}", color);
    // 使用借用进行调用
    print();
    // 不再能使用 被move后的变量
    // println!("{}", color);
    print();
}

```


## 闭包作为函数 入参

```rust
// 该函数将闭包作为参数，并且进行调用
// 注意点：F必须是泛型。这是由闭包定义的实现方式决定过的
// 当 闭包被定义时，编译器会隐式创建一个匿名类型的结构体
// 该结构体用来存储闭包捕获的变量，同时为这个未知类型的结构体实现函数功能
// 通过 Fn,FnMut,FnOnce 这三种trait中的一种
// Fn、FnMut 和 FnOnce 这些 trait 明确了闭包如何从周围的作用域中捕获变量。
fn apply<F>(mut f: F)
// trait 约束
where
    // rust 中的闭包在函数中使用时，不允许模糊的写法
    // 需要指定闭包的完整类型
    // 其类型通过三种trait进行指定
    // Fn 表示捕获方式为通过引用（&T）的闭包
    // FnMut 表示捕获方式为通过引用（&mut T）的闭包
    // FnOnce 表示捕获方式为通过引用（T）的闭包
    // F: FnMut(),
    F: FnOnce(), // 使用更高级的FnOnce 和 FnMut 都能接到这种类型
{
    f();
}

// 这是一个普通的函数，但是满足 trait Fn限定
// 可以被 Fn,FnMut,FnOnce 这三种约束的函数作为参数调用
fn function() {
    println!("a funciton")
}

fn main() {
    // 函数将闭包作为参数调用

    let xx = "str";
    let mut color = String::from("red");

    let mut print = || {
        // 捕获该变量，需要 Fn
        println!("xx:{}", color);

        // 改变了 color，需要 FnMut
        color = String::from("green");
    };
    print();

    // 这里就无法使用 Fn 了，因为 print 是一个FnMut闭包，Fn接不到这种类型的
    apply(print);
    apply(function);
    // apply(print);

    // ** 编译器： 在满足使用需求的前提下尽量以限制最多的方式捕获。
    // 我可以指定需要once，但是这个函数可以接受限制更多的闭包，但是反之不行
}

```

## 闭包作为返回参数
```rust

// 使用 闭包作为输出，但是因为现在Rust只支持 返回具体的类型
// 所以 因为匿名闭包的类型是未知的，所以只有使用impl Trait 才能返回闭包
// 并且还需要使用move 将闭包引用的捕获进行move，否则这些引用将会在函数退出时被销毁
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("this a {}", text)
}
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("this a {}", text)
}
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("this a {}", text)
}

fn main() {
    create_fn()();
    create_fnmut()();
    create_fnonce()();
}


```


## 闭包的例子

```rust

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    // any 要求一个闭包，接受值，并且内部只能借用

    println!("2 in this ? {}", vec1.into_iter().any(|x| x == 2));
    // 要求值类型
    // println!("2 in this ? {}", vec1.into_iter().any(|&x| x == 2));

    // iter 中返回的时 [&i32] ,需要解构
    println!("2 in this ? {}", vec2.iter().any(|&x| x == 2));

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    let mut iter1 = vec1.iter();
    let mut iter2 = vec2.into_iter();

    let x = vec1.iter().find(|&&x| 2 == x);

    // find 的 self 是 &mut 的
    println!("2 in this ? {:?}", iter1.find(|&&i| i == 2));
    println!("2 in this ? {:?}", iter2.find(|&i| i == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}

```


## 高阶函数
输入一个或者多个函数，生成一个函数的函数
惰性求值和高阶函数支持函数式风格编程


```rust
fn main() {
    // 寻找所有数字的平方是奇数就累加
    let upper = 1000;
    // 循环 0到无穷大
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if if_odd(n_squared) {
            acc += 1;
        }
    }
    println!("find acc:{}", acc);

    // 函数式编程
    let find = (0..)
        .map(|n| n * n) // map成n*n
        .take_while(|&n| n <= upper) //取小于上限的
        .map(|n| if if_odd(n) { 1 } else { 0 }) // 筛选是 odd的
        // .map(|_| 1)
        .sum::<i32>();

    println!("find1:{}", find);

    let find = (0..)
        .map(|n| n * n) // map成n*n
        .take_while(|&n| n <= upper) //取小于上限的
        .filter(|&n| if_odd(n)) // 筛选是 odd的
        .map(|_| 1)
        .sum::<i32>();

    println!("find2:{}", find);

    let find = (0..)
        .map(|n| n * n) // map成n*n
        .take_while(|&n| n <= upper) //取小于上限的
        .filter(|&n| if_odd(n)) // 筛选是 odd的
        .collect::<Vec<i32>>()
        .len();

    println!("find3:{}", find);

    let find = (0..)
        .map(|n| n * n) // map成n*n
        .take_while(|&n| n <= upper) //取小于上限的
        .filter(|&n| if_odd(n)) // 筛选是 odd的
        .fold(0, |sum, i| sum + 1);

    println!("find3:{}", find);
}

// 判断是否为奇数
fn if_odd(n: i32) -> bool {
    n % 2 == 1
}

```


## 发散函数
永远不会返回的函数，标记为!

```rust

fn main() {
    fn sum_odd_function(upper: i32) -> i32 {
        let mut sum = 0;
        for i in 0..upper {
            let x = match i % 2 == 1 {
                true => i,
                false => continue,
            };

            sum += x;
        }
        sum
    }
    println!("sum :{}", sum_odd_function(100));
}

```

## 模块系统
将代码按层次分成逻辑单元，管理这些模块之间的 可见性
模块是 item的集合
item: 函数，结构体，trait，impl ，其他模块

### 模块的可见性
```rust
mod mode1 {
    pub fn funciton() {
        println!("function 1 on mode1")
    }

    pub mod nested {
        use super::{funciton_pub_in_path1, funciton_puh_self};

        pub fn funciton() {
            println!("function 2 on mode1::nestd")
        }

        pub fn funciton2() {
            funciton_pub_in_path1();
            funciton_pub_in_path2();

            // nested 无法使用 上一级 mod 的self item
            // 但是 可以通过 use::super::{} 语法引用该 itme 来使用
            funciton_puh_self();

            // 能使用吗?
            funciton_pub_super();
        }

        // pub(in path),只能在 nested 内访问
        pub(in crate::mode1::nested) fn funciton_pub_in_path2() {
            println!("funciton_pub_in_path2")
        }

        // 使用 pub(super),只能在 mode 的父级以内使用
        pub(super) fn funciton_pub_super() {
            println!("funciton_pub_super")
        }
    }

    // pub crate 项 可以在 同crate 的任何位置访问
    pub(crate) fn function_pub_crate() {
        println!("function_pub_crate")
    }

    pub fn funciton3() {
        funciton_pub_in_path1();

        // 无法使用在 nested中定义的 item,因为 当前不再 path内
        // funciton_pub_in_path2();

        // 可以使用 同为 self 的函数
        funciton_puh_self();

        // 父级内也能使用吗?
        nested::funciton_pub_super();
    }

    // pub(in path),只能在 mode1 内访问
    pub(in crate::mode1) fn funciton_pub_in_path1() {
        println!("funciton_pub_in_path1")
    }

    // 只能在当前 模块中可见
    pub(self) fn funciton_puh_self() {
        println!("funciton_puh_self")
    }
}

fn function() {
    println!("funciton 1")
}

fn main() {
    // 使用模块机制 可以消除同名歧义
    function();
    mode1::funciton();

    // 对于 pub item 都可在外部进行访问
    mode1::funciton();
    mode1::nested::funciton();

    // pub crate 可以在同crate 中的 任何位置访问
    mode1::function_pub_crate();

    // pub in path 只能在 path 内访问
    // 会报错
    // mode1::funciton_pub_in_path();

    // 通过子模块使用 in path item
    mode1::nested::funciton2();
    mode1::funciton3();

    // 只能在 其 父mod 中使用
    // mode1::nested::funciton_pub_super();

    //在局部作用域中使用use ,可以屏蔽 外部的同名函数
    {
        // 使用use 将一个完整的路径绑定到一个 新的名字
        use mode1::funciton;
        // 此处的function 是 我们的 use 的 function
        funciton();
    }
    // 退出作用域后,又返回到原作用域
    function();
}


```

### 结构体的可见性

```rust
mod test {
    // 这个 结构体 是公有的,有一个公有字段
    #[derive(Debug)]
    pub struct Struct1 {
        pub var1: String,
    }

    // 这个公有结构体,带有一个私有字段
    #[derive(Debug)]
    pub struct Struct2 {
        var1: String,
    }
    impl Struct2 {
        pub fn new(var1: String) -> Struct2 {
            Struct2 { var1: var1 }
        }
    }
}

fn main() {
    let v1 = test::Struct1 {
        var1: "12121".to_owned(),
    };
    println!("{:?}", v1);

    // 私有属性无法使用 名字:值的方式构造
    // let v2 = test::Struct2 {
    // var1: "12121".to_owned(),
    // };
    let v2 = test::Struct2::new("121211".to_owned());
    // 但是可以使用他的pub 的new 方法构造
    println!("{:?}", v2);
}

```


## 测试
```rust

// 使用该标注 表示这个集成测试
#[cfg(test)]
mod test {

    // 该属性说明它是一个测试函数
    #[test]
    //
    fn function() {
        // cargo test -- --show-output 使用这个option 可以输出 print
        println!("out put something");
        // 使用 assert 宏 进行 判断
        assert_eq!(2 + 2, 4)
    }
    #[test]
    #[ignore] // 通过这个属性可以忽略一个测试
              // 但是可以 通过 cargo test -- --ignored ，只运行被忽略的测试
    fn function1() {
        // 当发生panic 时，测试就会失败
        panic!("error")
    }
    #[test]
    fn function2() {
        let result = false;
        // 使用assert 提供更多的信息
        assert!(result, "Failed value was `{}`", result);
    }
    use super::other_need_panic;
    #[test]
    // 使用该属性，当测试函数抛出panic时测试成功
    #[should_panic]
    fn function3() {
        other_need_panic()
    }

    #[test]
    // 更精确的捕捉异常
    #[should_panic(expected = "panic")]
    fn function4() {
        other_need_panic()
    }
}

fn other_need_panic() {
    panic!("panic")
}

// cargo test -- --test-threads=1
// 通过 设置线程数可以并行运行单元测试
fn main() {}

```
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

```rust
// 不可改变的值
const LANGUAGE: i32 = 1;

static StringConst: &str = "STRINT";

// 具有static生命周期的，可以是可变的变量（但是需要使用 static mut）
static Array: [i32; 2] = [0; 2];

fn main() {
    println!("{}", LANGUAGE);

    // 不能修改变量
    // StringConst.push_str("Hello");
    println!("{}", StringConst);

    println!("{:?}", Array);
}

```

## 可变变量

```rust
fn main() {
    let immutable_var = 1;
    let mut mubtable_var = 1;

    // 编译器会报错
    mubtable_var += 1;
    // immutable_var += 1;
    println!("{}", immutable_var);
    println!("{}", mubtable_var);
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
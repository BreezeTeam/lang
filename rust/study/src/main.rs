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

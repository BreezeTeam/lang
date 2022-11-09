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

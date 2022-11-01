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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // 两个语句相等
    let ref refc1 = c;
    let refc2 = &c;
    println!("{}", *refc1 == *refc2);
    // ref 用于结构 结构体
    let point = Point { x: 0, y: 0 };
    let point_x_ref = {
        // ref_to_x 是一个指向point的 x 字段的引用
        let Point { x: ref ref_to_x, y: _ } = point;
        // 返回该引用
        *ref_to_x
    };
    println!("ref_to_x:{:?}", point_x_ref);
    // ref 创建可变引用
    let mut mut_point = point;
    {
        let Point { x: _, y: ref mut mut_ref_to_y } = mut_point;
        // 通过 可变引用修改y的值
        *mut_ref_to_y = 1;
    }
    println!("point is {:?}", point);
    println!("mut_point is {:?}", mut_point);

    // 将 ref 指针存储在 元组中
    let mut mut_tuple = (Box::new(5u32), 3u32);
    {
        let (ref mut first, ref mut second) = mut_tuple;
        *first = Box::new(*second);
        *second = 2u32;
    }
    println!("tuple is {:?}", mut_tuple);
}
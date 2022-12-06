fn borrow(b: &i32) {
    println!("borrow:{}", b);
}
fn moveinto(b: Box<i32>) {
    println!("destroying {}", b);
}

fn main() {
    let boxed = Box::new(5i32);
    let stack = 6i32;

    // 借用了内容，没有取得所有权
    borrow(&boxed);
    borrow(&stack);

    {
        // 取得一个引用
        let ref_to: &i32 = &boxed;

        // 当 引用被使用时，无法被销毁
        // moveinto(boxed);

        // 在 moveinto 后面使用了 其借用，那么所有权无法在moveinto中转移
        // 会报错
        borrow(ref_to);
        // ref_to 离开所用域后，该借用不存在，即box不再被借用
    }
    moveinto(boxed);
}

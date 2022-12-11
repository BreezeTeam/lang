#[derive(Debug)]
struct Person {
    age: u32,
    name: &'static str,
}

// 使用不可变借用
fn borrow_person(b: &person) {
    println!("borrow person:{:?}", b);
}

// 使用可变借用
fn mut_borrow_person(b: &mut person) {
    b.name = "new Name";
    println!("mut borrow person:{:?}", b);
}

fn borrow(b: &i32) {
    println!("borrow:{}", b);
}

fn moveinto(b: Box<i32>) {
    println!("destroying {}", b);
}

fn main() {
    let boxed = Box::new(5i32);
    let stack = 6i32;

    // 用于测试可变借用
    let immut_person = Person {
        age: 12,
        name: "name",
    };
    let mut mut_person = immut_person;


    // 借用了内容，没有取得所有权
    borrow(&boxed);
    borrow(&stack);

    // 不可变借用一个不可变对象
    borrow_person(&immut_person);
    // 不可变借用一个可变对象
    borrow_person(&mut_person);
    // 可变借用一个可变对象
    mut_borrow_person(&mut mut_person);

    // 无法借用一个不可变对象
    // mut_borrow_person(&mut immut_person);

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
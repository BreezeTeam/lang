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

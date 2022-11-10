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

// 将会为这个这个结构体，实现一个特别的trait
#[derive(Debug)]
struct XX(i32, i32);

// 不使用关联类型实现的trait
trait YY1<A, B> {
    fn eqal(&self, number1: &A, number2: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// 利用为XX实现上述trait
impl YY1<i32, i32> for XX {
    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }

    fn eqal(&self, number1: &i32, number2: &i32) -> bool {
        (&self.0 == number1 && &self.1 == number2)
    }
}

// 不使用关联类型实现一个泛型函数，会很麻烦
fn difference1<A, B, C>(yy: &C) -> i32
where
    C: YY1<A, B>,
{
    yy.last() - yy.first()
}

// 使用关联类型实现的另一种trait
trait YY2 {
    // trait YY1<A, B> 中的 A,B使用 type 定义在 trait内部
    type A;
    type B;

    // 使用关联类型
    fn eqal2(&self, number1: &Self::A, number2: &Self::B) -> bool;
    fn first2(&self) -> i32;
    fn last2(&self) -> i32;
}
/// 利用关联类型为 XX实现trait
impl YY2 for XX {
    type A = i32;
    type B = i32;
    fn first2(&self) -> i32 {
        self.0
    }

    fn last2(&self) -> i32 {
        self.1
    }

    fn eqal2(&self, number1: &i32, number2: &i32) -> bool {
        (&self.0 == number1 && &self.1 == number2)
    }
}
// 使用关联类型实现一个泛型函数，很方便
fn difference2<C: YY2>(yy: &C) -> i32 {
    yy.last2() - yy.first2()
}

fn main() {
    let number1 = 1;
    let number2 = 2;
    let xx = XX(number1, number2);
    println!("{:?}", xx);

    println!(
        "Does equal {} and {}: {}",
        &number1,
        &number2,
        xx.eqal(&number1, &number2)
    );

    println!("First number: {}", xx.first());
    println!("Last number: {}", xx.last());

    // 使用泛型trait 实现的一个泛型函数
    println!("The difference is: {}", difference1(&xx));

    // 使用关联类型实现的 泛型函数
    println!("The difference2 is: {}", difference2(&xx));
}

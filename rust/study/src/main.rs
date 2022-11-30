use std::fmt::Debug;
// where 约束
trait PrintWhereDebug {
    fn print_where_debug(self);
}

// 我们这里需要一个 where 从句，否则就要表达成 T:Debug
// 这样意思就不对
impl<T> PrintWhereDebug for T
where
    Option<T>: Debug,
{
    fn print_where_debug(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    vec.print_where_debug();
}

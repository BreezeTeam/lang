// 使用该标注 表示这个集成测试
#[cfg(test)]
mod test {

    // 该属性说明它是一个测试函数
    #[test]
    //
    fn function() {
        // cargo test -- --show-output 使用这个option 可以输出 print
        println!("out put something");
        // 使用 assert 宏 进行 判断
        assert_eq!(2 + 2, 4)
    }
    #[test]
    #[ignore] // 通过这个属性可以忽略一个测试
              // 但是可以 通过 cargo test -- --ignored ，只运行被忽略的测试
    fn function1() {
        // 当发生panic 时，测试就会失败
        panic!("error")
    }
    #[test]
    fn function2() {
        let result = false;
        // 使用assert 提供更多的信息
        assert!(result, "Failed value was `{}`", result);
    }
    use super::other_need_panic;
    #[test]
    // 使用该属性，当测试函数抛出panic时测试成功
    #[should_panic]
    fn function3() {
        other_need_panic()
    }

    #[test]
    // 更精确的捕捉异常
    #[should_panic(expected = "panic")]
    fn function4() {
        other_need_panic()
    }
}

fn other_need_panic() {
    panic!("panic")
}

// cargo test -- --test-threads=1
// 通过 设置线程数可以并行运行单元测试
fn main() {}

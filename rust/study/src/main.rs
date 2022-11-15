mod mode1 {
    pub fn funciton() {
        println!("function 1 on mode1")
    }

    pub mod nested {
        use super::{funciton_pub_in_path1, funciton_puh_self};

        pub fn funciton() {
            println!("function 2 on mode1::nestd")
        }

        pub fn funciton2() {
            funciton_pub_in_path1();
            funciton_pub_in_path2();

            // nested 无法使用 上一级 mod 的self item
            // 但是 可以通过 use::super::{} 语法引用该 itme 来使用
            funciton_puh_self();

            // 能使用吗?
            funciton_pub_super();
        }

        // pub(in path),只能在 nested 内访问
        pub(in crate::mode1::nested) fn funciton_pub_in_path2() {
            println!("funciton_pub_in_path2")
        }

        // 使用 pub(super),只能在 mode 的父级以内使用
        pub(super) fn funciton_pub_super() {
            println!("funciton_pub_super")
        }
    }

    // pub crate 项 可以在 同crate 的任何位置访问
    pub(crate) fn function_pub_crate() {
        println!("function_pub_crate")
    }

    pub fn funciton3() {
        funciton_pub_in_path1();

        // 无法使用在 nested中定义的 item,因为 当前不再 path内
        // funciton_pub_in_path2();

        // 可以使用 同为 self 的函数
        funciton_puh_self();

        // 父级内也能使用吗?
        nested::funciton_pub_super();
    }

    // pub(in path),只能在 mode1 内访问
    pub(in crate::mode1) fn funciton_pub_in_path1() {
        println!("funciton_pub_in_path1")
    }

    // 只能在当前 模块中可见
    pub(self) fn funciton_puh_self() {
        println!("funciton_puh_self")
    }
}

fn function() {
    println!("funciton 1")
}

fn main() {
    // 使用模块机制 可以消除同名歧义
    function();
    mode1::funciton();

    // 对于 pub item 都可在外部进行访问
    mode1::funciton();
    mode1::nested::funciton();

    // pub crate 可以在同crate 中的 任何位置访问
    mode1::function_pub_crate();

    // pub in path 只能在 path 内访问
    // 会报错
    // mode1::funciton_pub_in_path();

    // 通过子模块使用 in path item
    mode1::nested::funciton2();
    mode1::funciton3();

    // 只能在 其 父mod 中使用
    // mode1::nested::funciton_pub_super();

    //在局部作用域中使用use ,可以屏蔽 外部的同名函数
    {
        // 使用use 将一个完整的路径绑定到一个 新的名字
        use mode1::funciton;
        // 此处的function 是 我们的 use 的 function
        funciton();
    }
    // 退出作用域后,又返回到原作用域
    function();
}

extern crate core;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Test(i32, i32);

impl Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.real, self.imag)
    }
}

fn main() {
    let test = Test(1, 2);
    println!("Debug {:?}", test);
    println!("Display {}", test);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Debug {:?}", complex);
    println!("Display {}", complex);
}
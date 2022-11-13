fn main() {
    fn sum_odd_function(upper: i32) -> i32 {
        let mut sum = 0;
        for i in 0..upper {
            let x = match i % 2 == 1 {
                true => i,
                false => continue,
            };

            sum += x;
        }
        sum
    }
    println!("sum :{}", sum_odd_function(100));
}

fn isAdult(age: &Years) -> bool {
    age.0 >= 18i64
}
struct Years(i64);
impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 356)
    }
}
struct Days(i64);
impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 356)
    }
}

fn main() {
    // new type

    let age = Years(18);
    let age_days = age.to_days();
    println!("is adult:{:?}", isAdult(&age));
    println!("is adult:{:?}", isAdult(&age));
    println!("is adult:{:?}", isAdult(&age_days.to_years()));
}

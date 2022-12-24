trait getUsername {
    fn get(&self) -> String;
}

trait getAge {
    fn get(&self) -> u32;
}

struct User {
    usernames: String,
    age: u32,
}
impl getUsername for User {
    fn get(&self) -> String {
        self.usernames.clone()
    }
}

impl getAge for User {
    fn get(&self) -> u32 {
        self.age
    }
}

fn main() {
    let user = User {
        usernames: "username".to_owned(),
        age: 32,
    };

    let username = <User as getUsername>::get(&user);
    println!("{:?}", username);
    let age = <User as getAge>::get(&user);
    println!("{:?}", age);
}

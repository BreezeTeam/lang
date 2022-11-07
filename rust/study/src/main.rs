fn main() {
    let mut optional = Some(1);

    while let Some(i) = optional {
        if i >= 9 {
            println!(">9, break");
            break;
        } else {
            optional = Some(i + 1);
            println!("other")
        }
    }
}

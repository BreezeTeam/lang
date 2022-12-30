fn main() {
    struct X(i32);
    macro_rules! match_test {
        ($function_name:ident,$input:path,$some:path) => {
            fn $function_name(input: Option<i32>) -> $some {
                match input {
                    $input(x) => $some(x + 1),
                    _ => $some(0),
                }
            }
        };
    }
    macro_rules! match_test2 {
        ($function_name:ident,$input:path,$output:path) => {
            fn $function_name(input: Option<i32>) -> Result<(i32, $output), $output> {
                match input {
                    $input(x) => Ok((x - 1, $output(x + 1))),
                    _ => Err($output(0)),
                }
            }
        };
    }
    match_test!(test_macro, Some, X);
    test_macro(Some(11));
    match_test2!(test_macro2, Some, X);
    test_macro2(Some(11));
}

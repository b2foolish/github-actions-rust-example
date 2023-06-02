//`#[cfg(test)]
//`mod tests {
//`use super::*;

//`#[test]
//`fn double_is_doubling_number_2() {
//`assert_eq!(double(2), 4);
//`}

//`#[test]
//`fn triple_is_tripling_number_2() {
//`assert_eq!(triple(2), 6);
//`}
//`}

////dobule function is defined here
pub fn double(number: i32) -> i32 {
    number * 2
}

//define triple
pub fn triple(number: i32) -> i32 {
    number * 3
}

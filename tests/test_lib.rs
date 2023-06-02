use trust;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn double_is_doubling_number_2() {
        assert_eq!(trust::double(2), 4);
    }

    #[test]
    fn triple_is_tripling_number_2() {
        assert_eq!(trust::triple(2), 6);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let superlist = _second_list.is_empty()
        || _first_list
            .windows(_second_list.len())
            .any(|w| w == _second_list);
    let sublist = _first_list.is_empty()
        || _second_list
            .windows(_first_list.len())
            .any(|w| w == _first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

#[cfg(test)]
mod sublist_tests {
    use crate::sublist::{sublist, Comparison};

    #[test]
    fn empty_lists() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_list_within_non_empty_list() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn non_empty_list_contains_empty_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn list_equals_itself() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn different_lists() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[2, 3, 4];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn false_start() {
        let list_one: &[i32] = &[1, 2, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn consecutive() {
        let list_one: &[i32] = &[1, 1, 2];
        let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_at_start() {
        let list_one: &[i32] = &[0, 1, 2];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_in_middle() {
        let list_one: &[i32] = &[2, 3, 4];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_at_end() {
        let list_one: &[i32] = &[3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn at_start_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn in_middle_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn at_end_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_element_from_second_list() {
        let list_one: &[i32] = &[1, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn second_list_missing_element_from_first_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_additional_digits_from_second_list() {
        let list_one: &[i32] = &[1, 2];
        let list_two: &[i32] = &[1, 22];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn order_matters_to_a_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[3, 2, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn same_digits_but_different_numbers() {
        let list_one: &[i32] = &[1, 0, 1];
        let list_two: &[i32] = &[10, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
}

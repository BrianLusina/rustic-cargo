// find_largest finds the largest item in a given list of items
pub fn find_largest<T>(list: &[T]) -> &T
where
    T: PartialOrd + Sized,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[cfg(test)]
mod find_largest_tests {
    use crate::vectors::largest::find_largest;

    #[test]
    fn test_find_largest_number() {
        let number_list = vec![34, 50, 25, 100, 65];
        let expected = &100;
        let actual = find_largest(&number_list);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_largest_character() {
        let char_list = vec!['y', 'm', 'a', 'q'];

        let expected = &'y';
        let actual = find_largest(&char_list);
        assert_eq!(expected, actual);
    }
}

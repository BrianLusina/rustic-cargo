#[cfg(test)]
mod sort_colors_tests {
    use algorithms::twopointers::sortcolors::sort_colors::sort_colors;

    #[test]
    fn test_1() {
        let colors = &mut [1, 0, 2, 1, 2, 2];
        let expected = &mut [0, 1, 1, 2, 2, 2];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let colors = &mut [0,1,1,2,0,2,0,2,1,2];
        let expected = &mut [0,0,0,1,1,1,2,2,2,2];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let colors = &mut [0];
        let expected = &mut [0];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let colors = &mut [0,1,0];
        let expected = &mut [0,0,1];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let colors = &mut [1];
        let expected = &mut [1];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_6() {
        let colors = &mut [2,2];
        let expected = &mut [2,2];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_7() {
        let colors = &mut [1,1,0,2];
        let expected = &mut [0,1,1,2];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_8() {
        let colors = &mut [2,1,1,0,0];
        let expected = &mut [0,0,1,1,2];
        let actual = sort_colors(colors);
        assert_eq!(actual, expected);
    }
}

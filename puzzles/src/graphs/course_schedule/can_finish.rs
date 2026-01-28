use std::collections::HashMap;

const WHITE: u32 = 1;
const GRAY: u32 = 2;
const BLACK: u32 = 3;

pub fn can_finish(num_courses: u32, prerequisites: Vec<Vec<u32>>) -> bool {
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut color: HashMap<u32, u32> = HashMap::new();

    for prerequisite in prerequisites.iter() {
        let destination = prerequisite[0];
        let source = prerequisite[1];
        if adjacency_list.contains_key(&source) {
            adjacency_list.get_mut(&source).unwrap().push(destination);
        } else {
            adjacency_list.insert(source, vec![destination]);
        }
    }

    let mut is_possible = true;

    for i in 0..num_courses {
        color.insert(i, WHITE);
    }

    let dfs = |node: u32| {
        if !is_possible {
            return;
        }

        color.insert(node, GRAY);

        // if adjacency_list.contains_key(&node) {
        //     for neighbour in adjacency_list[&node] {
        //         if color[&neighbour] == WHITE {
        //             // dfs(neighbour)
        //         } else if color[&neighbour] == GRAY {
        //             is_possible = false
        //         }
        //     }
        // }

        color.insert(node, BLACK);
    };

    // for i in 0.. num_courses {
    // if color[&i] == WHITE {
    //     dfs(i);
    // }
    // }

    is_possible
}

// fn dfs(node: u32, is_possible: &mut bool, color: &mut HashMap<u32, u32>, adjacency_list: &mut HashMap<u32, Vec<u32>>) {
//     if *is_possible {
//         return
//     }
//
//     color.insert(node, GRAY);
//
//     if adjacency_list.contains_key(&node) {
//         for neighbour in adjacency_list[&node] {
//             if color[&neighbour] == WHITE {
//                 dfs(neighbour, is_possible, color, adjacency_list);
//             } else if color[&neighbour] == GRAY {
//                 is_possible = false;
//             }
//         }
//     }
//
//     color.insert(node, BLACK);
// }

#[cfg(test)]
mod can_finish_tests {
    use super::*;
    use parameterized::parameterized;

    struct TestCase {
        num_courses: u32,
        prerequisites: Vec<Vec<u32>>,
        expected: bool,
    }

    #[parameterized(
        test_case = {
            TestCase {
                num_courses: 2,
                prerequisites: vec![vec![1,0]],
                expected: true
            },
            TestCase {
                num_courses: 4,
                prerequisites: vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]],
                expected: true
            },
            TestCase {
                num_courses: 1,
                prerequisites: vec![],
                expected: true
            },
            TestCase {
                num_courses: 3,
                prerequisites: vec![vec![1,0], vec![2,1], vec![1,2]],
                expected: false,
            },
            TestCase {
                num_courses: 3,
                prerequisites: vec![vec![1,0], vec![2,1], vec![4,3]],
                expected: true,
            },
        },
    )]
    fn test_can_finish(test_case: TestCase) {
        let actual = can_finish(test_case.num_courses, test_case.prerequisites);
        assert_eq!(actual, test_case.expected);
    }
}

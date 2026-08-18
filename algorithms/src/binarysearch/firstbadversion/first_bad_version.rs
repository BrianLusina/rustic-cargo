// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
pub struct Solution {
    bad: i32,
}

impl Solution {
    pub fn new(bad: i32) -> Self {
        Self { bad }
    }

    fn is_bad_version(&self, version: i32) -> bool {
        version >= self.bad
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left <= right {
            let mid = left + (right - left) / 2;
            if self.is_bad_version(mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

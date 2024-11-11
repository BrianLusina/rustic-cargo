struct VecIter {
    vec: Vec<u32>,
    index: usize,
}

impl Iterator for VecIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            None
        } else {
            let res = Some(self.vec[self.index]);
            self.index += 1;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iterator::vec_iter::VecIter;

    #[test]
    fn iter_test() {
        let fibs = vec![0, 1, 1, 2, 3, 5, 8, 13];
        let iter = VecIter {
            vec: fibs,
            index: 0,
        };
        for x in iter {
            println!("{}", x);
        }
    }
}

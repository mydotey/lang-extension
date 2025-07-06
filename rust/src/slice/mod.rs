pub trait SliceExtention {
    fn lift(&mut self, from: usize, to: usize);
}

impl<T: PartialOrd> SliceExtention for [T] {
    fn lift(&mut self, from: usize, to: usize) {
        if from >= self.len() || to >= self.len() || from == to {
            return;
        }
        if from < to {
            for i in from..to {
                self.swap(i, i + 1);
            }
        } else {
            for i in (to..from).rev() {
                self.swap(i + 1, i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SliceExtention;
    #[test]
    fn test_lift() {
        let mut v = [1, 2, 3, 4, 5];
        v.lift(1, 3);
        assert_eq!(v, [1, 3, 4, 2, 5]);
        v.lift(3, 1);
        assert_eq!(v, [1, 2, 3, 4, 5]);
        v.lift(0, 4);
        assert_eq!(v, [2, 3, 4, 5, 1]);
        v.lift(4, 0);
        assert_eq!(v, [1, 2, 3, 4, 5]);

        v.lift(5, 0);
        assert_eq!(v, [1, 2, 3, 4, 5]);
        v.lift(0, 5);
        assert_eq!(v, [1, 2, 3, 4, 5]);
        v.lift(0, 0);
        assert_eq!(v, [1, 2, 3, 4, 5]);
        v.lift(3, 3);
        assert_eq!(v, [1, 2, 3, 4, 5]);
        v.lift(10, 11);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}

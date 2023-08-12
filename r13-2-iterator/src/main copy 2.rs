#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![4, 1, 0, 2, 5];
        let mut v1_iter = v1.iter();

        assert_eq!(Some(&4), v1_iter.next());
        assert_eq!(Some(&1), v1_iter.next());
        assert_eq!(Some(&0), v1_iter.next());
        assert_eq!(Some(&2), v1_iter.next());
        assert_eq!(Some(&5), v1_iter.next());
        assert_eq!(None, v1_iter.next());
    }
}

fn main() {}

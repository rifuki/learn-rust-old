#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demo() {
        let v1 = vec![5, 3, 0, 1];
        let mut v1_iter = v1.iter();

        assert_eq!(Some(&5), v1_iter.next());
    }
}
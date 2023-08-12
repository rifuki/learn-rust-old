#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![5, 5, 4, 3, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 20);
    }
}

fn main() {
}
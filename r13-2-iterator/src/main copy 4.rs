#[cfg(test)]
mod tests {
    #[test]
    fn collect() {
        let v1 = vec![5, 3, 2, 1];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![6, 4, 3, 2]);
    }
}

fn main() {
}

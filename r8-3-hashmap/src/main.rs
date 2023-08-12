#[allow(unused)]
fn main(){
    let bun = "hello world sekai world morning world hello good sekai de ichiban";
    
    let mut count_kotoba = std::collections::HashMap::new();

    for kotoba in bun.split_whitespace() {
        let count = count_kotoba.entry(kotoba).or_insert(0);
        *count += 1;
    }

    println!("{:?}", count_kotoba);
}
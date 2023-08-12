fn main(){
    let config_max = Some(3u64);

    if let Some(max) = config_max {
        println!("the maximum is: {}", max);
    }
}
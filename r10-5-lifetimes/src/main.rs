fn main(){
    let kotoba1 = String::from("lorem ipsum dolor sit amet, ");
    let kotoba2 = String::from("consectetur adipiscing elit. ");

    let longest = find_longest_kotoba(&kotoba1, kotoba2.as_str(), "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua");
    println!("the longest kotoba is: {}", longest);
}

fn find_longest_kotoba<'a, T>(x: &'a str, y: &'a str, z: T) -> &'a str 
where T: std::fmt::Display 
{
    println!("announcement: {}", z);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
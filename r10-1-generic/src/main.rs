fn main(){
    let vec_num1 = vec![3, 5, 1, 2, 8, 9, 6, 4, 10, 24, 353, 8394];
    println!("the largest vec_num1 is: {}", find_largest_in_vec(&vec_num1));
    let vec_chara1 = vec!['5', ' ', '0', 'a', 'x'];
    println!("the largest vec_chara1 is: {}", find_largest_in_vec(&vec_chara1));

}

fn find_largest_in_vec<T>(vecta: &[T]) -> T 
    where T: std::cmp::PartialOrd + Copy
{
    let mut largest = vecta[0];

    for &e in vecta {
        if e > largest {
            largest = e
        }
    }
    largest
}
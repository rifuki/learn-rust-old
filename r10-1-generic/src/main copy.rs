fn main(){
    let vec_num = std::vec::Vec::from([5, 1, 2, 3, 6, 7, 8, 9]);
    let mut largest_vec_num = vec_num[0];
    for number in vec_num {
        if number > largest_vec_num {
            largest_vec_num = number
        }
    }
    println!("largest_vec_num is: {}", largest_vec_num);

    let vec_char = vec!['a', 'z', 'c', 'b', 'd', 'x', 'y', 'w', 'p'];
    let mut largest_vec_char = vec_char[0];

    for chara in vec_char {
        if chara > largest_vec_char {
            largest_vec_char = chara
        }
    }
    println!("largest_vec_char is: {}", largest_vec_char);
}
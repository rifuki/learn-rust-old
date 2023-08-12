fn main(){
    let vec_num = Vec::from([5, 2, 3, 9, 6, 1, 4, 7, 8]);
    let largest = find_largest_num(&vec_num);
    println!("largest_value_in_vec_number is: {}", largest);

    let vec_chara = vec!['b', 'c', 'd', 'a', 'e', 'f', 'g', 'h', 'n', 'm', 'j', 'l', 'z', 'x', 'o'];
    let largest = find_largest_chara(&vec_chara);
    println!("largest_value_in_vec_chara is: {}", largest);
}

fn find_largest_num(vec_num: &[i32]) -> i32 {
    let mut largest = vec_num[0];
    for &num in vec_num {
        if num > largest {
            largest = num
        }
    }
    largest
}

fn find_largest_chara(vec_chara: &[char]) -> char {
    let mut largest = vec_chara[0];
    for &chara in vec_chara {
        if chara > largest {
            largest = chara
        }
    }
    largest
}
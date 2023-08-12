fn main() {
    let v1 = Vec::from([5, 3, 2, 1]);
    let third = v1[2];
    println!("third is {}", third);

    match v1.get(3) {
        Some(val) => println!("element found: {}", val),
        None => println!("element not found")
    }


    let mut v2 = Vec::from([2, 3, 4]);
    print!("v2: [");
    for i in &mut v2 {
        *i = *i + 10;
        print!(" {} ", i);
    }
    println!("]");

    #[derive(Debug)]
    enum Excel {
        Int(i32),
        Float(f32),
        Text(String)
    }
    
    let mut row = vec![Excel::Int(5), Excel::Float(3 as f32), Excel::Text(String::from("huhuhaha"))];
    for i in &mut row {
        match i {
            Excel::Int(val) => *val += 100,
            Excel::Float(val) => *val *= 2.5,
            Excel::Text(val) => val.push_str(" keqingggg")
        }
    }
    
    print!("row: [");
    for i in &row {
        match i {
            Excel::Int(val) => print!(" {} ", val),
            Excel::Float(val) => print!(" {} ", val),
            Excel::Text(val) => print!(" {} ", val)
        }
    }
    println!("]");
}
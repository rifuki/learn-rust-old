#[derive(Debug)]
enum ShoeStyle {
    Sneaker,
    Sandal,
    Boot,
}

struct Shoe {
    size: u32,
    style: ShoeStyle,
}

fn shoe_in_size(shoes: &Vec<Shoe>, my_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(| x | x.size == my_size).collect()
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: ShoeStyle::Sneaker,
        },
        Shoe {
            size: 13,
            style: ShoeStyle::Sandal,
        },
        Shoe {
            size: 10,
            style: ShoeStyle::Boot
        }
    ];

    let my_shoes = shoe_in_size(&shoes, 10);
    for shoe in my_shoes {
        println!("shoe: {:?} | size: {}", shoe.style, shoe.size);
    }
}

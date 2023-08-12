fn main(){
    let f = match std::fs::File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("problem when creating file: {}", error)
            },
            other_errors => panic!("problem when opening file: {}", other_errors)
        }
    };
}
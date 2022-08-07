use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other => panic!("Problem opening the file: {:?}", other),
    //     },
    // };

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    println!("{:?}", read_username_from_file());
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // f.read_to_string(&mut s)?;

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

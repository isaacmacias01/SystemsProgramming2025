use std::fs::File;
use std::io::{self,Read};

// fn read_username_from_file() -> Result<String, io::Error> {

//     let f = File::open("username.txt"); // opening file may fail

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e), // same as calling manager. Error propagation 
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file_2ver() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?; // ? is a shortcut for above code
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

fn read_username_from_file_3ver() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let r = read_username_from_file_3ver();

    println!("Hey even if fn fails, I'm still working");
    println!("{:?}",r.unwrap());
}
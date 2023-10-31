use std::{path::PathBuf, error::Error};

pub fn quit(code: i32) {
    std::process::exit(code);
}

pub fn pause() -> Result<u32, Box<dyn Error>> {
    println!("Pausing! Press enter to continue, q to quit...");
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    //println!("{buffer}");
    let res_c = buffer.chars().collect::<Vec<char>>()[0];
    if res_c.is_ascii_digit() {
        let res_u = res_c.to_digit(10).unwrap();
        println!("{}", res_u);
        Ok(res_u - 1)
    } else {
        quit(0);
        Err("asdf".into())
    }
    
}

#[derive(Debug)]
pub struct Item {
    pub name: PathBuf,
    pub is_dir: bool,
}
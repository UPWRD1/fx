use std::{env, fs};

pub mod extra;
use extra::*;

fn printfiles() -> Vec<Item> {
    print!("{esc}c", esc = 27 as char);
    let paths = fs::read_dir(env::current_dir().unwrap()).unwrap();
    let mut sdirs: String = "Files: \n".to_string();
    let mut index = 1;
    let mut currentdir: Vec<Item> = vec![];
    for path in paths {
        let newpath = path.as_ref().unwrap().path();
        let new_i: Item = Item {
            name: newpath.to_owned().to_path_buf().file_name().unwrap().into(),
            is_dir: path.as_ref().unwrap().path().is_dir(),
        };
        match &new_i.is_dir {
            true => {
                sdirs = format!("{}{}: /{}\n", sdirs, index, &new_i.name.clone().display());
                index += 1;
                let _ = currentdir.push(new_i).to_owned();
            }
            false => {
                sdirs = format!("{}{}: {}\n", sdirs, index, &new_i.name.clone().display());
                index += 1;
                let _ = currentdir.push(new_i).to_owned();
            }
        }
    }
    println!("{}", sdirs);
    currentdir
}

fn explore(curr: Vec<Item>, choice: usize) {
    if curr[choice].is_dir {
        let new_dir = &curr[choice].name;
        if let Err(e) = env::set_current_dir(new_dir) {
            println!("{e}");
            quit(1);
        } else {
            println!("Switching to {}", new_dir.display());
        }
    } else {
        let filec = fs::read_to_string(&curr[choice].name);
        println!("{}", filec.unwrap());
        let _ = pause();
    }
}

fn mainloop() {
    loop {
        let current: Vec<Item> = printfiles();
        let choice = pause().unwrap();
        println!("{}", choice);
        explore(current, choice.try_into().unwrap());
    }
}

fn main() {
    mainloop();
}

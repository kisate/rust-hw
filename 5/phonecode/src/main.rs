use std::path::Path;
use std::{collections::HashMap};
use std::fs::{self, File};
use std::io::{self, BufRead};

use phonecode::code_seq;
use phonecode::trie::{Node, add_word};

fn read_dict(path: &str) -> HashMap<char, u8> {
    let mut dict = HashMap::new();
    let contents = fs::read_to_string(path).expect("Bad file");

    for line in contents.lines() {
        let splitted: Vec<&str> = line.split(" ").collect();
        dict.insert(splitted[0].to_string().chars().next().unwrap(), splitted[1].parse::<u8>().unwrap());
    }

    dict 
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let dict = read_dict("dict.txt");

    let mut root: phonecode::trie::Node = Node::default();

    if let Ok(lines) = read_lines("dictionary.txt") {
        for line in lines {
            if let Ok(ip) = line {
                add_word(&ip, &mut root, &dict);
            }
        }
    }

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let seq = ip.chars().filter(|c| c.is_numeric()).map(|c| c.to_string().parse().unwrap()).collect::<Vec<u8>>();
                for answ in code_seq(&root, &seq, false) {
                    println!("{}: {}", ip, answ)
                }
            }
        }
    }
}

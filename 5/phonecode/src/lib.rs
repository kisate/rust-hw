use std::{collections::HashMap};

use trie::{add_word, Node};

pub mod trie;

fn build_trie(words: &[String], dict: HashMap<char, u8>) -> Node {
    let mut node = Node::default();
    for word in words.iter() {
        add_word(word, &mut node, &dict)
    }

    node
}

fn collect_answers(root: &Node, seq: &[u8], use_number: bool) -> Vec<String> {
    // println!("{:?}", &seq);
    let mut ans = vec![];
    let mut node = &root.clone();
    for (i, x) in seq.iter().enumerate() {
        match &node.next[*x as usize] {
            Some(next_node) => {
                for word in node.words.iter() {
                    let part: Vec<String> = code_seq(root, &seq[i+1..], use_number)
                        .iter()
                        .map(|s| format!("{} {}", word, s))
                        .collect();
                    ans.extend(part);
                    println!("{:?} {:?}", next_node.words, ans);
                }
                node = next_node;
            }
            None => break,
        }
    }
    ans
}

pub fn code_seq(root: &Node, seq: &[u8], use_number: bool) -> Vec<String> {
    let mut ans = collect_answers(root, seq, true);
    if ans.is_empty() && use_number {
        ans = collect_answers(root, &seq[1..], false).iter().map(|word| format!("{} {}", seq[0], word)).collect()
    }
    ans
}

#[cfg(test)]
mod tests {
    use std::{fs, io::BufRead};

    use super::*;

    fn transform(old: &HashMap<u8, Vec<char>>) -> HashMap<char, u8> {
        let mut new_map: HashMap<char, u8> = HashMap::new();
    
        for (key, vals) in old {
            for val in vals {
                new_map.insert(val.to_ascii_lowercase(), *key);
            }
        }
    
        new_map
    }

    fn test_data() -> HashMap<char, u8> {
        let mut data = HashMap::new();
        data.insert(0, vec!['e']);
        data.insert(1, vec!['j', 'n', 'q']);
        data.insert(2, vec!['r', 'w', 'x']);
        data.insert(3, vec!['d', 's', 'y']);
        data.insert(4, vec!['f', 't']);
        data.insert(5, vec!['a', 'm']);
        data.insert(6, vec!['c', 'i', 'v']);
        data.insert(7, vec!['b', 'k', 'u']);
        data.insert(8, vec!['l', 'o', 'p']);
        data.insert(9, vec!['g', 'h', 'z']);
        transform( &data)
    }

    #[test]
    fn test_encoding() {
        let dict = test_data();
        let mut root = Node::default();

        for line in fs::read_to_string("test_dictionary.txt").expect("Bad file").lines() {
            add_word(&line.to_string(), &mut root, &dict);
        }

        let number: [u8; 6] = [5,6,2,4,8,2];

        let result = code_seq(&root, &number, true);
        println!("{:?}", result);
    }
}
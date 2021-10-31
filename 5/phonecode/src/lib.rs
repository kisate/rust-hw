use trie::Node;

pub mod trie;

fn collect_answers(root: &Node, mut seq: &[u8], use_number: bool) -> Vec<String> {

    let mut ans = vec![];
    let mut node = &root.clone();
    for x in seq.clone().iter() {
        match &node.next[*x as usize] {
            Some(next_node) => {
                if !seq.is_empty() {
                    seq = &seq[1..];
                    for word in next_node.words.iter() {
                        let part: Vec<String> = code_seq(root, seq, use_number)
                            .iter()
                            .map(|s| format!("{} {}", word, s))
                            .collect();
                        ans.extend(part);
                    }
                    node = next_node;
                }
            }
            None => break,
        }
    }
    if seq.is_empty(){
        ans.extend(node.words.clone());
    }
    ans
}

pub fn code_seq(root: &Node, seq: &[u8], use_number: bool) -> Vec<String> {
    let mut ans = collect_answers(root, seq, true);
    if ans.is_empty() && use_number && seq.len() == 1 {
        ans = vec![seq[0].to_string()];
    }
    else if ans.is_empty() && use_number && !seq.is_empty() {
        ans = collect_answers(root, &seq[1..], false).iter().map(|word| format!("{} {}", seq[0], word)).collect()
    }
    ans
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::trie::add_word;

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
        assert_eq!(result, ["mir Tor", "Mix Tor"]);
        
        let mut root2 = Node::default();

        for line in fs::read_to_string("dictionary.txt").expect("Bad file").lines() {
            add_word(&line.to_string(), &mut root2, &dict);
        }

        let number2: [u8; 8] = [8,8,5,6,3,5,3,8];
        let result = code_seq(&root2, &number2, true);
        assert_eq!(result, ["O\"l Midas 8", "Po Midas 8", "Opa 6 da so"]);
    }
}
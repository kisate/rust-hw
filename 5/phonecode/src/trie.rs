use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Node {
    pub next: [Option<Box<Node>>; 10],
    pub words: Vec<String>,
}

impl Node {
    fn add_seq(&mut self, seq: &[u8]) -> &mut Node {
        if seq.is_empty() {
            return self;
        }
        let number = seq[0];
        if self.next[number as usize].is_none() {
            self.next[number as usize] = Some(Box::new(Node::default()));
        }

        self.next[number as usize]
            .as_mut()
            .unwrap()
            .add_seq(&seq[1..])
    }

    pub fn search_seq<'a>(&'a self, seq: &[u8]) -> Box<dyn Iterator<Item = &String> + 'a> {
        let it = Box::new(self.words.iter());
        if seq.is_empty() {
            return it;
        }
        match &self.next[seq[0] as usize] {
            Some(node) => Box::new(it.chain(node.search_seq(&seq[1..]))),
            None => it,
        }
    }
}

pub fn add_word(word: &String, node: &mut Node, dict: &HashMap<char, u8>) {
    let seq: Vec<u8> = word
        .to_lowercase()
        .chars()
        .filter(|c| !"\"-".contains(c.clone()))
        .map(|c| dict[&c])
        .collect();

    node.add_seq(&seq).words.push(word.to_string());
}   

use std::collections::HashMap;

pub fn transform(old: &HashMap<u32, Vec<char>>) -> HashMap<char, u32> {
    let mut new_map:HashMap<char, u32> = HashMap::new();
    
    for (key, vals) in old {
        for val in vals {
            new_map.insert(val.to_ascii_lowercase(), *key);
        }
    }

    new_map
}

pub fn score(input: &str, score_table: &HashMap<char, u32>) -> u32 {
    input.chars().map(|x| -> u32 {*score_table.get(&x.to_ascii_lowercase()).unwrap_or(&0)}).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> HashMap<u32, Vec<char>> {
        let mut data = HashMap::new();
        data.insert(1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']);
        data.insert(2, vec!['D', 'G']);
        data.insert(3, vec!['B', 'C', 'M', 'P']);
        data.insert(4, vec!['F', 'H', 'V', 'W', 'Y']);
        data.insert(5, vec!['K']);
        data.insert(8, vec!['J', 'X']);
        data.insert(10, vec!['Q', 'Z']);
        data
    }

    #[test]
    fn test_transform() {
        let data = test_data();

        let new_data = transform(&data);
        assert_eq!(new_data[&'a'], 1);
        assert_eq!(new_data[&'b'], 3);
        assert_eq!(new_data[&'c'], 3);
        assert_eq!(new_data[&'q'], 10);
        assert_eq!(new_data[&'k'], 5);
        assert_eq!(new_data[&'t'], 1);
        assert_eq!(new_data.len(), 26);
    }

    #[test]
    fn test_score() {
        let input = "The quick brown fox jumps over the lazy dog";
        assert_eq!(score(&input, &transform(&test_data())), 99);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Comparison {
    Equal,     // список `a` равен списку `b`
    Sublist,   // список `a` является подсписком `b`
    Superlist, // список `b` является подсписком `a`
    Other,     // списки не равны и не являются подсписками друг друга
}

fn compare<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() > b.len() {
        for i in 0..a.len() - b.len() + 1 {
            if compare(&a[i..i + b.len()], b) == Comparison::Equal {
                return Comparison::Superlist;
            }
        }
        Comparison::Other
    } else if a.len() == b.len() {
        if a == b {
            Comparison::Equal
        } else {
            Comparison::Other
        }
    } else {
        if compare(b, a) == Comparison::Superlist {
            Comparison::Sublist
        } else {
            Comparison::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(compare(&[1, 2, 3], &[1, 2, 3]), Comparison::Equal);
        assert_eq!(compare(&[], &[1, 2, 3]), Comparison::Sublist);
        assert_eq!(compare(&[1, 2, 3], &[]), Comparison::Superlist);
        assert_eq!(compare(&[1, 2], &[1, 2, 3]), Comparison::Sublist);
        assert_eq!(compare(&[1, 2, 3], &[1, 2]), Comparison::Superlist);
        assert_eq!(compare(&[1, 2, 3, 4, 5], &[2, 3, 4]), Comparison::Superlist);
        assert_eq!(compare(&[1, 2, 3, 4, 5], &[3, 4, 5]), Comparison::Superlist);
        assert_eq!(compare(&[1, 2, 3, 4, 5], &[2, 4, 5]), Comparison::Other);
        assert_eq!(compare(&[2, 3], &[1, 2, 3, 4, 5]), Comparison::Sublist);
    }
}

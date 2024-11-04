// For a given string generate all possible unique permutations with letters of that string.

use std::{cell::Cell, collections::HashMap};

pub fn permute_unique_recurse(v: &mut Vec<Box<str>>, h: &HashMap<char, Cell<u8>>, s: &mut String) {
    if s.chars().count() == h.len() {
        v.push(Box::from(s.as_str()));
        return;
    }

    for (key, val) in h {
        if val.get() > 0 {
            s.push(*key);
            val.replace(val.get() - 1);
            permute_unique_recurse(v, h, s);
            val.replace(val.get() + 1);
            s.pop();
        }
    }
}

pub fn permute_unique(s: &str) -> Vec<Box<str>> {
    let mut h = HashMap::<char, Cell<u8>>::new();
    for c in s.chars() {
        h.entry(c)
            .and_modify(|v| {
                v.replace(v.get() + 1);
            })
            .or_insert(Cell::new(1));
    }

    // factorial calculation
    let n = if s.chars().count() == 0 || s.chars().count() == 1 {
        1
    } else {
        (2..=s.chars().count()).product()
    };
    let mut v = Vec::<Box<str>>::with_capacity(n);

    let mut s = "".to_string();
    permute_unique_recurse(&mut v, &h, &mut s);

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut x = permute_unique("abc");
        x.sort_unstable();
        assert_eq!(
            x,
            [
                Box::from("abc"),
                Box::from("acb"),
                Box::from("bac"),
                Box::from("bca"),
                Box::from("cab"),
                Box::from("cba"),
            ]
        );
    }
}

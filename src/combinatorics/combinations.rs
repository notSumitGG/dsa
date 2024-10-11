fn generate_all_combinations(v: &mut Vec<Box<str>>, s: &[char], i: usize, t: &mut String) {
    if i == s.len() {
        v.push(Box::from(t.as_str()));
        return;
    }

    t.push(s[i]);
    generate_all_combinations(v, s, i + 1, t);
    t.pop();
    generate_all_combinations(v, s, i + 1, t);
}

pub fn all_combinations(s: &str) -> Vec<Box<str>> {
    let mut v = Vec::with_capacity(usize::pow(2, s.len() as u32));
    let mut ch_list = Vec::with_capacity(s.len());

    for c in s.chars() {
        ch_list.push(c);
    }

    let mut t = "".to_string();
    generate_all_combinations(&mut v, &ch_list, 0, &mut t);

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            all_combinations("abc"),
            [
                Box::from("abc"),
                Box::from("ab"),
                Box::from("ac"),
                Box::from("a"),
                Box::from("bc"),
                Box::from("b"),
                Box::from("c"),
                Box::from("")
            ]
        );
    }
}

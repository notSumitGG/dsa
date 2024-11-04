// For a given string generate all possible combinations with letters of that string

fn combine_recurse(v: &mut Vec<Box<str>>, l: &[char], i: usize, s: &mut String) {
    if i == l.len() {
        v.push(Box::from(s.as_str()));
        return;
    }

    s.push(l[i]);
    combine_recurse(v, l, i + 1, s);
    s.pop();
    combine_recurse(v, l, i + 1, s);
}

pub fn combine(s: &str) -> Vec<Box<str>> {
    let mut v = Vec::with_capacity(usize::pow(2, s.len() as u32));
    let mut ch_list = Vec::with_capacity(s.len());

    for c in s.chars() {
        ch_list.push(c);
    }

    let mut t = "".to_string();
    combine_recurse(&mut v, &ch_list, 0, &mut t);

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            combine("abc"),
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

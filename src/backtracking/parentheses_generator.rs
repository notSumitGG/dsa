fn generate(current: &str, opened: usize, closed: usize, n: usize, result: &mut Vec<String>) {
    if current.len() == (n * 2) {
        result.push(current.to_string());
        return;
    }

    if opened < n {
        let new_str = current.to_string() + "(";
        generate(&new_str, opened + 1, closed, n, result);
    }

    if closed < opened {
        let new_str = current.to_string() + ")";
        generate(&new_str, opened, closed + 1, n, result);
    }
}

pub fn generate_parentheses(n: usize) -> Vec<String> {
    // the length of the vector is based on catalan numbers
    let l = ((n as u128 + 2..=2 * n as u128).product::<u128>() / (2..=n as u128).product::<u128>())
        as usize;
    let mut result = Vec::with_capacity(l);
    if n > 0 {
        generate("", 0, 0, n, &mut result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(generate_parentheses(1), vec!["()"]);
    }

    #[test]
    fn test2() {
        assert_eq!(generate_parentheses(2), vec!["(())", "()()"]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            generate_parentheses(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            generate_parentheses(4),
            vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            generate_parentheses(5),
            vec![
                "((((()))))",
                "(((()())))",
                "(((())()))",
                "(((()))())",
                "(((())))()",
                "((()(())))",
                "((()()()))",
                "((()())())",
                "((()()))()",
                "((())(()))",
                "((())()())",
                "((())())()",
                "((()))(())",
                "((()))()()",
                "(()((())))",
                "(()(()()))",
                "(()(())())",
                "(()(()))()",
                "(()()(()))",
                "(()()()())",
                "(()()())()",
                "(()())(())",
                "(()())()()",
                "(())((()))",
                "(())(()())",
                "(())(())()",
                "(())()(())",
                "(())()()()",
                "()(((())))",
                "()((()()))",
                "()((())())",
                "()((()))()",
                "()(()(()))",
                "()(()()())",
                "()(()())()",
                "()(())(())",
                "()(())()()",
                "()()((()))",
                "()()(()())",
                "()()(())()",
                "()()()(())",
                "()()()()()"
            ]
        );
    }

    #[test]
    fn test6() {
        assert_eq!(generate_parentheses(6).len(), 132);
    }
}

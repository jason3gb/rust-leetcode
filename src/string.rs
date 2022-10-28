use std::collections::{HashMap, VecDeque};

pub fn valid_parentheses(s: String) -> bool {
    let s_vec: Vec<char> = s.chars().collect();
    let mut stack = VecDeque::new();

    let bracket_map = HashMap::from([(')','('), ('}','{'),(']','[')]);

    for c in s_vec.iter() {
        if let Some(right) = bracket_map.get(c) {
            if let Some(left) = stack.pop_back() {
                if left != right {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            stack.push_back(c);
        }
    }

    return stack.len() == 0
}

pub fn generate_parentheses(n: i32) -> Vec<String> {
    let mut res = Vec::new();

    fn helper (left: i32, right: i32, s: String, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            res.push(s);
            return;
        }

        if left > 0 {
            helper(left-1, right+1, s.clone() + "(", res);
        }

        if right > 0 {
            helper(left, right-1, s.clone() + ")", res);
        }
    }

    helper(n, 0, "".to_string(), &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        let valid = "{}()[]".to_string();
        let invalid = "{}([]".to_string();

        assert_eq!(valid_parentheses(valid), true);
        assert_eq!(valid_parentheses(invalid), true)
    }

    #[test]
    fn test_generate_parenthese() {
        assert_eq!(generate_parentheses(1), vec!["()".to_string()]);
        assert_eq!(generate_parentheses(2).sort(),
                   vec!["()()".to_string(), "(())".to_string()].sort());
    }
}




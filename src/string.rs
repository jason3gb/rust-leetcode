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
}
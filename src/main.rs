use std::collections::HashMap;

fn main() {
    let test_cases = vec![
        ("hello", "olleh", true),
        ("node", "deno", true),
        ("dono", "oden", false),
        ("kasur", "rusak", true),
    ];

    for (i, (str_1, str_2, expected)) in test_cases.iter().enumerate() {
        let result = is_anagram(&String::from(*str_1), &String::from(*str_2));
        println!(
            "TEST {} str_1={} .. str_2={} => {} (expected={}) ===> correct : {}",
            i,
            str_1,
            str_2,
            result,
            expected,
            result == *expected
        );
    }
}

fn is_anagram(str_1: &String, str_2: &String) -> bool {
    let mut chars_1: HashMap<&str, u32> = HashMap::new();
    let mut chars_2: HashMap<&str, u32> = HashMap::new();

    if str_1.len() != str_2.len() {
        return false;
    }

    for str in str_1.split("").into_iter() {
        match chars_1.get(str) {
            Some(current) => chars_1.insert(str, current + 1),
            None => chars_1.insert(str, 1),
        };
    }

    for str in str_2.split("").into_iter() {
        match chars_2.get(str) {
            Some(current) => chars_2.insert(str, current + 1),
            None => chars_2.insert(str, 1),
        };
    }

    if chars_1.len() != chars_2.len() {
        return false;
    }

    for (key, value) in chars_1.iter() {
        match chars_2.get(key) {
            Some(value_2) => {
                if value != value_2 {
                    return false;
                }
            }
            None => return false,
        }
    }

    return true;
}

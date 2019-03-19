use itertools::Itertools;
use std::collections::HashMap;

fn get_id_checksum(id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();

    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let found_two = counts.iter().any(|(_, &v)| v == 2);
    let found_three = counts.iter().any(|(_, &v)| v == 3);
    return (found_two, found_three);
}

/// Given a list of strings, returns the checksum of this list
pub fn get_box_list_checksum(ids: Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let (two, three) = get_id_checksum(&id);

        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }

    return twos * threes;
}

fn find_matching_words<'a>(ids: Vec<&'a str>) -> Option<(&'a str, &'a str)> {
    fn check_matching_words(one: &str, two: &str) -> bool {
        let matching = one
            .chars()
            .zip(two.chars())
            .filter(|&(a, b)| a == b)
            .count();
        return matching == one.chars().count() - 1;
    }

    for (one, two) in ids.iter().tuple_combinations::<(_, _)>() {
        if check_matching_words(one, two) {
            return Some((&one, &two));
        }
    }

    return None;
}

/*
pub fn get_common_letters(ids: &[String]) -> &String {
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    struct ChecksumTest<'a> {
        id: &'a str,
        result: (bool, bool),
    }

    #[test]
    fn test_get_id_checksum() {
        let tests = vec![
            ChecksumTest {
                id: "abcdef",
                result: (false, false),
            },
            ChecksumTest {
                id: "bababc",
                result: (true, true),
            },
            ChecksumTest {
                id: "abbcde",
                result: (true, false),
            },
            ChecksumTest {
                id: "abcccd",
                result: (false, true),
            },
            ChecksumTest {
                id: "aabcdd",
                result: (true, false),
            },
            ChecksumTest {
                id: "abcdee",
                result: (true, false),
            },
            ChecksumTest {
                id: "ababab",
                result: (false, true),
            },
        ];

        for test in &tests {
            assert_eq!(test.result, get_id_checksum(&test.id.to_owned()));
        }
    }

    #[test]
    fn test_get_box_list_checksum() {
        let boxes = vec![
            "abcdef".to_owned(),
            "bababc".to_owned(),
            "abbcde".to_owned(),
            "abcccd".to_owned(),
            "aabcdd".to_owned(),
            "abcdee".to_owned(),
            "ababab".to_owned(),
        ];

        assert_eq!(12, get_box_list_checksum(boxes));
    }

    #[test]
    fn test_find_matching_words() {
        let boxes = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        let result = find_matching_words(boxes);

        assert!(result.is_some());

        let (first, second) = result.unwrap();
        assert_eq!("fghij", first);
        assert_eq!("fguij", second);
    }
}

use std::collections::HashMap;
use itertools::Itertools;

fn get_id_checksum(id: &String) -> (bool, bool) {
    let mut counts = HashMap::new();

    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let found_two = counts.iter().any(|(_, &v)| v == 2);
    let found_three = counts.iter().any(|(_, &v)| v == 3);
    return (found_two, found_three);
}

pub fn get_box_list_checksum(ids: &[String]) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let (two, three) = get_id_checksum(id);

        if two { twos += 1; }
        if three { threes += 1; }
    }

    return twos * threes;
}

fn find_matching_words(ids: &[String]) -> Option<(&String, &String)> {
    fn check_matching_words(one: &String, two: &String) -> bool {
        let matching = one.chars().zip(two.chars()).filter(|&(a, b)| a == b).count();
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
            ChecksumTest { id: "abcdef", result: (false, false) },
            ChecksumTest { id: "bababc", result: (true, true) },
            ChecksumTest { id: "abbcde", result: (true, false) },
            ChecksumTest { id: "abcccd", result: (false, true) },
            ChecksumTest { id: "aabcdd", result: (true, false) },
            ChecksumTest { id: "abcdee", result: (true, false) },
            ChecksumTest { id: "ababab", result: (false, true) },
        ];

        for test in &tests {
            assert_eq!(test.result, get_id_checksum(&test.id.to_owned()));
        }
    }

    #[test]
    fn test_get_box_list_checksum() {
        let boxes = vec![
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
        ];

        let owned_boxes = boxes.iter().map(|&x| x.to_owned()).collect::<Vec<String>>();

        assert_eq!(12, get_box_list_checksum(owned_boxes.as_slice()));
    }

    #[test]
    fn test_find_matching_words() {
        let boxes = vec![
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz"
        ];

        let owned_boxes = boxes.iter().map(|&x| x.to_owned()).collect::<Vec<String>>();

        let result = find_matching_words(owned_boxes.as_slice());

        assert!(result.is_some());

        let (first, second) = result.unwrap();
        assert_eq!("fghij", first);
        assert_eq!("fguij", second);
    }
}

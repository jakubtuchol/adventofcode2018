use std::collections::HashMap;

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
}

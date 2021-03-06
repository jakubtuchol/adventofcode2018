use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;

struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

// Given two claims, create a vector of overlapping spaces
fn get_claim_overlap<'a>(one: &'a Claim, two: &'a Claim) -> Vec<(u32, u32)> {
    let mut spaces = Vec::new();

    let left = max(one.left, two.left);
    let top = max(one.top, two.top);

    let right = min(one.left + one.width, two.left + two.width);
    let bottom = min(one.top + one.height, two.top + two.height);

    for x in left..right {
        for y in top..bottom {
            spaces.push((x, y));
        }
    }
    spaces
}

// Given a string in the format `#1 @ 1,3: 4x4`, return a structured
// claim struct
fn create_claim(code: &str) -> Option<Claim> {
    let split = code.split(' ').collect::<Vec<&str>>();
    if split.len() != 4 {
        return None;
    }

    let id_sector = split[0];
    let location_sector = split[2];
    let size_sector = split[3];

    let id: u32;
    match id_sector.replace("#", "").parse::<u32>() {
        Ok(n) => id = n,
        Err(_) => return None,
    }

    let location_cleaned = location_sector.replace(":", "");
    let location_split = location_cleaned.split(',').collect::<Vec<&str>>();

    if location_split.len() != 2 {
        return None;
    }

    let left: u32;
    let top: u32;
    match location_split[0].parse::<u32>() {
        Ok(n) => left = n,
        Err(_) => return None,
    }
    match location_split[1].parse::<u32>() {
        Ok(n) => top = n,
        Err(_) => return None,
    }

    let size_split = size_sector.split("x").collect::<Vec<&str>>();

    if size_split.len() != 2 {
        return None;
    }

    let width: u32;
    let height: u32;
    match size_split[0].parse::<u32>() {
        Ok(n) => width = n,
        Err(_) => return None,
    }
    match size_split[1].parse::<u32>() {
        Ok(n) => height = n,
        Err(_) => return None,
    }

    Some(Claim {
        id: id,
        left: left,
        top: top,
        width: width,
        height: height,
    })
}

// Find the of overlapping spaces given a vector of strings
// representing claims
pub fn find_num_overlapping_spaces(lines: Vec<String>) -> u32 {
    let mut claims: Vec<Claim> = Vec::new();
    for line in &lines {
        claims.push(create_claim(line).unwrap());
    }

    let mut overlaps = HashSet::new();

    for (one, two) in claims.iter().tuple_combinations::<(_, _)>() {
        let spaces = get_claim_overlap(&one, &two);
        for space in spaces {
            overlaps.insert(space);
        }
    }

    overlaps.len() as u32
}

// Find the id of a claim that does not overlap with any other claim
// given a vector of strings representing claims.
pub fn find_non_overlapping_claim(lines: Vec<String>) -> u32 {
    let mut claims = Vec::new();
    for line in &lines {
        claims.push(create_claim(line).unwrap());
    }

    let mut non_overlapping_candidates = HashSet::new();
    let mut overlaps = HashSet::new();

    for (one, two) in claims.iter().tuple_combinations::<(_, _)>() {
        let spaces = get_claim_overlap(&one, &two);
        if spaces.len() == 0 {
            if !overlaps.contains(&one.id) {
                non_overlapping_candidates.insert(one.id);
            }
            if !overlaps.contains(&two.id) {
                non_overlapping_candidates.insert(two.id);
            }
        } else {
            overlaps.insert(&one.id);
            overlaps.insert(&two.id);
        }
    }

    for id in &non_overlapping_candidates {
        if !overlaps.contains(id) {
            return *id;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CreateClaimTest<'a> {
        code: &'a str,
        result: Claim,
    }

    #[test]
    fn test_create_claim() {
        let tests = vec![
            CreateClaimTest {
                code: "#1 @ 1,3: 4x4",
                result: Claim {
                    id: 1,
                    left: 1,
                    top: 3,
                    width: 4,
                    height: 4,
                },
            },
            CreateClaimTest {
                code: "#2 @ 3,1: 4x4",
                result: Claim {
                    id: 2,
                    left: 3,
                    top: 1,
                    width: 4,
                    height: 4,
                },
            },
            CreateClaimTest {
                code: "#3 @ 5,5: 2x2",
                result: Claim {
                    id: 3,
                    left: 5,
                    top: 5,
                    width: 2,
                    height: 2,
                },
            },
        ];

        for test in &tests {
            let generated_claim = create_claim(test.code);
            assert!(generated_claim.is_some());
            let unwrapped_claim = generated_claim.unwrap();
            assert_eq!(test.result.id, unwrapped_claim.id);
            assert_eq!(test.result.left, unwrapped_claim.left);
            assert_eq!(test.result.top, unwrapped_claim.top);
            assert_eq!(test.result.width, unwrapped_claim.width);
            assert_eq!(test.result.height, unwrapped_claim.height);
        }
    }

    #[test]
    fn test_get_claim_overlap() {
        let spaces = get_claim_overlap(
            &Claim {
                id: 0,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            },
            &Claim {
                id: 1,
                left: 3,
                top: 1,
                width: 4,
                height: 4,
            },
        );

        assert_eq!(4, spaces.len());

        let spaces_non_overlapping = get_claim_overlap(
            &Claim {
                id: 0,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            },
            &Claim {
                id: 1,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            },
        );

        assert_eq!(0, spaces_non_overlapping.len());

        let single_overlap = get_claim_overlap(
            &Claim {
                id: 0,
                left: 0,
                top: 0,
                width: 4,
                height: 4,
            },
            &Claim {
                id: 1,
                left: 3,
                top: 3,
                width: 4,
                height: 4,
            },
        );

        assert_eq!(1, single_overlap.len());
    }

    #[test]
    fn test_find_num_overlapping_spaces() {
        let lines = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
        let owned_lines = lines.iter().map(|&x| x.to_owned()).collect::<Vec<String>>();
        assert_eq!(4, find_num_overlapping_spaces(owned_lines));
    }
}

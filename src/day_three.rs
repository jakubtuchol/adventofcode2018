struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

/*
fn get_claim_overlap<'a>(one: &'a Claim, two: &'a Claim) -> Vec<(u32, u32)> {

}
*/

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
}

use std::collections::HashSet;

pub fn calculate_final_frequency<'a>(frequencies: &'a [String]) -> i32 {
    let mut sum = 0;

    for frequency in frequencies {
        sum += frequency.parse::<i32>().unwrap();
    }

    return sum;
}

pub fn find_repeated_frequency<'a>(frequencies: &'a [String]) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();

    let mut sum = 0;
    set.insert(sum);

    let mut idx = 0;
    loop {
        let frequency = &frequencies[idx];
        sum += frequency.parse::<i32>().unwrap();

        if set.contains(&sum) {
            return sum;
        } else {
            set.insert(sum);
        }

        idx = (idx + 1) % frequencies.len();
    }
}



#[cfg(test)]
mod tests {
    use day_one::{calculate_final_frequency, find_repeated_frequency};

    struct FrequencyTest<'a> {
        input: &'a [&'a str],
        expected: i32,
    }

    #[test]
    fn test_calculate_final_frequency() {
        let tests = vec![
            FrequencyTest { input: &["+1", "+1", "+1"], expected: 3 },
            FrequencyTest { input: &["+1", "+1", "-2"], expected: 0 },
            FrequencyTest { input: &["-1", "-2", "-3"], expected: -6 },
        ];

        for test in &tests {
            let test_input = test.input
                .iter()
                .map(|&x| x.to_owned())
                .collect::<Vec<String>>();
            assert_eq!(test.expected, calculate_final_frequency(test_input.as_slice()));
        }
    }

    #[test]
    fn test_find_repeated_frequency() {
        let tests = vec![
            FrequencyTest { input: &["+1", "-1"], expected: 0 },
            FrequencyTest { input: &["+3", "+3", "+4", "-2", "-4"], expected: 10 },
            FrequencyTest { input: &["-6", "+3", "+8", "+5", "-6"], expected: 5 },
            FrequencyTest { input: &["+7", "+7", "-2", "-7", "-4"], expected: 14 },
        ];

        for test in &tests {
            let test_input = test.input
                .iter()
                .map(|&x| x.to_owned())
                .collect::<Vec<String>>();
            assert_eq!(test.expected, find_repeated_frequency(test_input.as_slice()));
        }
    }
}

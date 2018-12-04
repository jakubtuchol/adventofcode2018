pub fn calculate_final_frequency<'a>(frequencies: &'a [&'a str]) -> i32 {
    let mut sum = 0;

    for frequency in frequencies {
        sum += frequency.parse::<i32>().unwrap();
    }

    return sum;
}



#[cfg(test)]
mod tests {
    use day_one::calculate_final_frequency;

    struct FrequencyTest<'a> {
        input: &'a [&'a str],
        expected: i32,
    }

    #[test]
    fn test_provided_examples() {
        let tests = vec![
            FrequencyTest { input: &[ "+1", "+1", "+1"], expected: 3 },
            FrequencyTest { input: &[ "+1", "+1", "-2"], expected: 0 },
            FrequencyTest { input: &[ "-1", "-2", "-3"], expected: -6 },
        ];

        for test in &tests {
            assert_eq!(test.expected, calculate_final_frequency(test.input));
        }
    }
}

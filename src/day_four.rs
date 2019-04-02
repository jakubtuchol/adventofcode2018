extern crate regex;

use self::regex::Regex;

#[derive(Debug, PartialEq)]
enum EventType {
    Begin,
    Sleep,
    Wake,
}

struct Event {
    event: EventType,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

fn parse_event(line: &str) -> Event {
    let time_pattern = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\].+",
    ).unwrap();

    let time_caps = time_pattern.captures(line.clone()).unwrap();
    let year = &time_caps["year"].parse::<i32>().unwrap();
    let month = &time_caps["month"].parse::<i32>().unwrap();
    let day = &time_caps["day"].parse::<i32>().unwrap();
    let hour = &time_caps["hour"].parse::<i32>().unwrap();
    let minute = &time_caps["minute"].parse::<i32>().unwrap();

    let begin_regex = Regex::new(r".+Guard #\d+ begins shift").unwrap();
    let sleep_regex = Regex::new(r".+falls asleep").unwrap();
    let wake_regex = Regex::new(r".+wakes up").unwrap();

    let mut event: EventType;
    if begin_regex.is_match(line) {
        event = EventType::Begin;
    } else if sleep_regex.is_match(line) {
        event = EventType::Sleep;
    } else {
        event = EventType::Wake;
    }

    Event {
        event: event,
        year: *year,
        month: *month,
        day: *day,
        hour: *hour,
        minute: *minute,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ParseEventTest<'a> {
        line: &'a str,
        result: Event,
    }

    #[test]
    fn test_parse_event() {
        let tests = vec![
            ParseEventTest {
                line: "[1518-11-01 00:00] Guard #10 begins shift",
                result: Event {
                    event: EventType::Begin,
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 0,
                },
            },
            ParseEventTest {
                line: "[1518-11-01 00:05] falls asleep",
                result: Event {
                    event: EventType::Sleep,
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 5,
                },
            },
            ParseEventTest {
                line: "[1518-11-05 00:55] wakes up",
                result: Event {
                    event: EventType::Wake,
                    year: 1518,
                    month: 11,
                    day: 5,
                    hour: 0,
                    minute: 55,
                },
            },
        ];

        for test in &tests {
            let generated_result = parse_event(test.line);
            assert_eq!(test.result.event, generated_result.event);
            assert_eq!(test.result.year, generated_result.year);
            assert_eq!(test.result.month, generated_result.month);
            assert_eq!(test.result.day, generated_result.day);
            assert_eq!(test.result.hour, generated_result.hour);
            assert_eq!(test.result.minute, generated_result.minute);
        }
    }
}

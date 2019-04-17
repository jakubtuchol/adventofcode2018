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

impl Event {
    fn get_date(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

fn parse_event(line: &str) -> (Event, Option<i32>) {
    let time_pattern = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\].+",
    )
    .unwrap();

    let time_caps = time_pattern.captures(line.clone()).unwrap();
    let year = &time_caps["year"].parse::<i32>().unwrap();
    let month = &time_caps["month"].parse::<i32>().unwrap();
    let day = &time_caps["day"].parse::<i32>().unwrap();
    let hour = &time_caps["hour"].parse::<i32>().unwrap();
    let minute = &time_caps["minute"].parse::<i32>().unwrap();

    let begin_regex = Regex::new(r".+Guard #(?P<id>\d+) begins shift").unwrap();
    let sleep_regex = Regex::new(r".+falls asleep").unwrap();
    //let wake_regex = Regex::new(r".+wakes up").unwrap();

    let mut id = None;
    let event: EventType;
    if begin_regex.is_match(line) {
        event = EventType::Begin;
        let id_cap = begin_regex.captures(line.clone()).unwrap();
        let parsed_id = &id_cap["id"].parse::<i32>().unwrap();
        id = Some(*parsed_id);
    } else if sleep_regex.is_match(line) {
        event = EventType::Sleep;
    } else {
        event = EventType::Wake;
    }

    (
        Event {
            event: event,
            year: *year,
            month: *month,
            day: *day,
            hour: *hour,
            minute: *minute,
        },
        id,
    )
}

fn get_minute_difference(one: &Event, two: &Event) -> Vec<i32> {
    let mut minutes = vec![0; 60];

    let one_minute = one.minute as usize;
    let two_minute = two.minute as usize;

    for idx in one_minute..two_minute {
        minutes[idx] += 1;
    }

    minutes
}

fn get_vector_sum(one: Vec<i32>, two: Vec<i32>) -> Vec<i32> {
    one.iter()
        .zip(two.iter())
        .map(|(x, y)| x + y)
        .collect::<Vec<i32>>()
}

fn calculate_most_minutes_asleep(lines: Vec<String>) -> i32 {
    let mut last_event: Option<Event> = None;

    for line in lines {
        let new_event = parse_event(&line);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ParseEventTest<'a> {
        line: &'a str,
        event: Event,
        id: Option<i32>,
    }

    #[test]
    fn test_parse_event() {
        let tests = vec![
            ParseEventTest {
                line: "[1518-11-01 00:00] Guard #10 begins shift",
                event: Event {
                    event: EventType::Begin,
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 0,
                },
                id: Some(10),
            },
            ParseEventTest {
                line: "[1518-11-01 00:05] falls asleep",
                event: Event {
                    event: EventType::Sleep,
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 5,
                },
                id: None,
            },
            ParseEventTest {
                line: "[1518-11-05 00:55] wakes up",
                event: Event {
                    event: EventType::Wake,
                    year: 1518,
                    month: 11,
                    day: 5,
                    hour: 0,
                    minute: 55,
                },
                id: None,
            },
        ];

        for test in &tests {
            let (generated_event, generated_id) = parse_event(test.line);
            assert_eq!(test.event.event, generated_event.event);
            assert_eq!(test.event.year, generated_event.year);
            assert_eq!(test.event.month, generated_event.month);
            assert_eq!(test.event.day, generated_event.day);
            assert_eq!(test.event.hour, generated_event.hour);
            assert_eq!(test.event.minute, generated_event.minute);
            assert_eq!(test.id, generated_id);
        }
    }

    #[test]
    fn test_get_vector_sum() {
        let one = vec![1, 2, 3, 4];
        let two = vec![5, 6, 7, 8];

        assert_eq!(vec![6, 8, 10, 12], get_vector_sum(one, two));
    }
}

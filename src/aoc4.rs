use std::collections::HashMap;
use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::multi::many1;
use nom::*;
use std::cmp::Ordering;

use super::helper::u32_val;
use chrono::prelude::*;

fn date_time(i: &str) -> IResult<&str, DateTime<Local>> {
    let (i, _) = tag("[")(i)?;
    let (i, year) = u32_val(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, month) = u32_val(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, day) = u32_val(i)?;
    let (i, _) = space0(i)?;
    let (i, hour) = u32_val(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, minute) = u32_val(i)?;
    let (i, _) = tag("]")(i)?;
    Ok((
        i,
        Local
            .ymd(year as i32 + 500, month, day)
            .and_hms(hour, minute, 0),
    ))
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    WakesUp,
    FallsAsleep,
    BeginsShift(u32),
}

fn parse_wakes_up(i: &str) -> IResult<&str, Action> {
    let (i, _) = tag("wakes up")(i)?;
    Ok((i, Action::WakesUp))
}

fn parse_falls_asleep(i: &str) -> IResult<&str, Action> {
    let (i, _) = tag("falls asleep")(i)?;
    Ok((i, Action::FallsAsleep))
}

fn parse_begins_shift(i: &str) -> IResult<&str, Action> {
    let (i, _) = tag("Guard")(i)?;
    let (i, _) = space0(i)?;
    let (i, _) = tag("#")(i)?;
    let (i, id) = u32_val(i)?;
    let (i, _) = space0(i)?;
    let (i, _) = tag("begins shift")(i)?;
    Ok((i, Action::BeginsShift(id)))
}

fn action(i: &str) -> IResult<&str, Action> {
    alt((parse_wakes_up, parse_falls_asleep, parse_begins_shift))(i)
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    dt: DateTime<Local>,
    action: Action,
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Line) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Line) -> Ordering {
        self.dt.cmp(&other.dt)
    }
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    let (i, dt) = date_time(i)?;
    let (i, _) = space0(i)?;
    let (i, action) = action(i)?;
    let (i, _) = opt(tag("\n"))(i)?;
    Ok((i, Line { dt, action }))
}

fn parse(i: &str) -> IResult<&str, Vec<Line>> {
    many1(parse_line)(i)
}

/*
[1518-09-14 00:54] wakes up
[1518-04-15 23:58] Guard #373 begins shift
[1518-06-21 00:43] falls asleep
*/

pub fn run() {
    let input = fs::read_to_string("day4.txt").unwrap();
    let (_, mut lines) = parse(&input).unwrap();
    lines.sort();

    println!("4:1 {}", run_1(&lines));
    println!("4:2 {}", run_2(&lines));
}

fn run_1(lines: &[Line]) -> u32 {
    let mut current_guard = 0;
    let mut sleep_start = 0;
    let mut acc_sleep_time = HashMap::new();
    let mut sleep_minute_count: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let zero = 0;
    for line in lines {
        match line.action {
            Action::BeginsShift(id) => current_guard = id,
            Action::FallsAsleep => sleep_start = line.dt.minute(),
            Action::WakesUp => {
                let wakeup_time = line.dt.minute();
                let old = acc_sleep_time.get(&current_guard).unwrap_or(&zero);
                acc_sleep_time.insert(current_guard, old + wakeup_time - sleep_start);

                if !sleep_minute_count.contains_key(&current_guard) {
                    sleep_minute_count.insert(current_guard, HashMap::new());
                }
                let min_lu = sleep_minute_count.get_mut(&current_guard).unwrap();
                for minute in sleep_start..wakeup_time {
                    let min_cnt = min_lu.get(&minute).unwrap_or(&zero);
                    min_lu.insert(minute, min_cnt + 1);
                }
            }
        }
    }

    let (sleepiest_guard, _) = acc_sleep_time
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    let (sleepiest_minute, _) = sleep_minute_count
        .get(&sleepiest_guard)
        .unwrap()
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    // println!("Sleepiest: {:?}", sleepiest_guard);
    // println!("Sleepiest minute: {:?}", sleepiest_minute);

    sleepiest_guard * sleepiest_minute
}

fn run_2(lines: &[Line]) -> u32 {
    let mut current_guard = 0;
    let mut sleep_start = 0;
    let mut sleep_minute_count: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let zero = 0;
    for line in lines {
        match line.action {
            Action::BeginsShift(id) => current_guard = id,
            Action::FallsAsleep => sleep_start = line.dt.minute(),
            Action::WakesUp => {
                let wakeup_time = line.dt.minute();

                if !sleep_minute_count.contains_key(&current_guard) {
                    sleep_minute_count.insert(current_guard, HashMap::new());
                }
                let min_lu = sleep_minute_count.get_mut(&current_guard).unwrap();
                for minute in sleep_start..wakeup_time {
                    let min_cnt = min_lu.get(&minute).unwrap_or(&zero);
                    min_lu.insert(minute, min_cnt + 1);
                }
            }
        }
    }

    let mut sleepiest_guard = 0;
    let mut sleepiest_minute = 0;
    let mut minute_sleep = 0;

    for (guard, lu) in sleep_minute_count.iter() {
        for (minute, acc) in lu.iter() {
            if *acc > minute_sleep {
                sleepiest_minute = *minute;
                sleepiest_guard = *guard;
                minute_sleep = *acc;
            }
        }
    }

    // println!("Sleepiest: {:?}", sleepiest_guard);
    // println!("Sleepiest minute: {:?}", sleepiest_minute);

    sleepiest_guard * sleepiest_minute
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc4_parse_dt() {
        assert_eq!(u32_val("123"), Ok(("", 123)));
        assert_eq!(
            date_time("[1518-09-14 00:54]"),
            Ok(("", Local.ymd(1518 + 500, 9, 14).and_hms(00, 54, 0),))
        );
    }

    #[test]
    fn aoc4_parse_action() {
        assert_eq!(parse_wakes_up("wakes up"), Ok(("", Action::WakesUp)));
        assert_eq!(action("wakes up"), Ok(("", Action::WakesUp)));
        assert_eq!(
            parse_falls_asleep("falls asleep"),
            Ok(("", Action::FallsAsleep))
        );
        assert_eq!(action("falls asleep"), Ok(("", Action::FallsAsleep)));
        assert_eq!(
            parse_begins_shift("Guard #373 begins shift"),
            Ok(("", Action::BeginsShift(373)))
        );
        assert_eq!(
            action("Guard #373 begins shift"),
            Ok(("", Action::BeginsShift(373)))
        );
    }

    #[test]
    fn aoc4_parse_line() {
        assert_eq!(
            parse_line("[1518-04-15 23:58] Guard #373 begins shift"),
            Ok((
                "",
                Line {
                    dt: Local.ymd(1518 + 500, 4, 15).and_hms(23, 58, 00),
                    action: Action::BeginsShift(373),
                }
            ))
        );
    }

    #[test]
    fn aoc4_parse_lines_and_sort() {
        let input = r#"[1518-09-14 00:54] wakes up
[1518-04-15 23:58] Guard #373 begins shift
[1518-07-25 00:53] wakes up"#;
        let (_, mut lines) = parse(input).unwrap();

        lines.sort();

        assert_eq!(
            lines[0],
            Line {
                dt: Local.ymd(1518 + 500, 4, 15).and_hms(23, 58, 00),
                action: Action::BeginsShift(373)
            }
        );
        assert_eq!(
            lines[2],
            Line {
                dt: Local.ymd(1518 + 500, 9, 14).and_hms(0, 54, 0),
                action: Action::WakesUp
            }
        );
    }

    #[test]
    fn aoc4_run() {
        let input = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;

        let (_, mut lines) = parse(input).unwrap();

        lines.sort();

        assert_eq!(run_1(&lines), 240);
        assert_eq!(run_2(&lines), 4455);
    }
}

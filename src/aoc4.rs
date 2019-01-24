// use regex::Regex;
// use std::collections::HashMap;
// use std::fs;

use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, PartialEq)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

fn from_dec(input: CompleteStr) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

named!(u32_val<CompleteStr, u32>,
       map_res!(digit, |s: CompleteStr| { s.parse::<u32>()} )
       );

named!(date_time<CompleteStr, DateTime>,
       do_parse!(
           tag!("[") >>
           year: u32_val >>
           tag!("-") >>
           month: u32_val >>
           tag!("-") >>
           day: u32_val >>
           space >>
           hour: u32_val >>
           tag!(":") >>
           minute: u32_val >>
           tag!("]") >>
           (DateTime { year, month, day, hour, minute })
           )
       );

#[derive(Debug, PartialEq)]
enum Action {
    WakesUp,
    FallsAsleep,
    BeginsShift(u32),
}

named!(parse_wakes_up<CompleteStr, Action>, do_parse!(
       tag!("wakes up") >>
       (Action::WakesUp)
       ));

named!(parse_falls_asleep<CompleteStr, Action>, do_parse!(
        tag!("falls asleep") >>
        (Action::FallsAsleep)
        ));

named!(parse_begins_shift<CompleteStr, Action>, do_parse!(
        tag!("Guard") >>
        space >>
        tag!("#") >>
        id: u32_val >>
        space >>
        tag!("begins shift") >>
        (Action::BeginsShift(id))
        ));

named!(
    action<CompleteStr, Action>,
    alt!(parse_wakes_up | parse_falls_asleep | parse_begins_shift)
);

#[derive(Debug, PartialEq)]
struct Line {
    dt: DateTime,
    action: Action,
}

named!(parse_line<CompleteStr, Line>, do_parse!(
        dt: date_time >>
        space >>
        action: action >>
        (Line { dt, action })
        ));

/*
[1518-09-14 00:54] wakes up
[1518-04-15 23:58] Guard #373 begins shift
[1518-06-21 00:43] falls asleep
*/

pub fn run() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc4_parse_dt() {
        assert_eq!(from_dec(CompleteStr("123")), Ok(123));
        assert_eq!(u32_val(CompleteStr("123")), Ok((CompleteStr(""), 123)));
        assert_eq!(
            date_time(CompleteStr("[1518-09-14 00:54]")),
            Ok((
                CompleteStr(""),
                DateTime {
                    year: 1518,
                    month: 09,
                    day: 14,
                    hour: 00,
                    minute: 54,
                }
            ))
        );
    }

    #[test]
    fn aoc4_parse_action() {
        assert_eq!(
            parse_wakes_up(CompleteStr("wakes up")),
            Ok((CompleteStr(""), Action::WakesUp))
        );
        assert_eq!(
            action(CompleteStr("wakes up")),
            Ok((CompleteStr(""), Action::WakesUp))
        );
        assert_eq!(
            parse_falls_asleep(CompleteStr("falls asleep")),
            Ok((CompleteStr(""), Action::FallsAsleep))
        );
        assert_eq!(
            action(CompleteStr("falls asleep")),
            Ok((CompleteStr(""), Action::FallsAsleep))
        );
        assert_eq!(
            parse_begins_shift(CompleteStr("Guard #373 begins shift")),
            Ok((CompleteStr(""), Action::BeginsShift(373)))
        );
        assert_eq!(
            action(CompleteStr("Guard #373 begins shift")),
            Ok((CompleteStr(""), Action::BeginsShift(373)))
        );
    }

    #[test]
    fn aoc4_parse_line() {
        assert_eq!(
            parse_line(CompleteStr("[1518-04-15 23:58] Guard #373 begins shift")),
            Ok((
                CompleteStr(""),
                Line {
                    dt: DateTime {
                        year: 1518,
                        month: 04,
                        day: 15,
                        hour: 23,
                        minute: 58
                    },
                    action: Action::BeginsShift(373),
                }
            ))
        );
    }
}
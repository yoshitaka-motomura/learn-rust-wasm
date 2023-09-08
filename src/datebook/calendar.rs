//! # Calendar
//! This module provides a function to get a list of japanese holidays in a year.
//! 
use chrono::{Datelike, Duration, Weekday, NaiveDate, Local, DateTime};
use chrono::TimeZone;
use anyhow::{Result, Error, Ok};
use serde::Serialize;
use super::timebase::{get_schedule, get_equinox_dates, Condition};

/// Holiday
#[derive(Serialize)]
pub struct Holiday {
    pub name: String, // name of holiday
    pub date: NaiveDate, // date of holiday
    pub substitute: bool, // if it is a substitute holiday
}

/// Get a list of japanese holidays in a year.
pub fn holiday(year: u32)-> Result<Vec<Holiday>, Error> {
    //List of holidays stipulated in the Holidays Act
    let mut m = prepara(year);
    let e= pick_exuinox_from_year(year);
    m.extend(e);
    substitute_adjustment(&mut m);

    //sort
    m.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(m)
}

// private functions

fn substitute_adjustment(data: &mut Vec<Holiday>) {
   let mut i:usize = 0;
   while i < data.len() {
        // if it a Sunday
        if data[i].date.weekday() == Weekday::Sun {
            let mut last_holiday_date = data[i].date;
            while let Some(next_holiday) = data.get(i+1) {
                if next_holiday.date == last_holiday_date + Duration::days(1) {
                    i += 1;
                    last_holiday_date = next_holiday.date;
                } else {
                    break;
                }
            }
            let mut sub_date = last_holiday_date + Duration::days(1);
            while data.iter().any(|h:&Holiday| h.date == sub_date) {
                sub_date = sub_date + Duration::days(1);
            }

            data.push(Holiday {
                name: format!("振替休日({})", data[i].name),
                date: sub_date,
                substitute: true,
            });
        }
        i += 1;
   }
}


fn pick_exuinox_from_year(year:u32) -> Vec<Holiday> {
    if year < 2020 || year > 2050 {
        return Vec::new();
    }
    let equinoxes = get_equinox_dates().unwrap();
    let target = equinoxes.into_iter().find(|x| x.year == year);
    let mut return_value: Vec<Holiday> = Vec::new();
    match target {
        Some(v) => {
            v.equinox.into_iter().for_each(|x| {
                return_value.push(Holiday {
                    name: x.name,
                    date: NaiveDate::parse_from_str(&format!("{}/{}", year, x.date).to_string(), "%Y/%m/%d").unwrap(),
                    substitute: false,
                });
            })
        },
        None => {},
    }

    return_value


}

// for base dates
fn prepara(year: u32)->Vec<Holiday> {
    let dataset = get_schedule().unwrap();
    let mut days: Vec<Holiday> = Vec::new();
    for d in dataset {
        if d.relative {
            let relative_date = get_relative_date(year, d.condition.unwrap()).unwrap();
            days.push(Holiday {
                name: d.name,
                date: relative_date.format("%Y-%m-%d").to_string().parse::<NaiveDate>().unwrap(),
                substitute: false,
            })
        } else {
            days.push(Holiday {
                name: d.name,
                date: NaiveDate::parse_from_str(&format!("{}/{}", year, d.date.unwrap()).to_string()
                , "%Y/%m/%d").unwrap(),
                substitute: false,
            })
        }
    }
    days
}

// for relative date comvart Datetime
fn get_relative_date(year: u32, condition: Condition)-> Option<DateTime<Local>> {
    let month = get_month_num_from_string(&condition.month).unwrap();
    let weekday = get_weekday_from_string(&condition.weekday).unwrap();
    let n = condition.n;
    let mut dates: Vec<DateTime<Local>> = Vec::new();
    let mut day:DateTime<Local> = Local.with_ymd_and_hms(year as i32, month, 1, 0, 0, 0).unwrap();

    while day.month() == month {
        if day.weekday() == weekday {
            dates.push(day);
        }
        day = day + Duration::days(1);
    }

    Some(dates[n as usize -1])
}

fn get_weekday_from_string(char: &str)-> Option<Weekday> {
    match char.trim().to_lowercase().as_str() {
        "monday" | "mon" => Some(Weekday::Mon),
        "tuesday" | "tue" => Some(Weekday::Tue),
        "wednesday" | "wed" => Some(Weekday::Wed),
        "thursday" | "thu" => Some(Weekday::Thu),
        "friday" | "fri" => Some(Weekday::Fri),
        "saturday" | "sat" => Some(Weekday::Sat),
        "sunday" | "sun" => Some(Weekday::Sun),
        _ => None,
    }
}
fn get_month_num_from_string(char: &str) -> Option<u32> {
    match char.trim().to_lowercase().as_str() {
        "january" | "jan" => Some(1),
        "february" | "feb" => Some(2),
        "march" | "mar" => Some(3),
        "april" | "apr" => Some(4),
        "may" => Some(5),
        "june" | "jun" => Some(6),
        "july" | "jul" => Some(7),
        "august" | "aug" => Some(8),
        "september" | "sep" => Some(9),
        "october" | "oct" => Some(10),
        "november" | "nov" => Some(11),
        "december" | "dec" => Some(12),
        _ => None,
    }
}


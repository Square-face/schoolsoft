use chrono::{Datelike, Local, Months, NaiveDate, NaiveTime, NaiveWeek, Weekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{deserializers::Deserializer, types::error::ScheduleParseError, utils::WeekRange};

/// Holds information for the entire schedule as it is currently known
#[derive(Debug)]
pub struct Schedule {
    pub weeks: [ScheduleWeek; 53],
}

/// Contains information for a single week in the schedule
#[derive(Debug)]
pub struct ScheduleWeek {
    pub week: NaiveWeek,
    pub monday: ScheduleDay,
    pub tuesday: ScheduleDay,
    pub wednesday: ScheduleDay,
    pub thursday: ScheduleDay,
    pub friday: ScheduleDay,
    pub saturday: ScheduleDay,
    pub sunday: ScheduleDay,
}

/// Contains information for a single day in the schedule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleDay {
    pub date: NaiveDate,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lesson {
    pub start: chrono::NaiveTime,
    pub end: chrono::NaiveTime,
    pub name: String,
    pub room: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawOccasion {
    weeks: u64,
    excluding_weeks: u64,
    cre_by_id: u64,
    source: serde_json::Value,
    external_ref: String,
    subject_id: u64,
    org_id: u64,
    upd_date: String,
    upd_by_type: i32,
    exclude_class: u64,
    start_time: String,
    id: u64,
    including_weeks: u64,
    subject_name: String,
    upd_by_id: u64,
    cre_by_type: i32,
    cre_date: String,
    length: u64,
    external_id: String,
    room_name: String,
    period_weeks: u64,
    including_weeks_string: String,
    day_id: u8,
    name: String,
    absence_type: u64,
    guid: String,
    excluding_weeks_string: String,
    end_time: String,
    weeks_string: String,
    tmp_lesson: u64,
}

#[derive(Debug)]
pub struct Occasion {
    pub id: u64,
    pub uuid: Uuid,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub subject_name: String,
    pub room_name: String,
    pub week_day: Weekday,
    pub weeks: Vec<u8>,
}

impl Schedule {
    /// Create a new Schedule
    ///
    /// Note that weeks are shifted down by one, i.e the first week of the year (week 1) will have
    /// the index 0
    ///
    /// If the start date is at the first half of the year, it means we are in the end of the school year.
    /// And the later half of the weeks will be from last year.
    ///
    /// If the start date is at the second half of the year, it means we are in the beginning of the school year.
    /// And the first half of the weeks will be from next year.
    ///
    /// # Arguments
    ///
    /// * `start` - The date to use as origin when creating the schedule
    ///
    /// # Returns
    ///
    /// A new Schedule
    ///
    /// # Examples
    ///
    /// Example with the start date in the first half of the year
    ///
    /// ```
    /// # use chrono::{NaiveDate, Local, Datelike};
    /// # use schoolsoft::schedule::Schedule;
    /// let april = NaiveDate::from_ymd(2024, 4, 1);
    /// let schedule = Schedule::from(april);
    ///
    /// assert_eq!(schedule.weeks.len(), 53);
    /// assert_eq!(schedule.weeks[10].monday.date.year_ce(), (true, 2024));
    /// assert_eq!(schedule.weeks[40].monday.date.year_ce(), (true, 2023));
    /// ```
    ///
    /// Example with the start date in the second half of the year
    ///
    /// ```
    /// # use chrono::{NaiveDate, Local, Datelike};
    /// # use schoolsoft::schedule::Schedule;
    /// let september = NaiveDate::from_ymd(2024, 9, 1);
    /// let schedule = Schedule::from(september);
    ///
    /// assert_eq!(schedule.weeks.len(), 53);
    /// assert_eq!(schedule.weeks[10].monday.date.year_ce(), (true, 2025));
    /// assert_eq!(schedule.weeks[40].monday.date.year_ce(), (true, 2024));
    /// ```
    pub fn from(start: NaiveDate) -> Self {
        let start_of_year = start.with_ordinal0(0).unwrap();

        // Check if we are in the first or second half of the year.
        let mut weeks = match start < start.with_ordinal(365 / 2).unwrap() {
            true => {
                // First half of the year
                let this_year_weeks = start_of_year.iter_weeks().take(26);

                // Second half of last year
                let prev_year_weeks = start_of_year
                    .checked_sub_months(Months::new(12))
                    .unwrap()
                    .iter_weeks()
                    .skip(26)
                    .take(27);

                // Combine the two
                this_year_weeks.chain(prev_year_weeks)
            }

            false => {
                // First half of next year
                let next_year_weeks = start_of_year
                    .checked_add_months(Months::new(12))
                    .unwrap()
                    .iter_weeks()
                    .take(26);

                // Second half of this year
                let this_year_weeks = start_of_year.iter_weeks().skip(26).take(27);

                // Combine the two
                next_year_weeks.chain(this_year_weeks)
            }
        };

        Schedule {
            weeks: [0; 53].map(|_| {
                ScheduleWeek::new_empty(weeks.next().unwrap().week(Weekday::Mon)).unwrap()
            }),
        }
    }
}

impl ScheduleDay {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            lessons: Vec::new(),
        }
    }
}

impl ScheduleWeek {
    /// Create a new empty ScheduleWeek
    ///
    /// # Arguments
    /// * `start` - The first day of the week
    ///
    /// # Examples
    /// ```
    /// # use chrono::{NaiveDate, Weekday, NaiveWeek, Datelike};
    /// # use schoolsoft::schedule::ScheduleWeek;
    /// let start = NaiveDate::from_ymd(2024, 2, 5);
    /// let week = ScheduleWeek::new_empty(start.week(Weekday::Mon));
    /// ```
    ///
    /// # Returns
    /// A new ScheduleWeek with no lessons
    pub fn new_empty(start: NaiveWeek) -> Option<Self> {
        let mut days = start.first_day().iter_days();

        Some(ScheduleWeek {
            week: start,
            monday: ScheduleDay::new(days.next()?),
            tuesday: ScheduleDay::new(days.next()?),
            wednesday: ScheduleDay::new(days.next()?),
            thursday: ScheduleDay::new(days.next()?),
            friday: ScheduleDay::new(days.next()?),
            saturday: ScheduleDay::new(days.next()?),
            sunday: ScheduleDay::new(days.next()?),
        })
    }
}

impl TryFrom<RawOccasion> for Occasion {
    type Error = ScheduleParseError;

    fn try_from(value: RawOccasion) -> Result<Self, Self::Error> {
        let mut weeks_mask = [false; 54];
        let mut weeks = Vec::new();

        // Base weeks
        for week in WeekRange::from(value.weeks_string.as_str()) {
            weeks_mask[week as usize] = true;
        }

        // Excluded weeks
        for week in WeekRange::from(value.excluding_weeks_string.as_str()) {
            weeks_mask[week as usize] = false;
        }

        // Included weeks
        for week in WeekRange::from(value.including_weeks_string.as_str()) {
            weeks_mask[week as usize] = true;
        }

        // Convert mask to vector of weeks
        for (i, week) in weeks_mask.iter().enumerate() {
            if *week {
                weeks.push(i as u8);
            }
        }

        let uuid = Uuid::parse_str(&value.guid).map_err(ScheduleParseError::UuidParseError)?;
        let week_day =
            Weekday::try_from(value.day_id).map_err(ScheduleParseError::DayOfWeekError)?;

        let start_time = NaiveTime::parse_from_str(&value.start_time, "%Y-%m-%d %H:%M:%S%.f")
            .map_err(ScheduleParseError::TimeParseError)?;
        let end_time = NaiveTime::parse_from_str(&value.end_time, "%Y-%m-%d %H:%M:%S%.f")
            .map_err(ScheduleParseError::TimeParseError)?;

        Ok(Occasion {
            id: value.id,
            uuid,
            start_time,
            end_time,
            subject_name: value.subject_name,
            room_name: value.room_name,
            week_day,
            weeks,
        })
    }
}

impl Deserializer for Schedule {
    type Error = ScheduleParseError;

    fn deserialize(data: &str) -> Result<Self, Self::Error> {
        let raw: Vec<RawOccasion> =
            serde_json::from_str(data).map_err(ScheduleParseError::SerdeError)?;

        let mut schedule = Schedule::from(Local::now().naive_local().date());

        for raw_occasion in raw {
            let occasion = Occasion::try_from(raw_occasion)?;
            let lesson = Lesson {
                start: occasion.start_time,
                end: occasion.end_time,
                name: occasion.subject_name,
                room: occasion.room_name,
            };

            for week in occasion.weeks {
                let x = &mut schedule.weeks[(week - 1) as usize];
                match occasion.week_day {
                    Weekday::Mon => &mut x.monday,
                    Weekday::Tue => &mut x.tuesday,
                    Weekday::Wed => &mut x.wednesday,
                    Weekday::Thu => &mut x.thursday,
                    Weekday::Fri => &mut x.friday,
                    Weekday::Sat => &mut x.saturday,
                    Weekday::Sun => &mut x.sunday,
                }
                .lessons
                .push(lesson.clone());
            }
        }

        Ok(schedule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_schedule_week() {
        let week =
            ScheduleWeek::new_empty(NaiveDate::from_ymd(2024, 3, 25).week(Weekday::Mon)).unwrap();

        assert_eq!(week.monday.date, NaiveDate::from_ymd(2024, 3, 25));
        assert_eq!(week.tuesday.date, NaiveDate::from_ymd(2024, 3, 26));
        assert_eq!(week.wednesday.date, NaiveDate::from_ymd(2024, 3, 27));
        assert_eq!(week.thursday.date, NaiveDate::from_ymd(2024, 3, 28));
        assert_eq!(week.friday.date, NaiveDate::from_ymd(2024, 3, 29));
    }
}

#[cfg(test)]
mod occasion {
    use pretty_assertions::assert_eq;

    use super::{Occasion, RawOccasion};

    #[test]
    fn deserialize() {
        let data = r#"{
            "weeks": 2242995147496956,
            "excludingWeeks": 0,
            "creById": 0,
            "source": {},
            "externalRef": "",
            "subjectId": 236,
            "orgId": 1,
            "updDate": "2023-08-16 12:42:41.0",
            "updByType": -1,
            "excludeClass": 0,
            "startTime": "1970-01-01 08:20:00.0",
            "id": 36505,
            "includingWeeks": 0,
            "subjectName": "IDRIDO02 - Idrott och hälsa 2 - specialisering",
            "updById": 0,
            "creByType": -1,
            "creDate": "2023-08-16 12:42:41.0",
            "length": 70,
            "externalId": "34b0d97c-35ea-415f-9c7b-7f9492ef9fb4",
            "roomName": "IKSU",
            "periodWeeks": 2242995147496956,
            "includingWeeksString": "",
            "dayId": 0,
            "name": "",
            "absenceType": 1,
            "guid": "afbae58f-c35e-4480-bfd1-574fc8de5572",
            "excludingWeeksString": "",
            "endTime": "1970-01-01 09:30:00.0",
            "weeksString": "34-43, 45-51, 3-9, 11-13, 15-24",
            "tmpLesson": 0
        }"#;

        let raw: RawOccasion = serde_json::from_str(data).expect("Deserializing should work");


        assert_eq!(raw.upd_date, "2023-08-16 12:42:41.0");
        assert_eq!(raw.cre_date, "2023-08-16 12:42:41.0");
        assert_eq!(raw.upd_by_id, 0);
        assert_eq!(raw.upd_by_type, -1);
        assert_eq!(raw.cre_by_id, 0);
        assert_eq!(raw.cre_by_type, -1);

        assert_eq!(raw.org_id, 1);

        assert_eq!(raw.external_ref, "");
        assert_eq!(raw.external_id, "34b0d97c-35ea-415f-9c7b-7f9492ef9fb4");
        assert_eq!(raw.subject_id, 236);
        assert_eq!(raw.guid, "afbae58f-c35e-4480-bfd1-574fc8de5572");
        assert_eq!(raw.id, 36505);

        assert_eq!(raw.subject_name, "IDRIDO02 - Idrott och hälsa 2 - specialisering");
        assert_eq!(raw.room_name, "IKSU");

        assert_eq!(raw.day_id, 0);
        assert_eq!(raw.start_time, "1970-01-01 08:20:00.0");
        assert_eq!(raw.end_time, "1970-01-01 09:30:00.0");
        assert_eq!(raw.length, 70);

        assert_eq!(raw.including_weeks, 0);

        assert_eq!(raw.including_weeks_string, "");
        assert_eq!(raw.excluding_weeks_string, "");
        assert_eq!(raw.weeks_string, "34-43, 45-51, 3-9, 11-13, 15-24");

        let occasion = Occasion::try_from(raw).expect("Converting should work");

        assert_eq!(occasion.id, 36505);
        assert_eq!(occasion.uuid.to_string(), "afbae58f-c35e-4480-bfd1-574fc8de5572");
        assert_eq!(occasion.start_time.to_string(), "08:20:00");
        assert_eq!(occasion.end_time.to_string(), "09:30:00");
        assert_eq!(occasion.subject_name, "IDRIDO02 - Idrott och hälsa 2 - specialisering");
        assert_eq!(occasion.room_name, "IKSU");
        assert_eq!(occasion.week_day, chrono::Weekday::Mon);
        assert_eq!(occasion.weeks, vec![3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 45, 46, 47, 48, 49, 50, 51]);
    }
}

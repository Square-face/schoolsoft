use chrono::{Datelike, Months, NaiveDate, NaiveWeek, Weekday};

/// Holds information for the entire schedule as it is currently known
#[derive(Debug)]
pub struct Schedule {
    pub weeks: [ScheduleWeek; 52],
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

impl Schedule {
    /// Create a new Schedule
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
    /// assert_eq!(schedule.weeks.len(), 52);
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
    /// assert_eq!(schedule.weeks.len(), 52);
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
                    .take(26);

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
                let this_year_weeks = start_of_year.iter_weeks().skip(26).take(26);

                // Combine the two
                next_year_weeks.chain(this_year_weeks)
            }
        };

        Schedule {
            weeks: [0; 52].map(|_| {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_schedule_week() {
        let week = ScheduleWeek::new_empty(NaiveDate::from_ymd(2024, 3, 25).week(Weekday::Mon)).unwrap();

        assert_eq!(week.monday.date, NaiveDate::from_ymd(2024, 3, 25));
        assert_eq!(week.tuesday.date, NaiveDate::from_ymd(2024, 3, 26));
        assert_eq!(week.wednesday.date, NaiveDate::from_ymd(2024, 3, 27));
        assert_eq!(week.thursday.date, NaiveDate::from_ymd(2024, 3, 28));
        assert_eq!(week.friday.date, NaiveDate::from_ymd(2024, 3, 29));
    }
}

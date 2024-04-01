use chrono::{Datelike, Utc};
use schoolsoft::ClientBuilder;
use std::io;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = ClientBuilder::new().build();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("Username: ");
    let username = lines.next().unwrap()?;

    println!("Password: ");
    let password = lines.next().unwrap()?;

    println!("School: ");
    let school = lines.next().unwrap()?;

    // Login
    client.login(&username, &password, &school).await.unwrap();
    let mut user = client.user.unwrap();
    println!("Logged in as {}", user.name);

    // Get current time
    let now = Utc::now().naive_local();
    let today = now.date();

    // Get schedule
    let mut weeks = user.get_schedule().await.unwrap().weeks;

    // Find the next lesson
    for day in today.iter_days() {
        let week = weeks.get_mut(day.iso_week().week0() as usize).unwrap();
        let schedule = week.get_day(day.weekday());
        let lessons = &schedule.lessons;

        if lessons.is_empty() {
            continue;
        }

        // Find the first lesson that hasn't started yet, if any
        for lesson in lessons.iter() {

            // if lesson is in the past, skip it
            if schedule.date.and_time(lesson.start) < now {
                continue;
            }

            dbg!(lesson);
            return Ok(());
        }
    }

    Ok(())
}

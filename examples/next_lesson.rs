/// This example logs in to Schoolsoft and prints the next lesson.
///
/// The user is prompted for their username, password and school. Which is then used to authenticate
/// with Schoolsoft. The user's schedule is then fetched and searched for the next lesson.
use chrono::{Datelike, Utc};
use schoolsoft::ClientBuilder;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = ClientBuilder::new().build();

    // Login
    let username = prompt("Username: ")?;
    let password = prompt("Password: ")?;
    let school = prompt("School: ")?;

    match client.login(&username, &password, &school).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to login: {}", e);
            return Ok(());
        }
    };

    // Get user
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

fn prompt(prompt: &str) -> io::Result<String> {
    let mut buf = String::new();

    println!("{}", prompt);
    io::stdin().read_line(&mut buf)?;

    Ok(buf.trim_end().to_string())
}

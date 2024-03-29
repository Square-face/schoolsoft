use schoolsoft::{deserializers::Deserializer, schedule::{self, RawOccasion, Schedule}};

#[test]
fn full() {
    let data = include_str!("../hurl/output/schedule.json");

    let schedule = Schedule::deserialize(data).unwrap();
}

#[test]
fn single_occasion() {
    let data = r#"
    {
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
    }
        "#;

    let raw: RawOccasion = serde_json::from_str(data).unwrap();

    let occasion = schedule::Occasion::try_from(raw).unwrap();
}



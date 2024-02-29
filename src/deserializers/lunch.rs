use crate::{
    types::{error::LunchMenuParseError, Lunch, LunchMenu},
    utils,
};
use serde::Deserialize;

use super::Deserializer;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct RawLunchMenu {
    saturday: String,
    sunday: String,
    week: u32,
    upd_by_id: u32,
    cre_by_type: i32,
    cre_date: String,
    dish_category_name: String,
    cre_by_id: u32,
    thursday: String,
    dates: [String; 7],
    org_id: u32,
    upd_date: String,
    empty: bool,
    upd_by_type: i32,
    tuesday: String,
    dish: u32,
    wednesday: String,
    friday: String,
    id: i32,
    monday: String,
}

impl Deserializer for LunchMenu {
    type Error = LunchMenuParseError;

    fn deserialize(data: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let raw: Vec<RawLunchMenu> =
            serde_json::from_str(data).map_err(LunchMenuParseError::SerdeError)?;
        let raw = raw
            .first()
            .ok_or_else(|| LunchMenuParseError::NoLunchMenu)?;

        let dates = raw
            .dates
            .iter()
            .map(|date| {
                utils::parse_date(date)
                    .map_err(|err| LunchMenuParseError::DateParseError(date.to_string(), err))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(LunchMenu {
            week: raw.week,
            monday: Lunch {
                date: dates[0],
                food: raw.monday.clone(),
            },
            tuesday: Lunch {
                date: dates[1],
                food: raw.tuesday.clone(),
            },
            wednesday: Lunch {
                date: dates[2],
                food: raw.wednesday.clone(),
            },
            thursday: Lunch {
                date: dates[3],
                food: raw.thursday.clone(),
            },
            friday: Lunch {
                date: dates[4],
                food: raw.friday.clone(),
            },
            created_at: utils::parse_datetime(&raw.cre_date)
                .map_err(|err| LunchMenuParseError::DateParseError(raw.cre_date.clone(), err))?,
            category: raw.dish_category_name.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let data = r#"[
            {
                "saturday": "",
                "week": 8,
                "updById": 112,
                "creByType": -1,
                "creDate": "2024-02-16 15:03:15.0",
                "dishCategoryName": "Lunch",
                "creById": 112,
                "thursday": "Pestobakad fisk med vitvinsås och pasta penne.\r\n\r\nVeg:\r\nGrönsaksbiffar med vitvinsås och pasta penne.",
                "dates": [
                    "2024-02-19",
                    "2024-02-20",
                    "2024-02-21",
                    "2024-02-22",
                    "2024-02-23",
                    "2024-02-24",
                    "2024-02-25"
                ],
                "orgId": 1,
                "updDate": "2024-02-16 15:03:15.0",
                "empty": false,
                "updByType": -1,
                "sunday": "",
                "tuesday": "Het köttfärssoppa med kökets bröd.\r\n\r\nVeg:\r\nHet bön och rotfruktssoppa med kökets bröd.",
                "dish": 1,
                "wednesday": "Kyckling- och gröncurry thai med ris.\r\n\r\nVeg:\r\nBlomkål- och gröncurry thai med ris.",
                "friday": "Kryddiga korvar med potatissallad och paprikamajo.\r\n\r\nVeg:\r\nKryddig sojakorv med potatissallad och paprikamajo.",
                "id": -1,
                "monday": "Pasta med strimlat fläskkött och pepparsås.\r\n\r\nVeg:\r\nPasta med vegobitar och pepparsås."
            }
        ]"#;

        let lunch_menu = LunchMenu::deserialize(data).unwrap();
        assert_eq!(lunch_menu.week, 8);
        assert_eq!(
            lunch_menu.created_at,
            chrono::NaiveDate::from_ymd_opt(2024, 2, 16)
                .unwrap()
                .and_hms_opt(15, 3, 15)
                .unwrap()
        );

        assert_eq!(
            lunch_menu.monday.date,
            chrono::NaiveDate::from_ymd_opt(2024, 2, 19).unwrap()
        );
        assert_eq!(lunch_menu.monday.food, "Pasta med strimlat fläskkött och pepparsås.\r\n\r\nVeg:\r\nPasta med vegobitar och pepparsås.");

        assert_eq!(
            lunch_menu.tuesday.date,
            chrono::NaiveDate::from_ymd_opt(2024, 2, 20).unwrap()
        );
        assert_eq!(lunch_menu.tuesday.food, "Het köttfärssoppa med kökets bröd.\r\n\r\nVeg:\r\nHet bön och rotfruktssoppa med kökets bröd.");

        assert_eq!(
            lunch_menu.wednesday.date,
            chrono::NaiveDate::from_ymd_opt(2024, 2, 21).unwrap()
        );
        assert_eq!(lunch_menu.wednesday.food, "Kyckling- och gröncurry thai med ris.\r\n\r\nVeg:\r\nBlomkål- och gröncurry thai med ris.");

        assert_eq!(
            lunch_menu.thursday.date,
            chrono::NaiveDate::from_ymd_opt(2024, 2, 22).unwrap()
        );
        assert_eq!(lunch_menu.thursday.food, "Pestobakad fisk med vitvinsås och pasta penne.\r\n\r\nVeg:\r\nGrönsaksbiffar med vitvinsås och pasta penne.");

        assert_eq!(
            lunch_menu.friday.date,
            chrono::NaiveDate::from_ymd_opt(2024, 2, 23).unwrap()
        );
        assert_eq!(lunch_menu.friday.food, "Kryddiga korvar med potatissallad och paprikamajo.\r\n\r\nVeg:\r\nKryddig sojakorv med potatissallad och paprikamajo.");
    }
}

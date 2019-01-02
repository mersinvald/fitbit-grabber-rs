use super::FitbitClient;
use crate::date;

pub trait Activities {
    fn get_daily_activity_summary(&self, user_id: &str, date: &date::Date)
        -> crate::Result<String>;
    // TODO: type for period
    fn get_minutes_sedentary_intraday(
        &self,
        user_id: &str,
        date: &date::Date,
        period: &str,
    ) -> crate::Result<String>;
    fn get_log_calories_intraday(
        &self,
        user_id: &str,
        date: &date::Date,
        period: &str,
    ) -> crate::Result<String>;
}

impl Activities for FitbitClient {
    fn get_daily_activity_summary(
        &self,
        user_id: &str,
        date: &date::Date,
    ) -> crate::Result<String> {
        let path = format!("user/{}/activities/date/{}.json", user_id, date);
        self.do_get(&path)
    }

    fn get_minutes_sedentary_intraday(
        &self,
        user_id: &str,
        date: &date::Date,
        period: &str,
    ) -> crate::Result<String> {
        let path = format!(
            "user/{}/activities/minutesSedentary/date/{}/{}/{}.json",
            user_id, date, date, period
        );
        self.do_get(&path)
    }

    fn get_log_calories_intraday(
        &self,
        user_id: &str,
        date: &date::Date,
        period: &str,
    ) -> crate::Result<String> {
        let path = format!(
            "user/{}/activities/log/calories/date/{}/{}/{}.json",
            user_id, date, date, period
        );
        self.do_get(&path)
    }
}

use super::FitbitClient;
use crate::date;

pub trait Sleep {
    fn get_sleep_log(&self, user_id: &str, date: &date::Date) -> crate::Result<String>;
}

impl Sleep for FitbitClient {
    fn get_sleep_log(&self, user_id: &str, date: &date::Date) -> crate::Result<String> {
        let path = format!("user/{}/sleep/date/{}.json", user_id, date);
        self.do_get_1_2(&path)
    }
}

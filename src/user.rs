use super::FitbitClient;
use crate::errors::Error;
use serde::{Deserialize, Serialize};

/// UserProfile is a partial serialization struct of the Fitbit API profile. See:
/// https://dev.fitbit.com/build/reference/web-api/user/
#[allow(missing_copy_implementations)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    age: i64,
    #[serde(rename = "offsetFromUTCMillis")]
    utc_offset: i64,
    // TODO: lots of fields...
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfileResult {
    user: UserProfile,
}

pub trait User {
    fn get_user_profile(&self) -> Result<UserProfileResult, Error>;
}

impl User for FitbitClient {
    fn get_user_profile(&self) -> Result<UserProfileResult, Error> {
        let url = self.base_1.join("user/-/profile.json")?;
        Ok(self.client.get(url).send()?.json()?)
    }
}

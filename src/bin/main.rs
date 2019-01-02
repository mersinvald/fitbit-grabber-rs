use failure::{format_err, Error};

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use chrono::NaiveDate;
use clap::{App, Arg, SubCommand};
use directories::ProjectDirs;

mod config;
use crate::config::Config;
use fitbit::activities::Activities;
use fitbit::date;
use fitbit::user::User;

fn main() -> Result<(), Error> {
    env_logger::init();
    let project_dirs =
        ProjectDirs::from("", "", "fitbit-grabber")
            .ok_or_else(|| format_err!("app dirs do not exist"))?;
    let config_path = project_dirs.config_dir();
    let default_config = config_path.join("conf.toml");
    let date_arg = Arg::with_name("date")
        .long("date")
        .required(true)
        .takes_value(true)
        .help("date to fetch data for");

    let matches = App::new("Fitbit Grabber")
        .arg(
            Arg::with_name("config")
                .help("path to config file")
                .short("c")
                .long("config")
                .default_value(default_config.to_str().unwrap()),
        )
        .subcommand(
            SubCommand::with_name("heart")
                .about("fetch heart data")
                .arg(date_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("step")
                .about("fetch step data")
                .arg(date_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("weight")
                .about("fetch body weight data")
                .arg(date_arg.clone()),
        )
        .subcommand(SubCommand::with_name("token").about("request an access token"))
        .subcommand(SubCommand::with_name("refresh-token").about("refresh token"))
        .subcommand(SubCommand::with_name("user").about("get user profile"))
        .subcommand(
            SubCommand::with_name("daily-activity-summary")
                .about("get daily activity summary")
                .arg(date_arg.clone()),
        )
        .get_matches();

    let conf = Config::load(matches.value_of("config"))?;
    let config::FitbitConfig {
        client_id,
        client_secret,
    } = conf.fitbit.unwrap();
    let auth = fitbit::FitbitAuth::new(&client_id.unwrap(), &client_secret.unwrap());

    if matches.subcommand_matches("token").is_some() {
        let token = auth.get_token()?;
        save_token(".token", &token)?;
    }

    if matches.subcommand_matches("refresh-token").is_some() {
        let token = load_token(".token")?;
        let exchanged = auth.exchange_refresh_token(token)?;
        save_token(".token", &exchanged)?;
    }

    let token = load_token(".token").unwrap();
    let f = fitbit::FitbitClient::new(&token)?;

    if let Some(matches) = matches.subcommand_matches("heart") {
        let raw_date = matches
            .value_of("date")
            .ok_or_else(|| format_err!("please give a starting date"))?;
        let date = NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d")?;
        let heart_rate_data = f.heart(date)?;
        println!("{}", heart_rate_data);
    }

    if let Some(matches) = matches.subcommand_matches("step") {
        let raw_date = matches
            .value_of("date")
            .ok_or_else(|| format_err!("please give a starting date"))?;
        let date = NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d")
            .map_err(|e| format_err!("could not parse date {}", e))?;
        let step_data = f.step(date)?;
        println!("{}", step_data);
    }

    if matches.subcommand_matches("user").is_some() {
        let profile = f.get_user_profile()?;
        println!("{:?}", profile);
    }

    if let Some(matches) = matches.subcommand_matches("daily-activity-summary") {
        let raw_date = matches
            .value_of("date")
            .ok_or_else(|| format_err!("please give a starting date"))?;
        let date = date::Date::from_str(raw_date)?;
        let summary = f.get_daily_activity_summary("-", &date)?;
        println!("{}", summary);
    }

    Ok(()) // ok!
}

fn save_token(filename: &str, token: &oauth2::Token) -> Result<(), Error> {
    let json = serde_json::to_string(&token).unwrap();
    let path = Path::new(filename);
    File::create(&path).and_then(|mut file| file.write_all(json.as_bytes()))?;
    Ok(())
}

fn load_token(filename: &str) -> Result<fitbit::Token, Error> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("unable to read file");

    Ok(serde_json::from_str::<fitbit::Token>(contents.trim())?)
}

use std::process::Command;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;
use serde_json::Value;
use chrono::offset::Utc;
use chrono::DateTime;


fn main() {
    println!("Grabbing data from NYS");
    Command::new("/root/givevaccine/grabdata.sh").output().expect("Failed to grab data from nys site");
    let path = Path::new("./out.json");
    let file = File::open(path);
    let file = match file {
        Err(error) => panic!("Couldn't open json file"),
        Ok(file) => file
    };
    let reader = BufReader::new(file);
    let data:Value = match serde_json::from_reader(reader) {
        Ok(x) => x,
        Err(e) => panic!("Failed to parse json file into serde Value")
    };
    let providers = match data.get("providerList"){
        Some(list) => list,
        None => panic!("Got json from NYS but no provider list found")
    };
    let alb_apt = match &providers[3].get("availableAppointments"){
        Some(value) => value.as_str().unwrap().eq("AA"),
        None => panic!("Json structure was not as expected")
    };

    let system_time = SystemTime::now();
    let curr_time: DateTime<Utc> = system_time.into();

    match alb_apt {
        true => {
            println!("Vaccine appointment available; sending notification");
            Command::new("/root/givevaccine/notify.sh").output().expect("Vaccine available but notification script failed");
        }
        false => {
            println!("Vaccine appointment unavailable as of:");
            println!("{}", curr_time.format("%d/%m/%Y %T"));
        }
    }
}

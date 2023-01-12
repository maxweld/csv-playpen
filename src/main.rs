use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
// use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct RateRecord {
    #[serde(rename = "Membership Code")]
    membership: String,
    #[serde(rename = "Centre Id")]
    centre: String,
    name: String,
    #[serde(rename = "Total Price")]
    total_price: f64,
    price: f64,
    from: String,
    to: Option<String>,
    #[serde(rename = "Price Date")]
    price_date: Option<String>,
    season: Option<String>,
    #[serde(rename = "Budget Code")]
    budget_code: String,
    #[serde(rename = "Cost Centre")]
    cost_centre: Option<String>,
    #[serde(rename = "No of Payments")]
    no_of_payments: i32,
    #[serde(rename = "Type")]
    membership_type: String,
    duration: String,
    category: String,
    period: String,
    class: Option<String>,
    status: String,
    #[serde(rename = "Start Date")]
    start_date: String,
    #[serde(rename = "Application Date")]
    application_date: String,
    // #[serde(rename = "Membership Rules")]
    // membership_rules: Vec<u8>,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let mut rates_records: Vec<RateRecord> = Vec::new();

    for result in rdr.deserialize() {
        let record: RateRecord = result?;
        println!("{:#?}", record);
        rates_records.push(record);
    }

    println!("{:#?}", rates_records[3]);

    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

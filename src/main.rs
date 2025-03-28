use reqwest;
use serde_json::{self};
use std::env;
mod country_names;
use crate::person::Person;
mod person;

// TODO add information flag that prints competitioninfo such as whether reg is still open...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "reg: Unexpected number of arguments. Expected 1 but got {}",
            args.len() - 1
        );
        std::process::exit(1);
    }

    list_competitors(args[1].to_string()).await?;
    Ok(())
}

async fn list_competitors(competition_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let formatted_url: String = format!(
        "https://worldcubeassociation.org/api/v0/competitions/{}/wcif/public",
        competition_id
    );
    let url: String = formatted_url;
    println!("Requesting data...");
    let resp = reqwest::get(url).await?;

    if !resp.status().is_success() {
        println!("Failed to fetch data: {}", resp.status());
        std::process::exit(1);
    }

    let data: serde_json::Value = resp
        .json()
        .await?;

    let competition_name = data["name"]
        .to_string()
        .replace('\"', "");

    print!("\x1B[1A");
    print!("\r");
    println!("Showing results for {}:\n", competition_name);

    let registrants: Vec<Person> = data["persons"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(Person::from)
        .collect();

    let first_timers = registrants
        .iter()
        .filter(|r| r.wca_id == "null" && r.is_competing)
        .count();

    let total_registrants = registrants
        .iter()
        .filter(|r| r.is_competing)
        .count();

    let competitor_limit = data["competitorLimit"]
        .to_string()
        .parse::<u16>()
        .unwrap();

    for person in registrants.iter().filter(|r| r.is_competing) {
        println!(
            "{}: {}, {} ({})",
            person.id,
            person.name,
            person.country_id,
            if person.wca_id == "null" { "First-timer" } else { &person.wca_id }
        );
    }

    println!("\nTotal registations: {}/{}", total_registrants, competitor_limit);
    println!("First-time competitors: {}", first_timers);

    Ok(())
}

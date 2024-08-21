use reqwest;
use serde_json::{self};
use std::env;
mod country_names;
mod person;

// TODO add information flag that prints competitioninfo such as whether reg is still open...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  TODO: Sanitize input
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        list_competitors(args[1].to_string()).await?;
        std::process::exit(1);
    }

    println!(
        "reg: Unexpected number of arguments. Expected 1 but got {}",
        args.len() - 1
    );
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

    if resp.status().is_success() {
        let data: serde_json::Value = resp.json().await?;
        let competition_name = data["name"].to_string().replace("\"", "");
        println!("\n----------------------------------\n");
        println!("Showing results for {}:\n", competition_name);
        // Create and print persons
        if let Some(persons) = data["persons"].as_array() {
            let mut non_competing_persons: usize = 0;
            let mut first_timers: u32 = 0;
            for person in persons {
                if format!("{}", person["registration"]).as_str() == "null" {
                    non_competing_persons += 1;
                    continue;
                }

                let person = person::new(
                    person["registrantId"].to_string(),
                    person["name"].to_string().replace("\"", ""),
                    person["wcaId"].to_string().replace("\"", ""),
                    country_names::common_name(&person["countryIso2"].as_str().unwrap())
                        .to_string(),
                    person["registration"]["isCompeting"].as_bool().unwrap(),
                );

                // Remove non-competing organizers and delegates
                if !person.is_competing {
                    non_competing_persons += 1;
                    continue;
                }
                
                let mut output: String = format!("{}: {}, {}", person.id, person.name, person.country_id);

                // Mark and count first-time competitors
                if person.wca_id == "null" {
                    output.push_str(" (First-Timer)");
                    first_timers += 1;
                }

                println!("{}", output);
            }

            // Count registered competitors by taking the amount of people minus the amount of
            // people not registered.
            let competitor_count: usize = persons.len() - non_competing_persons;

            // Print a message if there are no competitors
            if competitor_count <= 0 {
                println!("No registrations for {}", competition_name);
            }

            println!(
                "\nTotal registrations: {}/{}",
                competitor_count, data["competitorLimit"]
            );
            println!("First-time competitors: {}", first_timers);
        } else {
            println!("'persons' field is not in the expected format or is missing.");
        }
    } else {
        println!("Failed to fetch data: {}", resp.status());
    }
    Ok(())
}

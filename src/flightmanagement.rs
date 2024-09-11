/// Flight management is the module that allow 
/// Adding new flight, editing flight info,
/// Cancele flight, manage flight crew and shows all flights
///



use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;


pub mod flight_management {
    use super::*;

    pub async fn manage_flights(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        loop {
            println!("\nFlight Management Menu:");
            println!();
            println!("1. Add New Flight");
            println!();
            println!("2. Edit Flight Information");
            println!();
            println!("3. Cancel Flight");
            println!();
            println!("4. Manage Flight Crew");
            println!();
            println!("5. List All Flights");
            println!();
            println!("6. Exit");
            println!();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let choice = input.trim().to_lowercase();

            match choice.as_str() {
                "1" | "add new flight" | "add" | "flight" => add_new_flight(pool).await,
                "2" | "edit flight" | "edit" => edit_flight(pool).await,
                "3" | "cancel flight" | "cancel" => cancel_flight(pool).await,
                "4" | "manage flight crew" | "manage crew" | "crew" => {
                    manage_flight_crew(pool).await
                }
                "5" | "list flights" | "list" => list_all_flights(pool).await,
                "6" | "exit" | "quit" => {
                    println!("Exiting Flight Management System. Goodbye!");
                    break Ok(());
                }
                _ => println!("Invalid option! Please choose a valid option."),
            }
        }
    }

    async fn add_new_flight(pool: &SqlitePool) {  // Function to add new flight
        let mut flight_data = HashMap::new();
        flight_data.insert("flight_number", prompt("Flight Number: "));
        flight_data.insert("origin", prompt("Origin: "));
        flight_data.insert("destination", prompt("Destination: "));
        flight_data.insert("schedule", prompt("Schedule (YYYY-MM-DD HH:MM): "));
        flight_data.insert("aircraft_type", prompt("Aircraft Type: "));
        flight_data.insert("available_seats", prompt("Available Seat: "));

        let flight_uuid = Uuid::new_v4().to_string();

        let result = sqlx::query(
            r#"
            INSERT INTO flights (uuid, flight_number, origin, destination, schedule, aircraft_type)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(flight_uuid.clone())
        .bind(flight_data["flight_number"].clone())
        .bind(flight_data["origin"].clone())
        .bind(flight_data["destination"].clone())
        .bind(flight_data["schedule"].clone())
        .bind(flight_data["aircraft_type"].clone())
        .bind(flight_data["available_seats"].clone())
        .execute(pool)
        .await;

        match result {
            Ok(_) => println!("Flight added successfully with UUID: {}", flight_uuid),
            Err(e) => println!("Error adding flight: {:?}", e),
        }
    }

    async fn edit_flight(pool: &SqlitePool) {
        let flight_number = prompt("Enter the flight number to edit: ");

        let flight = sqlx::query("SELECT * FROM flights WHERE flight_number = ?")
            .bind(&flight_number)
            .fetch_one(pool)
            .await;

        match flight {
            Ok(row) => {
                let mut flight_data = HashMap::new();
                flight_data.insert("origin", row.get::<String, _>("origin"));
                flight_data.insert("destination", row.get::<String, _>("destination"));
                flight_data.insert("schedule", row.get::<String, _>("schedule"));
                flight_data.insert("aircraft_type", row.get::<String, _>("aircraft_type"));

                println!("Editing Flight: {}", row.get::<String, _>("flight_number"));
                for (key, value) in flight_data.clone() {
                    let new_value = prompt(&format!("{} [{}]: ", key, value));
                    if !new_value.trim().is_empty() {
                        flight_data.insert(key, new_value);
                    }
                }

                let result = sqlx::query(
                    r#"
                    UPDATE flights SET origin = ?, destination = ?, schedule = ?, aircraft_type = ?
                    WHERE flight_number = ?
                    "#,
                )
                .bind(flight_data["origin"].clone())
                .bind(flight_data["destination"].clone())
                .bind(flight_data["schedule"].clone())
                .bind(flight_data["aircraft_type"].clone())
                .bind(&flight_number)
                .execute(pool)
                .await;

                match result {
                    Ok(_) => println!("Flight updated successfully!"),
                    Err(e) => println!("Error updating flight: {:?}", e),
                }
            }
            Err(e) => println!("Flight not found: {:?}", e),
        }
    }

    async fn cancel_flight(pool: &SqlitePool) {
        let flight_number = prompt("Enter the flight number to cancel: ");
        let confirmation = prompt("Are you sure you want to cancel this flight? (yes/no): ");
        if confirmation.to_lowercase() != "yes" {
            println!("Operation cancelled.");
            return;
        }

        let result = sqlx::query("DELETE FROM flights WHERE flight_number = ?")
            .bind(&flight_number)
            .execute(pool)
            .await;

        match result {
            Ok(_) => println!("Flight cancelled successfully!"),
            Err(e) => println!("Error cancelling flight: {:?}", e),
        }
    }

    async fn manage_flight_crew(pool: &SqlitePool) {
        let flight_number = prompt("Enter the flight number to manage crew: ");
        let crew_member = prompt("Enter crew member name to assign/update: ");
        let role = prompt("Enter role of the crew member (pilot/attendant/etc.): ");

        let result = sqlx::query(
            r#"
            INSERT INTO flight_crew (flight_number, crew_member, role)
            VALUES (?, ?, ?)
            ON CONFLICT(flight_number, crew_member) DO UPDATE SET role = excluded.role;
            "#,
        )
        .bind(&flight_number)
        .bind(&crew_member)
        .bind(&role)
        .execute(pool)
        .await;

        match result {
            Ok(_) => println!("Crew member assigned/updated successfully!"),
            Err(e) => println!("Error managing flight crew: {:?}", e),
        }
    }

    async fn list_all_flights(pool: &SqlitePool) {
        let flights = sqlx::query("SELECT * FROM flights").fetch_all(pool).await;

        match flights {
            Ok(rows) => {
                println!("List of All Flights:");
                for row in rows {
                    println!(
                        "Flight Number: {}, Origin: {}, Destination: {}, Schedule: {}, Aircraft Type: {}",
                        row.get::<String, _>("flight_number"),
                        row.get::<String, _>("origin"),
                        row.get::<String, _>("destination"),
                        row.get::<String, _>("schedule"),
                        row.get::<String, _>("aircraft_type"),
                    );
                }
            }
            Err(e) => println!("Error fetching flights: {:?}", e),
        }
    }

    fn prompt(message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}

use crate::analytics;
use crate::booking;
use crate::flightmanagement::flight_management;
use crate::staff;
use sqlx::SqlitePool;
use std::io::{self, Write};

pub async fn managements(pool: &SqlitePool) {
    println!();
    println!("1. Staff Management");
    println!();
    println!("2. Flight Management");
    println!();
    println!("3. Booking Management");
    println!();
    println!("4. Reports and Analytics");
    println!();
    println!("5. Exit");
    println!();

    // Beginning of management staff
    print!("Select a management option: ");

    let mut role = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut role).unwrap();
    let input_user = role.trim().to_lowercase();

    if input_user == "staff management" || input_user == "staff" || input_user == "1" {
        if let Err(e) = staff::stafu().await {
            println!("Error: {:?}", e);
        }
    } else if input_user == "flight management" || input_user == "flight" || input_user == "2" {
        // Call the flight management function
        if let Err(e) = flight_management::manage_flights(pool).await {
            println!("Error: {:?}", e);
        }
    } else if input_user == "booking management" || input_user == "booking" || input_user == "3" {
        if let Err(e) = booking::manage_bookings(pool).await {
            println!("Error: {:?}", e);
        }
    } else if input_user == "reports and analytics"
        || input_user == "analytics"
        || input_user == "4"
    {
        if let Err(e) = analytics::generate_reports_and_analytics(pool).await {
            println!("Error: {:?}", e);
        }
    } else if input_user == "exit" || input_user == "5" {
        println!("Exiting The Management Interface. Goodbye!");
    } else {
        println!("Invalid input. Please try again.");
    }
}

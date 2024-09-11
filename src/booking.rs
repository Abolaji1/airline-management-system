/// This module will handle operations related to booking management,
/// such as adding new bookings, updating existing bookings, 
///and canceling bookings


use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;

pub async fn manage_bookings(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    loop {
        println!();
        println!("1. Book by Passenger");
        println!("2. Book by Agent");
        println!("3. Book by Staff Delegate");
        println!("4. View All Bookings (Admin Only)");
        println!("5. Exit");
        println!();

        print!("Choose a booking option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "1" | "book by passenger" | "passenger" => {
                book_by_passenger(pool).await;
            }
            "2" | "book by agent" | "agent" => {
                book_by_agent(pool).await;
            }
            "3" | "book by staff delegate" | "staff" => {
                book_by_staff_delegate(pool).await;
            }
            "4" | "view all bookings" | "admin" => {
                view_all_bookings(pool).await;
            }
            "5" | "exit" => {
                println!("Exiting Booking Management. Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select a valid booking option."),
        }
    }
    Ok(())
}

async fn book_by_passenger(pool: &SqlitePool) {
    let booking_data = gather_booking_data();
    let booking_uuid = Uuid::new_v4().to_string();
    let passenger_id = prompt("Enter Passenger ID: ");
    let agent_id = "None";
    let staff_id = "None";

    // Insert booking into the database as a passengr
    insert_booking(
        pool,
        &booking_uuid,
        &passenger_id,
        &agent_id,
        &staff_id,
        booking_data,
    )
    .await;
}

async fn book_by_agent(pool: &SqlitePool) {
    let booking_data = gather_booking_data();
    let booking_uuid = Uuid::new_v4().to_string();
    let passenger_id = prompt("Enter Passenger ID: ");
    let agent_id = prompt("Enter Agent ID: ");
    let staff_id = "None";

    // Insert booking into the database as an agent
    insert_booking(
        pool,
        &booking_uuid,
        &passenger_id,
        &agent_id,
        &staff_id,
        booking_data,
    )
    .await;
}

async fn book_by_staff_delegate(pool: &SqlitePool) {
    let booking_data = gather_booking_data();
    let booking_uuid = Uuid::new_v4().to_string();
    let passenger_id = prompt("Enter Passenger ID: ");
    let agent_id = "None";
    let staff_id = prompt("Enter Staff Delegate ID: ");

    // Insert booking into the database as a staff of airline
    insert_booking(
        pool,
        &booking_uuid,
        &passenger_id,
        &agent_id,
        &staff_id,
        booking_data,
    )
    .await;
}

async fn view_all_bookings(pool: &SqlitePool) {
    let bookings = sqlx::query("SELECT * FROM bookings").fetch_all(pool).await;

    match bookings {
        Ok(rows) => {
            println!("All Bookings:");
            for row in rows {
                let booking_id: String = row.get("uuid");
                let passenger_id: String = row.get("passenger_id");
                let agent_id: String = row.get("agent_id");
                let staff_id: String = row.get("staff_id");
                let details: String = row.get("details");

                println!(
                    "Booking ID: {}, Passenger ID: {}, Agent ID: {}, Staff ID: {}, Details: {}",
                    booking_id, passenger_id, agent_id, staff_id, details
                );
            }
        }
        Err(e) => println!("Error fetching bookings: {:?}", e),
    }
}

async fn insert_booking(
    pool: &SqlitePool,
    booking_uuid: &str,
    passenger_id: &str,
    agent_id: &str,
    staff_id: &str,
    booking_data: HashMap<&str, String>,
) {
    let result = sqlx::query(
        r#"
        INSERT INTO bookings (uuid, passenger_id, agent_id, staff_id, details)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(booking_uuid)
    .bind(passenger_id)
    .bind(agent_id)
    .bind(staff_id)
    .bind(booking_data["details"].clone())
    .execute(pool)
    .await;

    match result {
        Ok(_) => println!("Booking added successfully with UUID: {}", booking_uuid),
        Err(e) => println!("Error adding booking: {:?}", e),
    }
}

fn gather_booking_data() -> HashMap<&'static str, String> {
    let mut booking_data = HashMap::new();
    booking_data.insert("details", prompt("Enter booking details: ")); // Additional fields can be added
    booking_data
}

fn prompt(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

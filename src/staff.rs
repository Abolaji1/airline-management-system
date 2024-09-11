use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
//use std::fs;
use std::io;
use std::path::Path;
use uuid::Uuid;

use dotenv::dotenv;
use std::env;

//#[tokio::main]
pub async fn stafu() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Load environment variables from .env file

    // Get the database URL from the environment variable
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    println!("Database path: {}", db_url); // Debug print to verify the path

    // Extract the directory and database file path from the database URL
    let db_path = db_url
        .strip_prefix("sqlite://")
        .expect("Invalid database URL format");

    // Check if the database file exists
    if !Path::new(db_path).exists() {
        eprintln!(
            "Database file does not exist at the specified path: {}",
            db_path
        );
        return Err(sqlx::Error::Configuration(
            "Database file does not exist.".into(),
        ));
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Connect to the SQLite database using the DATABASE_URL environment variable
    let pool = SqlitePool::connect(&db_url).await?;

    // Create the staff table if it doesn't exist, now including a UUID column
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS staff (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            middle_name TEXT,
            email TEXT UNIQUE NOT NULL,
            role TEXT NOT NULL,
            phone_number TEXT NOT NULL,
            address TEXT,
            password TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    loop {
        match staffs(&pool).await {
            StaffManagementResult::Continue => continue, // Continue the loop
            StaffManagementResult::ExitToMainMenu => break, // Break to return to the main menu
        }
    }

    Ok(())
}

pub enum StaffManagementResult {
    Continue,
    ExitToMainMenu,
}

pub async fn staffs(pool: &SqlitePool) -> StaffManagementResult {
    println!();
    println!("1. Add New Staff");
    println!();
    println!("2. Update Staff");
    println!();
    println!("3. Delete Staff");
    println!();
    println!("4. Exit");
    println!();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "1" | "add new staff" | "new staff" | "add" | "staff" => add_new_staff(pool).await,
        "2" | "update" | "update staff" => update_staff(pool).await,
        "3" | "delete" | "delete staff" => delete_staff(pool).await,
        "4" | "exit" | "log out" | "quit" => {
            println!("Exiting The Staff Management Interface. Goodbye!");

            return  StaffManagementResult::ExitToMainMenu;
        }
        _ => println!("Invalid option! - add or new staff or update or update staff or delete or delete staff or exit or quitor 1 or 2 or 3 or 4"),
    }
    StaffManagementResult::Continue // to continue operation of add new staff, update neww staff, delete
}

pub async fn add_new_staff(pool: &SqlitePool) {
    let mut staff_data = HashMap::new();
    staff_data.insert("first_name", prompt("First Name: "));
    println!();
    staff_data.insert("last_name", prompt("Last Name: "));
    println!();
    staff_data.insert("middle_name", prompt("Middle Name: "));
    println!();
    staff_data.insert("email", prompt("Email: "));
    println!();
    staff_data.insert("role", prompt("Role (admin/staff): "));
    println!();
    staff_data.insert("phone_number", prompt("Phone Number: "));
    println!();
    staff_data.insert("address", prompt("Address: "));
    println!();
    staff_data.insert("password", prompt("Password: "));
    println!();

    // Generate a new UUID for the staff member
    let staff_uuid = Uuid::new_v4().to_string();

    // Insert the new staff into the database with a UUID
    let result = sqlx::query(
        r#"
        INSERT INTO staff (uuid, first_name, last_name, middle_name, email, role, phone_number, address, password)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(staff_uuid.clone())
    .bind(staff_data["first_name"].clone())
    .bind(staff_data["last_name"].clone())
    .bind(staff_data["middle_name"].clone())
    .bind(staff_data["email"].clone())
    .bind(staff_data["role"].clone())
    .bind(staff_data["phone_number"].clone())
    .bind(staff_data["address"].clone())
    .bind(staff_data["password"].clone())
    .execute(pool)
    .await;

    match result {
        Ok(_) => println!("Staff added successfully with UUID: {}", staff_uuid),
        Err(e) => println!("Error adding staff: {:?}", e),
    }
}

async fn update_staff(pool: &SqlitePool) {
    let email = prompt("Enter the email of the staff to update: ");

    // Fetch the staff details
    let staff = sqlx::query("SELECT * FROM staff WHERE email = ?")
        .bind(&email)
        .fetch_one(pool)
        .await;

    match staff {
        Ok(row) => {
            let mut staff_data = HashMap::new();
            staff_data.insert("first_name", row.get::<String, _>("first_name"));
            staff_data.insert("last_name", row.get::<String, _>("last_name"));
            staff_data.insert("middle_name", row.get::<String, _>("middle_name"));
            staff_data.insert("role", row.get::<String, _>("role"));
            staff_data.insert("phone_number", row.get::<String, _>("phone_number"));
            staff_data.insert("address", row.get::<String, _>("address"));

            println!("Editing Staff: {}", row.get::<String, _>("first_name"));
            for (key, value) in staff_data.clone() {
                let new_value = prompt(&format!("{} [{}]: ", key, value));
                if !new_value.trim().is_empty() {
                    staff_data.insert(key, new_value);
                }
            }

            // Update the staff in the database
            let result = sqlx::query(
                r#"
                UPDATE staff SET first_name = ?, last_name = ?, middle_name = ?, role = ?, phone_number = ?, address = ?
                WHERE email = ?
                "#,
            )
            .bind(staff_data["first_name"].clone())
            .bind(staff_data["last_name"].clone())
            .bind(staff_data["middle_name"].clone())
            .bind(staff_data["role"].clone())
            .bind(staff_data["phone_number"].clone())
            .bind(staff_data["address"].clone())
            .bind(&email)
            .execute(pool)
            .await;

            match result {
                Ok(_) => println!("Staff updated successfully!"),
                Err(e) => println!("Error updating staff: {:?}", e),
            }
        }
        Err(e) => println!("Staff not found: {:?}", e),
    }
}

async fn delete_staff(pool: &SqlitePool) {
    let email = prompt("Enter the email of the staff to delete: ");
    let confirmation = prompt("Are you sure you want to delete this staff? (yes/no): ");
    if confirmation.to_lowercase() != "yes" {
        println!("Operation cancelled.");
        return;
    }

    // Delete the staff from the database
    let result = sqlx::query("DELETE FROM staff WHERE email = ?")
        .bind(&email)
        .execute(pool)
        .await;

    match result {
        Ok(_) => println!("Staff deleted successfully!"),
        Err(e) => println!("Error deleting staff: {:?}", e),
    }
}

fn prompt(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

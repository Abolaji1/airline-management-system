/// Analytics generate reports base on Booking activities.  
/// This module will handle reports and analytics, 
/// allowing management to generate reports or view analytics data.

use sqlx::Row;
use sqlx::SqlitePool;

pub async fn generate_reports_and_analytics(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    loop {

        // Given options of reporting either Booking or Staff prformance
        println!();
        println!("1. View Booking Statistics");
        println!("2. View Staff Performance");
        println!("3. Exit");
        println!();

        print!("Choose a reporting option: ");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "1" | "view booking statistics" | "booking" => {
                view_booking_statistics(pool).await;
            }
            "2" | "view staff performance" | "staff" => {
                view_staff_performance(pool).await;
            }
            "3" | "exit" => {
                println!("Exiting Reports and Analytics. Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select a valid reporting option."),
        }
    }

    Ok(())
}

async fn view_booking_statistics(pool: &SqlitePool) { //This function count all the total booking from bookings table
    let stats = sqlx::query("SELECT COUNT(*) as total_bookings FROM bookings") 
        .fetch_one(pool)
        .await;

    match stats {
        Ok(row) => {
            let total_bookings: i64 = row.get("total_bookings");
            println!("Total Bookings: {}", total_bookings);
        }
        Err(e) => println!("Error fetching booking statistics: {:?}", e),
    }
}

async fn view_staff_performance(pool: &SqlitePool) {
    let staff_performance = sqlx::query(
        "SELECT staff_id, COUNT(*) as bookings_count FROM bookings WHERE staff_id != 'None' GROUP BY staff_id", 
    )
    .fetch_all(pool)
    .await;

    match staff_performance {
        Ok(rows) => {
            println!("Staff Performance Report:");
            for row in rows {
                let staff_id: String = row.get("staff_id");
                let bookings_count: i64 = row.get("bookings_count");
                println!(
                    "Staff ID: {}, Bookings Handled: {}",
                    staff_id, bookings_count
                );
            }
        }
        Err(e) => println!("Error fetching staff performance: {:?}", e),
    }
}

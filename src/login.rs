use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use rpassword::read_password;
use std::env;
use std::io::{self, Write};
use rand::random;
use sqlx::SqlitePool; // Import SqlitePool
use sqlx::query;

use crate::management;

pub fn get_hashed_passwords() -> (String, String, String) {
    dotenv().ok();
    let password1 = env::var("PASSWORD1").expect("PASSWORD1 not found in .env");
    let password2 = env::var("PASSWORD2").expect("PASSWORD2 not found in .env");
    let password3 = env::var("PASSWORD3").expect("PASSWORD3 not found in .env");

    (password1, password2, password3)
}

pub async fn userinterface(pool: &SqlitePool) {
    // accept pool parameter
    // Define valid credentials
    let user1 = "Abolaji";
    let user2 = "Kazeem";
    let user3 = "Biola";

    println!("");
    println!("Welcome to Oragrick Airline!");

    println!("");
    println!("1. Admin [Type/Enter Admin or 1]");
    println!("");
    println!("2. Passenger [Type/Enter Passenger or 2]");
    println!("");
    println!("3. Exit  [To leave this interface Enter 3 or Exit]");
    println!("");

    // Hash the password
    let (password1_plain, password2_plain, password3_plain) = get_hashed_passwords();

    let password1_h = hash(password1_plain, DEFAULT_COST).unwrap();
    let password2_h = hash(password2_plain, DEFAULT_COST).unwrap();
    let password3_h = hash(password3_plain, DEFAULT_COST).unwrap();

    let mut attempt = 0;
    const MAX_ATTEMPT: u8 = 3;

    //begining of admin or passenger
    loop {
        print!("Admin or Passenger? : ");

        io::stdout().flush().unwrap();
        let mut role = String::new();

        io::stdin().read_line(&mut role).unwrap();
        let input_user = role.trim().to_lowercase();

        if input_user == "admin" || input_user == "1" {
            println!("Welcome! You are now at Administration Interface");

            while attempt < MAX_ATTEMPT {
                // Prompt for username
                let mut username = String::new();
                println!("");
                print!("Enter your Username: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim(); // Trim the input to remove newline characters

                // Prompt for password
                println!("");
                print!("Enter your Password: ");
                io::stdout().flush().unwrap();
                let passwordinp = read_password().unwrap();

                // Check credentials
                if (username == user1 && verify(&passwordinp, &password1_h).unwrap())
                    || (username == user2 && verify(&passwordinp, &password2_h).unwrap())
                    || (username == user3 && verify(&passwordinp, &password3_h).unwrap())
                {
                    println!("");
                    println!("Login Successful!");
                    println!("");
                    println!("Welcome {}!", username);
                    println!("");

                    // Beginning of managements
                    //let mut organizer: HashMap<String, Vec<String>> = HashMap::new();

                    management::managements(pool).await;

                    break;
                } else {
                    println!("");
                    println!("Incorrect Username or Password");
                    println!("");

                    attempt += 1;

                    if attempt == 1 {
                        println!("You have 2 more attempts to login");
                        println!("");
                    }

                    if attempt == 2 {
                        println!("Last attempt to login");
                        println!("");
                    }
                    if attempt >= MAX_ATTEMPT {
                        println!("You have exceeded the maximum login attempt.");
                        println!("");
                        println!("Your account is locked.");
                        println!("");
                        println!("Please contact admin manager!");

                        println!("");

                        return;
                    }
                }
            }
        }else if input_user == "passenger" || input_user == "2"{
            passenger_interface(pool).await;  // Call passenger interface function
        } else {
            let input_user = &input_user.to_uppercase();
            println!("");
            println!("Invalid input. Please enter 'admin' or 'passenger'.");
            println!("");
            println!("You entered {}", input_user);
            println!("");
        }
    }
}

async fn passenger_interface(pool: &SqlitePool) {
    loop {
        println!();
        println!("Passenger Menu:");
        println!("1. Register");
        println!("2. Login");
        println!("3. Exit to Main Menu");
        println!();

        let mut choice = String::new();
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => register_passenger(pool).await,
            "2" => login_passenger(pool).await,
            "3" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

async fn register_passenger(pool: &SqlitePool) {
    println!("Registering a new passenger...");

    // Prompt for passenger details
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    let mut email = String::new();
    print!("Enter your email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    //let mut password = String::new();
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    let hashed_password = hash(password, DEFAULT_COST).unwrap();

    // Store passenger details in the database
    match query("INSERT INTO passengers (name, email, password) VALUES (?, ?, ?)")
        .bind(&name.trim())
        .bind(&email.trim())
        .bind(&hashed_password)
        .execute(pool)
        .await
    {
        Ok(_) => println!("Registration successful! Please log in to see your updates."),
        Err(e) => println!("Error registering passenger: {}", e),
    }
}

async fn login_passenger(pool: &SqlitePool) {
    println!("Logging in...");

    let mut email = String::new();
    print!("Enter your email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    //let mut password = String::new();
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    // Fetch stored password hash from database
    let trimmed_email = email.trim();
    let result = query!("SELECT password FROM passengers WHERE email = ?", trimmed_email)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(Some(row)) => {
            if verify(&password, &row.password).unwrap() {
                println!("Login successful! Welcome back.");

                // Fetch flight updates or any other info after login
                fetch_passenger_updates(trimmed_email, pool).await;
            } else {
                println!("Incorrect password. Please try again.");
            }
        }
        Ok(None) => println!("No account found with this email. Please register first."),
        Err(e) => println!("Error during login: {}", e),
    }
}

async fn fetch_passenger_updates(email: &str, pool: &SqlitePool) {
    println!("Fetching updates for {}...", email);

    // Here, you'd implement the logic to fetch flight time, updates, etc.
    // For now, let's assume we simply display a message.
    println!("Your flight is scheduled for 10:00 AM tomorrow.");

    // Allow passenger to make a payment and generate ticket
    make_payment_and_generate_ticket(email, pool).await;
}

async fn make_payment_and_generate_ticket(email: &str, pool: &SqlitePool) {
    println!("Proceeding to payment...");

    // Implement payment logic and generate ticket number
    let ticket_number = format!("TICKET-{}", random::<u32>());

    println!("Payment successful! Your ticket number is {}", ticket_number);

    // Store ticket information in the database
    match query("INSERT INTO tickets (email, ticket_number) VALUES (?, ?)")
        .bind(email)
        .bind(&ticket_number)
        .execute(pool)
        .await
    {
        Ok(_) => println!("Ticket details saved successfully."),
        Err(e) => println!("Error saving ticket details: {}", e),
    }
}


/*fn capitalize(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}*/

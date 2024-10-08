
struct Car {
    make: String,
    model: String,
    year: u32,
}

fn main() -> io::Result<()> {
    let car = reading_from_console();
    
    save_car_info_to_file(&car)?;
    
    let content = read_car_info_from_file()?;
    println!("Car Information from file:\n{}", content);
    
    Ok(())
}

fn reading_from_console() -> Car {
    let mut buffer = String::new();

    // Get the make of the car
    print!("What is the make of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    // Get the model of the car
    print!("What is the model of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    // Get the year of the car
    print!("What is the year of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().expect("Please enter a valid number");
    buffer.clear();

    // Create and return the Car struct
    let car = Car { make, model, year };
    println!(
        "Your car is a {} {} from the year {}.",
        car.make, car.model, car.year
    );

    car
}

// Function to save the car struct into a file
fn save_car_info_to_file(car: &Car) -> io::Result<()> {
    let mut file = File::create("user_info.txt")?;
    let car_info = format!("Make: {}\nModel: {}\nYear: {}\n", car.make, car.model, car.year);
    file.write_all(car_info.as_bytes())?;
    Ok(())
}

// Function to read the file and return its content
fn read_car_info_from_file() -> io::Result<String> {
    let mut file = File::open("user_info.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
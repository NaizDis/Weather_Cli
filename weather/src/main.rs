mod fns;
use colored::Colorize;
use fns::{display_weather_info, get_weather_info};
use std::io;

fn main() {
    println!("{}", "Welcome To Weather Station".bright_yellow());
    loop {
        //Reading User
        println!("{}", "Please enter name of the City : ".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed To Read input!!");
        let city: &str = city.trim();
        println!(
            "{}",
            "Please enter name of the Country Code (e.g. ,IN for India) : ".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed To Read input!!");
        let country_code: &str = country_code.trim();

        //API key
        let api_key = "0ec0cc04e52ca02345374a38e81abfa0";

        //Fethcing Weather Information
        match get_weather_info(city, country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                //Error IN case of Failiure
                eprintln!("Error : {}", err);
            }
        }

        //Looping Program
        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no) : ".bright_yellow()
        );
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed To Read Input !!");
        let inp = inp.trim().to_lowercase();
        if inp != "yes" {
            println!("GoodBye!!");
            break;
        }
    }
}

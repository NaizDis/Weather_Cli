use colored::*;
use serde::Deserialize;

//Struct to parse response JSON from openweatherApi
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

//Weather description Struct
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

//Struct weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

//Struct for Wind
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_temp_emoji(temp: f64) -> &'static str {
    if temp < 0.0 {
        "❄️"
    } else if temp >= 0.0 && temp < 10.0 {
        "☁️"
    } else if temp >= 10.0 && temp < 20.0 {
        "⛅"
    } else if temp >= 20.0 && temp < 30.0 {
        "🌤️"
    } else {
        "☀️"
    }
}

//Function to get weather info OpenWeatherAPI
pub fn get_weather_info(
    city: &str,
    country: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country, api_key,
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

//Dispaly response Function
pub fn display_weather_info(respose: &WeatherResponse) {
    let description: &String = &respose.weather[0].description;
    let tmep: f64 = respose.main.temp;
    let humidity: f64 = respose.main.humidity;
    let pressure: f64 = respose.main.pressure;
    let wind_speed: f64 = respose.wind.speed;
    // Display format
    let weather_text: String = format!(
        "Weather in {} : {} {}
>Tempreature :{:.1}°C,
>Humidity : {:.1}%,
>Wind Speed : {:.1}m/s,
>pressure : {:.1}hPa",
        respose.name,
        description,
        get_temp_emoji(tmep),
        tmep,
        humidity,
        wind_speed,
        pressure
    );

    //Theme of string based on weather
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    println!("{}", weather_text_colored);
}

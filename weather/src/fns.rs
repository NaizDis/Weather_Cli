//Function to get weather info OpenWeatherAPI

use std::string;

use crate::WeatherResponse;

fn get_weather_info(
    city: &str,
    country: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: string = format! {};
}

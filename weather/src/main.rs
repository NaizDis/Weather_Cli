mod fns;
use colored::*;
use serde::Deserialize;
use std::io;

//Struct to parse response JSON from openweatherApi
#[derive(Deserialize, Debug)]
struct WeatherResponse {
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
    humdity: f64,
    pressure: f64,
}

//Struct for Wind
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

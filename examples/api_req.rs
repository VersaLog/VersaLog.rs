use serde::{Deserialize, Serialize};
use std::error::Error;
use versalogrs::NewVersaLog;

#[derive(Debug, Deserialize)]
struct WeatherData {
    name: String,
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    humidity: i32,
    pressure: i32,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let logger = NewVersaLog(
        "detailed",
        false,
        true,
        "Request",
        false,
        false,
        false,
        vec![],
    );

    let api = "http://api.openweathermap.org/data/2.5/weather";

    let params = [
        ("q", "location name"),
        ("appid", "api key"),
        ("units", "metric"),
        ("lang", "ja"),
    ];

    let client = reqwest::Client::new();
    let req = client.get(api).query(&params).send().await?;

    if req.status().is_success() {
        let data: WeatherData = req.json().await?;

        let location_name = data.name;
        let weather_description = &data.weather[0].description;
        let temperature = data.main.temp;
        let humidity = data.main.humidity;
        let pressure = data.main.pressure;
        let wind_speed = data.wind.speed;

        logger.info("success", &[]);

        let msg = format!(
            "< {}の天気予報 >\n\n> 天気\n・{}\n\n> 気温\n・{}°C\n\n> 湿度\n・{}%\n\n> 気圧\n・{} hPa\n\n> 風速\n・{} m/s",
            location_name, weather_description, temperature, humidity, pressure, wind_speed
        );

        println!("{}", msg);
    } else {
        logger.error("failed", &[]);
    }

    Ok(())
}

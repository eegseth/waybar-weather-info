use std::fs::{metadata, read_to_string, File};
use std::io::Write;
use std::process::exit;
use std::time::{Duration, SystemTime};

use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;

use crate::cli::Args;

mod cli;
mod constants;
mod format;
mod lang;

pub struct CurrentWeather {
    pub temperature: f64,
    pub symbol_code: String,
    pub wind_speed: f64,
    pub humidity: f64,
    pub precipitation: f64,
}

fn main() {
    let args = Args::parse();

    let (lat, lon) = match &args.location {
        Some(loc) => parse_location(loc),
        None => get_location_from_ip(),
    };

    let weather_data = fetch_weather_data(lat, lon);
    let current = extract_current_weather(&weather_data);
    
    let text = format::format_indicator(&current, &args.indicator_style, &args.temp_format);
    let tooltip = format::build_tooltip(&weather_data, &args.lang, &args.tooltip_style, &args.temp_format);
    
    // Output JSON for Waybar
    println!(
        "{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\":\"{}\"}}",
        text,
        tooltip.replace('\n', "\\n").replace('"', "\\\""),
        format::get_weather_class(&current.symbol_code)
    );
}

fn parse_location(location: &str) -> (f64, f64) {
    if location.contains(',') {
        let parts: Vec<&str> = location.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(lat), Ok(lon)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return (lat, lon);
            }
        }
        eprintln!("Error: Invalid coordinate format. Expected 'lat,lon'");
        exit(1);
    }
    
    // Location ID format not supported - use as fallback to IP geolocation
    eprintln!("Warning: Location ID '{}' not directly supported. Using IP-based geolocation.", location);
    get_location_from_ip()
}

fn get_location_from_ip() -> (f64, f64) {
    const CACHE_FILE: &str = "/tmp/waybar-weather-location.json";
    const CACHE_DURATION_SECS: u64 = 3600; // 1 hour
    const DEFAULT_LAT: f64 = 59.911491; // Oslo, Norway
    const DEFAULT_LON: f64 = 10.757933;
    
    // Try to use cached location
    if let Ok(cache_content) = read_to_string(CACHE_FILE) {
        if let Ok(metadata) = metadata(CACHE_FILE) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    if elapsed.as_secs() < CACHE_DURATION_SECS {
                        if let Ok(json) = serde_json::from_str::<Value>(&cache_content) {
                            let lat = json["latitude"].as_f64().unwrap_or(DEFAULT_LAT);
                            let lon = json["longitude"].as_f64().unwrap_or(DEFAULT_LON);
                            return (lat, lon);
                        }
                    }
                }
            }
        }
    }
    
    // Fetch fresh location from IP geolocation service
    let client = Client::new();
    if let Ok(response) = client.get("https://ipapi.co/json/").send() {
        if let Ok(json) = response.json::<Value>() {
            let lat = json["latitude"].as_f64().unwrap_or(DEFAULT_LAT);
            let lon = json["longitude"].as_f64().unwrap_or(DEFAULT_LON);
            
            // Cache the location
            let cache_data = serde_json::json!({
                "latitude": lat,
                "longitude": lon,
            });
            let _ = std::fs::write(CACHE_FILE, cache_data.to_string());
            
            return (lat, lon);
        }
    }
    
    // Default to Oslo, Norway if geolocation fails
    eprintln!("Warning: Could not determine location from IP. Using Oslo, Norway as default.");
    (DEFAULT_LAT, DEFAULT_LON)
}

fn fetch_weather_data(lat: f64, lon: f64) -> Value {
    const CACHE_DURATION_SECS: u64 = 900; // 15 minutes
    let cache_file = format!("/tmp/waybar-weather-{}-{}.json", lat, lon);
    
    // Try to use cached data
    if let Ok(json_str) = read_to_string(&cache_file) {
        if let Ok(metadata) = metadata(&cache_file) {
            if let Ok(modified) = metadata.modified() {
                let cache_age = SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or(Duration::from_secs(u64::MAX));
                
                if cache_age.as_secs() < CACHE_DURATION_SECS {
                    if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
                        return json;
                    }
                }
            }
        }
    }
    
    // Fetch from MET Norway API
    let url = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
        lat, lon
    );
    
    let client = Client::builder()
        .user_agent("waybar-weather-info/0.1.0")
        .build()
        .expect("Failed to create HTTP client");
    
    let weather = match client.get(&url).send() {
        Ok(response) => match response.json::<Value>() {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error parsing weather data: {}", e);
                println!("{{\"text\":\"❌\", \"tooltip\":\"Failed to parse weather data\"}}");
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error fetching weather data: {}", e);
            println!("{{\"text\":\"❌\", \"tooltip\":\"Failed to fetch weather data\"}}");
            exit(1);
        }
    };
    
    // Cache the result
    if let Ok(mut file) = File::create(&cache_file) {
        if let Ok(json_str) = serde_json::to_string_pretty(&weather) {
            let _ = file.write_all(json_str.as_bytes());
        }
    }
    
    weather
}

fn extract_current_weather(data: &Value) -> CurrentWeather {
    let current = &data["properties"]["timeseries"][0]["data"];
    let instant = &current["instant"]["details"];
    let next_1h = &current["next_1_hours"];
    
    CurrentWeather {
        temperature: instant["air_temperature"].as_f64().unwrap_or(0.0),
        symbol_code: next_1h["summary"]["symbol_code"]
            .as_str()
            .unwrap_or("cloudy")
            .to_string(),
        wind_speed: instant["wind_speed"].as_f64().unwrap_or(0.0),
        humidity: instant["relative_humidity"].as_f64().unwrap_or(0.0),
        precipitation: next_1h["details"]["precipitation_amount"]
            .as_f64()
            .unwrap_or(0.0),
    }
}


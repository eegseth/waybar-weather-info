use serde_json::Value;
use crate::cli::{IndicatorStyle, TooltipStyle, TempFormat};
use crate::constants::WEATHER_SYMBOL_MAP;
use crate::lang::Lang;
use crate::CurrentWeather;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn format_temp(temp_c: f64, format: &TempFormat) -> String {
    match format {
        TempFormat::Celsius => format!("{}°C", temp_c.round() as i32),
        TempFormat::Fahrenheit => format!("{}°F", celsius_to_fahrenheit(temp_c).round() as i32),
    }
}

fn format_temp_short(temp_c: f64, format: &TempFormat) -> i32 {
    match format {
        TempFormat::Celsius => temp_c.round() as i32,
        TempFormat::Fahrenheit => celsius_to_fahrenheit(temp_c).round() as i32,
    }
}

fn get_symbol_from_data(data: &Value) -> &str {
    data["next_1_hours"]["summary"]["symbol_code"]
        .as_str()
        .or_else(|| data["next_6_hours"]["summary"]["symbol_code"].as_str())
        .unwrap_or("cloudy")
}

fn display_in_columns(tooltip: &mut String, entries: &[String], columns: usize, width: usize) {
    let rows = entries.len().div_ceil(columns);
    
    for row in 0..rows {
        for col in 0..columns {
            let idx = col * rows + row;
            if idx < entries.len() {
                tooltip.push_str(&format!("{:<width$}", entries[idx], width = width));
            }
        }
        tooltip.push('\n');
    }
}

pub fn format_indicator(current: &CurrentWeather, style: &IndicatorStyle, temp_format: &TempFormat) -> String {
    let icon = get_weather_icon(&current.symbol_code);
    let temp_str = format_temp(current.temperature, temp_format);
    
    match style {
        IndicatorStyle::Concise => {
            format!("{} {} 💨{:.0}m/s", icon, temp_str, current.wind_speed)
        }
        IndicatorStyle::Detailed => {
            format!("{} {} 💧{:.1}mm 💨{:.0}m/s", 
                icon, temp_str, current.precipitation, current.wind_speed)
        }
        IndicatorStyle::Full => {
            format!("{} {} 💧{:.1}mm 💨{:.0}m/s 💦{:.0}%", 
                icon, temp_str, current.precipitation, current.wind_speed, current.humidity)
        }
    }
}

pub fn build_tooltip(data: &Value, lang: &Lang, style: &TooltipStyle, temp_format: &TempFormat) -> String {
    let mut tooltip = String::new();
    
    // Current weather
    let current_data = &data["properties"]["timeseries"][0]["data"];
    let instant = &current_data["instant"]["details"];
    let next_1h = &current_data["next_1_hours"];
    
    let temp = instant["air_temperature"].as_f64().unwrap_or(0.0);
    let temp_str = format_temp(temp, temp_format);
    let symbol = next_1h["summary"]["symbol_code"].as_str().unwrap_or("cloudy");
    let wind = instant["wind_speed"].as_f64().unwrap_or(0.0);
    let humidity = instant["relative_humidity"].as_f64().unwrap_or(0.0);
    let precip = next_1h["details"]["precipitation_amount"].as_f64().unwrap_or(0.0);
    
    tooltip.push_str(&format!("<b>{}</b>\n", symbol_to_description(symbol, lang)));
    tooltip.push_str(&format!("{}: {}\n", lang.temperature(), temp_str));
    tooltip.push_str(&format!("{}: {:.1} m/s\n", lang.wind(), wind));
    tooltip.push_str(&format!("{}: {:.0}%\n", lang.humidity(), humidity));
    tooltip.push_str(&format!("{}: {:.1} mm\n", lang.precipitation(), precip));
    
    // Forecast based on tooltip style
    match style {
        TooltipStyle::CurrentDay => {
            // Show rest of today (next 12 hours)
            tooltip.push_str("\n<b>Next hours:</b>\n");
            build_hourly_forecast(&mut tooltip, data, 12, temp_format);
        }
        TooltipStyle::ThreeDays => {
            // Show next 72 hours (3 days at 3-hour intervals = 24 entries)
            tooltip.push_str("\n<b>Next 3 days:</b>\n");
            build_extended_forecast(&mut tooltip, data, 24, 3, temp_format);
        }
        TooltipStyle::Week => {
            // Show next 168 hours (7 days at 6-hour intervals = 28 entries)
            tooltip.push_str("\n<b>Next week:</b>\n");
            build_extended_forecast(&mut tooltip, data, 28, 6, temp_format);
        }
    }
    
    tooltip
}

fn build_hourly_forecast(tooltip: &mut String, data: &Value, hours: usize, temp_format: &TempFormat) {
    let Some(timeseries) = data["properties"]["timeseries"].as_array() else {
        return;
    };
    
    let mut entries = Vec::new();
    
    for (i, entry) in timeseries.iter().take(hours + 1).enumerate() {
        if i == 0 {
            continue; // Skip current hour
        }
        
        let time = entry["time"].as_str().unwrap_or("");
        let hour = if time.len() >= 13 { &time[11..13] } else { "?" };
        
        let temp_c = entry["data"]["instant"]["details"]["air_temperature"]
            .as_f64()
            .unwrap_or(0.0);
        let temp_display = format_temp_short(temp_c, temp_format);
        
        let symbol = get_symbol_from_data(&entry["data"]);
        let icon = get_weather_icon(symbol);
        
        entries.push(format!("{}:00 {} {}°", hour, icon, temp_display));
    }
    
    // Display in 3 columns, sorted vertically
    display_in_columns(tooltip, &entries, 3, 15);
}

fn build_extended_forecast(tooltip: &mut String, data: &Value, max_entries: usize, interval_hours: usize, temp_format: &TempFormat) {
    use chrono::{DateTime, Utc};
    
    if let Some(timeseries) = data["properties"]["timeseries"].as_array() {
        if timeseries.is_empty() {
            return;
        }
        
        // Get the first timestamp as our reference
        let first_time = timeseries[0]["time"].as_str().unwrap_or("");
        let start_time = if let Ok(dt) = first_time.parse::<DateTime<Utc>>() {
            dt
        } else {
            return;
        };
        
        let mut entries = Vec::new();
        let mut count = 0;
        let mut target_offset_hours = interval_hours as i64; // Start at first interval
        
        for entry in timeseries.iter().skip(1) {
            if count >= max_entries {
                break;
            }
            
            let time_str = entry["time"].as_str().unwrap_or("");
            let entry_time = if let Ok(dt) = time_str.parse::<DateTime<Utc>>() {
                dt
            } else {
                continue;
            };
            
            let hours_elapsed = (entry_time - start_time).num_hours();
            
            if hours_elapsed >= target_offset_hours {
                let date_hour = if time_str.len() >= 16 {
                    format!("{} {}", &time_str[5..10], &time_str[11..13])
                } else {
                    "?".to_string()
                };
                
                let temp_c = entry["data"]["instant"]["details"]["air_temperature"]
                    .as_f64()
                    .unwrap_or(0.0);
                let temp_display = format_temp_short(temp_c, temp_format);
                
                let symbol = get_symbol_from_data(&entry["data"]);
                let icon = get_weather_icon(symbol);
                
                entries.push(format!("{} {} {}°", date_hour, icon, temp_display));
                
                count += 1;
                target_offset_hours += interval_hours as i64;
            }
        }
        
        display_in_columns(tooltip, &entries, 2, 18);
    }
}

pub fn get_weather_icon(symbol_code: &str) -> &str {
    // Remove _day/_night/_polar suffix if present
    let base_symbol = symbol_code
        .trim_end_matches("_day")
        .trim_end_matches("_night")
        .trim_end_matches("_polartwilight");
    
    WEATHER_SYMBOL_MAP
        .iter()
        .find(|(code, _)| *code == base_symbol)
        .map(|(_, icon)| *icon)
        .unwrap_or("🌡️")
}

pub fn get_weather_class(symbol_code: &str) -> &str {
    let base = symbol_code
        .trim_end_matches("_day")
        .trim_end_matches("_night")
        .trim_end_matches("_polartwilight");
    
    if base.contains("clearsky") {
        "clear"
    } else if base.contains("fair") {
        "fair"
    } else if base.contains("cloudy") {
        "cloudy"
    } else if base.contains("rain") || base.contains("sleet") {
        "rain"
    } else if base.contains("snow") {
        "snow"
    } else if base.contains("thunder") {
        "thunder"
    } else if base.contains("fog") {
        "fog"
    } else {
        "weather"
    }
}

fn symbol_to_description<'a>(symbol: &str, lang: &'a Lang) -> &'a str {
    lang.weather_desc(symbol)
}

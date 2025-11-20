use crate::lang::Lang;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum IndicatorStyle {
    Concise,
    Detailed,
    Full,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TooltipStyle {
    CurrentDay,
    ThreeDays,
    Week,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TempFormat {
    Celsius,
    Fahrenheit,
}

#[derive(Parser, Debug)]
#[command(
    author = "Endre Egset",
    version,
    about = "Weather indicator for Waybar using yr.no",
    long_about = None
)]
pub struct Args {
    #[arg(
        long,
        help = "Location ID from yr.no (e.g. '1-72837') or geo-coordinates (e.g. '59.911561,10.7492741'). If not specified, uses IP-based geolocation"
    )]
    pub location: Option<String>,

    #[arg(
        long,
        default_value = "concise",
        help = "Indicator style shown in waybar (concise, detailed, full)"
    )]
    pub indicator_style: IndicatorStyle,

    #[arg(
        long,
        default_value = "en",
        help = "Language (en, nb, nn, sme, fr, de, es)"
    )]
    pub lang: Lang,

    #[arg(
        long,
        default_value = "current-day",
        help = "Tooltip detail level (current-day, three-days, week)"
    )]
    pub tooltip_style: TooltipStyle,

    #[arg(
        long,
        default_value = "celsius",
        help = "Temperature format (celsius, fahrenheit)"
    )]
    pub temp_format: TempFormat,
}

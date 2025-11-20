<h1 align="center">
waybar-weather-info
</h1>

<p align="center">
Weather indicator for <a href="https://github.com/Alexays/Waybar/">Waybar</a> using <a href="https://yr.io/">yr.no</a>.
</p>
<p align="center">
Current day forecast:<br/>
<img src="https://github.com/eegseth/waybar-weather-info/blob/main/img/current-day.jpg" alt="current day forecast"/>
<br/><br/>
3 Days forecast:<br/>
<img src="https://github.com/eegseth/waybar-weather-info/blob/main/img/3days.jpg" alt="3 day forecast"/>
<br/><br/>
1 Week forecast:<br/>
<img src="https://github.com/eegseth/waybar-weather-info/blob/main/img/week.jpg" alt="Weekly forecast"/>
<hr />

## Installation

Compile yourself using `cargo build --release`, or download the precompiled binary from the [releases](https://github.com/eegseth/waybar-weather-info/releases) page.

For Arch Linux, use the [AUR](https://aur.archlinux.org/packages/waybar-weather-info) package.

### Dependencies

The binary is mostly self-contained with all Rust dependencies statically linked. The only system dependencies are:
- **OpenSSL** (libssl.so.3 / libcrypto.so.3) - for HTTPS connections
- **libc** and **libgcc** - standard system libraries

These are typically already installed on most Linux systems. No additional packages need to be installed by users.

## Usage
### Get location ID (optional)
Go to [yr.no](https://yr.no), search for your desired location, and copy the ID from the URL. 

**Example:** If the URL is `https://www.yr.no/en/forecast/daily-table/1-72837/Norway/Oslo/Oslo/Oslo`, then the ID is `1-72837`

If you don't provide a location, the module will use IP-based geolocation to determine your coordinates.

### Command line options
- `--location STRING` - location ID from yr.no (e.g. `1-72837`) or geo-coordinates (e.g. `59.911561,10.7492741`). If not specified, uses IP-based geolocation
- `--indicator-style STRING` - indicator style shown in waybar: `concise`, `detailed`, or `full` (default: `concise`)
  - `concise`: Shows icon, temperature, and wind (e.g. `☀️ -2°C 💨3m/s`)
  - `detailed`: Adds precipitation (e.g. `☀️ -2°C 💧0.0mm 💨3m/s`)
  - `full`: Adds humidity (e.g. `☀️ -2°C 💧0.0mm 💨3m/s 💦66%`)
- `--tooltip-style STRING` - tooltip detail level: `current-day`, `three-days`, or `week` (default: `current-day`)
  - `current-day`: Shows current conditions + next 12 hours
  - `three-days`: Shows current conditions + next 3 days (every 3 hours)
  - `week`: Shows current conditions + next 7 days (every 6 hours)
- `--lang LANG` - language for tooltip labels: `en` (English), `nb` (Norwegian Bokmål), `nn` (Norwegian Nynorsk), `sme` (Northern Sami), `fr` (French), `de` (German), `es` (Spanish) (default: `en`)
- `--temp-format STRING` - temperature format: `celsius` or `fahrenheit` (default: `celsius`)

### Examples
```bash
# Use IP-based geolocation with default settings
waybar-weather-info

# Oslo with detailed indicator
waybar-weather-info --location '59.911561,10.7492741' --indicator-style detailed

# Use coordinates with 3-day tooltip
waybar-weather-info --location '59.911561,10.7492741' --tooltip-style three-days

# Norwegian language with full indicator and week tooltip
waybar-weather-info --location '59.911561,10.7492741' --lang nb --indicator-style full --tooltip-style week

# Use Fahrenheit instead of Celsius
waybar-weather-info --location '59.911561,10.7492741' --temp-format fahrenheit
```


### Icons

To display the weather icons correctly, you will need to have a font that supports emojis installed.

## Waybar configuration

Assuming `waybar-weather-info` is in your path, add this to your waybar config.

**The module displays a weather indicator on the bar with a detailed tooltip on hover. Optionally, clicking can open the yr.no weather widget in your browser.**

### Recommended configuration

```json
"custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 3600,
    "exec": "waybar-weather-info --location '59.911561,10.7492741'",
    "return-type": "json"
}
```

### Configuration examples

**Simple configuration** (uses IP-based location):
```json
"custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 3600,
    "exec": "waybar-weather-info",
    "return-type": "json"
}
```

**With extended 3-day tooltip**:
```json
"custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 3600,
    "exec": "waybar-weather-info --location '59.911561,10.7492741' --tooltip-style three-days",
    "return-type": "json"
}
```

**With week-long tooltip**:
```json
"custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 3600,
    "exec": "waybar-weather-info --location '59.911561,10.7492741' --tooltip-style week",
    "return-type": "json"
}
```

**Norwegian language with full indicator**:
```json
"custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 3600,
    "exec": "waybar-weather-info --location '59.911561,10.7492741' --lang nb --indicator-style full",
    "return-type": "json"
}
```

**With detailed indicator and full week forecast**:
```json
"custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 3600,
    "exec": "waybar-weather-info --location '1-72837' --indicator-style detailed --tooltip-style week",
    "return-type": "json"
}
```

*Note: Weather data is cached for 15 min and location data (if not provided manually), is caced for 1 hour .*

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[allow(clippy::upper_case_acronyms)]
pub enum Lang {
    #[value(name = "en")]
    EN,
    #[value(name = "nb")]
    NB,
    #[value(name = "nn")]
    NN,
    #[value(name = "sme")]
    SME,
    #[value(name = "fr")]
    FR,
    #[value(name = "de")]
    DE,
    #[value(name = "es")]
    ES,
}

impl Lang {
    pub fn temperature(&self) -> &str {
        match self {
            Self::EN => "Temperature",
            Self::NB => "Temperatur",
            Self::NN => "Temperatur",
            Self::SME => "Temperatuvra",
            Self::FR => "Température",
            Self::DE => "Temperatur",
            Self::ES => "Temperatura",
        }
    }
    pub fn wind(&self) -> &str {
        match self {
            Self::EN => "Wind",
            Self::NB => "Vind",
            Self::NN => "Vind",
            Self::SME => "Biegga",
            Self::FR => "Vent",
            Self::DE => "Wind",
            Self::ES => "Viento",
        }
    }
    pub fn humidity(&self) -> &str {
        match self {
            Self::EN => "Humidity",
            Self::NB => "Luftfuktighet",
            Self::NN => "Luftfuktigheit",
            Self::SME => "Vuoigatvuohta",
            Self::FR => "Humidité",
            Self::DE => "Luftfeuchtigkeit",
            Self::ES => "Humedad",
        }
    }
    pub fn precipitation(&self) -> &str {
        match self {
            Self::EN => "Precipitation",
            Self::NB => "Nedbør",
            Self::NN => "Nedbør",
            Self::SME => "Šaddadeapmi",
            Self::FR => "Précipitations",
            Self::DE => "Niederschlag",
            Self::ES => "Precipitación",
        }
    }

    pub fn weather_desc(&self, symbol: &str) -> &str {
        match (self, symbol) {
            // Clear sky
            (Self::EN, s) if s.starts_with("clearsky") => "Clear sky",
            (Self::NB, s) if s.starts_with("clearsky") => "Klar himmel",
            (Self::NN, s) if s.starts_with("clearsky") => "Klar himmel",
            (Self::SME, s) if s.starts_with("clearsky") => "Čeaskat allahas",
            (Self::FR, s) if s.starts_with("clearsky") => "Ciel dégagé",
            (Self::DE, s) if s.starts_with("clearsky") => "Klarer Himmel",
            (Self::ES, s) if s.starts_with("clearsky") => "Cielo despejado",
            
            // Fair
            (Self::EN, s) if s.starts_with("fair") => "Fair",
            (Self::NB, s) if s.starts_with("fair") => "Lettskyet",
            (Self::NN, s) if s.starts_with("fair") => "Lettskya",
            (Self::SME, s) if s.starts_with("fair") => "Geaidnolaš",
            (Self::FR, s) if s.starts_with("fair") => "Beau",
            (Self::DE, s) if s.starts_with("fair") => "Heiter",
            (Self::ES, s) if s.starts_with("fair") => "Despejado",
            
            // Partly cloudy
            (Self::EN, s) if s.starts_with("partlycloudy") => "Partly cloudy",
            (Self::NB, s) if s.starts_with("partlycloudy") => "Delvis skyet",
            (Self::NN, s) if s.starts_with("partlycloudy") => "Delvis skya",
            (Self::SME, s) if s.starts_with("partlycloudy") => "Muhtun ládje pilvehagas",
            (Self::FR, s) if s.starts_with("partlycloudy") => "Partiellement nuageux",
            (Self::DE, s) if s.starts_with("partlycloudy") => "Teilweise bewölkt",
            (Self::ES, s) if s.starts_with("partlycloudy") => "Parcialmente nublado",
            
            // Cloudy
            (Self::EN, s) if s.starts_with("cloudy") => "Cloudy",
            (Self::NB, s) if s.starts_with("cloudy") => "Skyet",
            (Self::NN, s) if s.starts_with("cloudy") => "Skya",
            (Self::SME, s) if s.starts_with("cloudy") => "Pilvehagas",
            (Self::FR, s) if s.starts_with("cloudy") => "Nuageux",
            (Self::DE, s) if s.starts_with("cloudy") => "Bewölkt",
            (Self::ES, s) if s.starts_with("cloudy") => "Nublado",
            
            // Rain showers
            (Self::EN, s) if s.starts_with("lightrainshowers") => "Light rain showers",
            (Self::NB, s) if s.starts_with("lightrainshowers") => "Lette regnbyger",
            (Self::NN, s) if s.starts_with("lightrainshowers") => "Lette regnbyer",
            (Self::SME, s) if s.starts_with("lightrainshowers") => "Geahpes arvebuolus",
            (Self::FR, s) if s.starts_with("lightrainshowers") => "Averses légères",
            (Self::DE, s) if s.starts_with("lightrainshowers") => "Leichte Regenschauer",
            (Self::ES, s) if s.starts_with("lightrainshowers") => "Chubascos ligeros",
            
            (Self::EN, s) if s.starts_with("rainshowers") => "Rain showers",
            (Self::NB, s) if s.starts_with("rainshowers") => "Regnbyger",
            (Self::NN, s) if s.starts_with("rainshowers") => "Regnbyer",
            (Self::SME, s) if s.starts_with("rainshowers") => "Arvebuolus",
            (Self::FR, s) if s.starts_with("rainshowers") => "Averses",
            (Self::DE, s) if s.starts_with("rainshowers") => "Regenschauer",
            (Self::ES, s) if s.starts_with("rainshowers") => "Chubascos",
            
            (Self::EN, s) if s.starts_with("heavyrainshowers") => "Heavy rain showers",
            (Self::NB, s) if s.starts_with("heavyrainshowers") => "Kraftige regnbyger",
            (Self::NN, s) if s.starts_with("heavyrainshowers") => "Kraftige regnbyer",
            (Self::SME, s) if s.starts_with("heavyrainshowers") => "Garrasat arvebuolus",
            (Self::FR, s) if s.starts_with("heavyrainshowers") => "Fortes averses",
            (Self::DE, s) if s.starts_with("heavyrainshowers") => "Starke Regenschauer",
            (Self::ES, s) if s.starts_with("heavyrainshowers") => "Chubascos fuertes",
            
            // Rain
            (Self::EN, s) if s.starts_with("lightrain") => "Light rain",
            (Self::NB, s) if s.starts_with("lightrain") => "Lett regn",
            (Self::NN, s) if s.starts_with("lightrain") => "Lett regn",
            (Self::SME, s) if s.starts_with("lightrain") => "Geahpes arvi",
            (Self::FR, s) if s.starts_with("lightrain") => "Pluie légère",
            (Self::DE, s) if s.starts_with("lightrain") => "Leichter Regen",
            (Self::ES, s) if s.starts_with("lightrain") => "Lluvia ligera",
            
            (Self::EN, s) if s.starts_with("rain") => "Rain",
            (Self::NB, s) if s.starts_with("rain") => "Regn",
            (Self::NN, s) if s.starts_with("rain") => "Regn",
            (Self::SME, s) if s.starts_with("rain") => "Arvi",
            (Self::FR, s) if s.starts_with("rain") => "Pluie",
            (Self::DE, s) if s.starts_with("rain") => "Regen",
            (Self::ES, s) if s.starts_with("rain") => "Lluvia",
            
            (Self::EN, s) if s.starts_with("heavyrain") => "Heavy rain",
            (Self::NB, s) if s.starts_with("heavyrain") => "Kraftig regn",
            (Self::NN, s) if s.starts_with("heavyrain") => "Kraftig regn",
            (Self::SME, s) if s.starts_with("heavyrain") => "Garrasat arvi",
            (Self::FR, s) if s.starts_with("heavyrain") => "Forte pluie",
            (Self::DE, s) if s.starts_with("heavyrain") => "Starker Regen",
            (Self::ES, s) if s.starts_with("heavyrain") => "Lluvia fuerte",
            
            // Sleet
            (Self::EN, s) if s.starts_with("lightsleetshowers") => "Light sleet showers",
            (Self::NB, s) if s.starts_with("lightsleetshowers") => "Lette sluddbyger",
            (Self::NN, s) if s.starts_with("lightsleetshowers") => "Lette sluddbyer",
            (Self::SME, s) if s.starts_with("lightsleetshowers") => "Geahpes čievžabuolus",
            (Self::FR, s) if s.starts_with("lightsleetshowers") => "Averses légères de neige fondue",
            (Self::DE, s) if s.starts_with("lightsleetshowers") => "Leichte Schneeregenschauer",
            (Self::ES, s) if s.starts_with("lightsleetshowers") => "Chubascos ligeros de aguanieve",
            
            (Self::EN, s) if s.starts_with("sleetshowers") => "Sleet showers",
            (Self::NB, s) if s.starts_with("sleetshowers") => "Sluddbyger",
            (Self::NN, s) if s.starts_with("sleetshowers") => "Sluddbyer",
            (Self::SME, s) if s.starts_with("sleetshowers") => "Čievžabuolus",
            (Self::FR, s) if s.starts_with("sleetshowers") => "Averses de neige fondue",
            (Self::DE, s) if s.starts_with("sleetshowers") => "Schneeregenschauer",
            (Self::ES, s) if s.starts_with("sleetshowers") => "Chubascos de aguanieve",
            
            (Self::EN, s) if s.starts_with("heavysleetshowers") => "Heavy sleet showers",
            (Self::NB, s) if s.starts_with("heavysleetshowers") => "Kraftige sluddbyger",
            (Self::NN, s) if s.starts_with("heavysleetshowers") => "Kraftige sluddbyer",
            (Self::SME, s) if s.starts_with("heavysleetshowers") => "Garrasat čievžabuolus",
            (Self::FR, s) if s.starts_with("heavysleetshowers") => "Fortes averses de neige fondue",
            (Self::DE, s) if s.starts_with("heavysleetshowers") => "Starke Schneeregenschauer",
            (Self::ES, s) if s.starts_with("heavysleetshowers") => "Chubascos fuertes de aguanieve",
            
            (Self::EN, s) if s.starts_with("lightsleet") => "Light sleet",
            (Self::NB, s) if s.starts_with("lightsleet") => "Lett sludd",
            (Self::NN, s) if s.starts_with("lightsleet") => "Lett sludd",
            (Self::SME, s) if s.starts_with("lightsleet") => "Geahpes čievža",
            (Self::FR, s) if s.starts_with("lightsleet") => "Neige fondue légère",
            (Self::DE, s) if s.starts_with("lightsleet") => "Leichter Schneeregen",
            (Self::ES, s) if s.starts_with("lightsleet") => "Aguanieve ligera",
            
            (Self::EN, s) if s.starts_with("sleet") => "Sleet",
            (Self::NB, s) if s.starts_with("sleet") => "Sludd",
            (Self::NN, s) if s.starts_with("sleet") => "Sludd",
            (Self::SME, s) if s.starts_with("sleet") => "Čievža",
            (Self::FR, s) if s.starts_with("sleet") => "Neige fondue",
            (Self::DE, s) if s.starts_with("sleet") => "Schneeregen",
            (Self::ES, s) if s.starts_with("sleet") => "Aguanieve",
            
            (Self::EN, s) if s.starts_with("heavysleet") => "Heavy sleet",
            (Self::NB, s) if s.starts_with("heavysleet") => "Kraftig sludd",
            (Self::NN, s) if s.starts_with("heavysleet") => "Kraftig sludd",
            (Self::SME, s) if s.starts_with("heavysleet") => "Garrasat čievža",
            (Self::FR, s) if s.starts_with("heavysleet") => "Forte neige fondue",
            (Self::DE, s) if s.starts_with("heavysleet") => "Starker Schneeregen",
            (Self::ES, s) if s.starts_with("heavysleet") => "Aguanieve fuerte",
            
            // Snow
            (Self::EN, s) if s.starts_with("lightsnowshowers") => "Light snow showers",
            (Self::NB, s) if s.starts_with("lightsnowshowers") => "Lette snøbyger",
            (Self::NN, s) if s.starts_with("lightsnowshowers") => "Lette snøbyer",
            (Self::SME, s) if s.starts_with("lightsnowshowers") => "Geahpes muohttabuolus",
            (Self::FR, s) if s.starts_with("lightsnowshowers") => "Averses de neige légères",
            (Self::DE, s) if s.starts_with("lightsnowshowers") => "Leichte Schneeschauer",
            (Self::ES, s) if s.starts_with("lightsnowshowers") => "Chubascos de nieve ligeros",
            
            (Self::EN, s) if s.starts_with("snowshowers") => "Snow showers",
            (Self::NB, s) if s.starts_with("snowshowers") => "Snøbyger",
            (Self::NN, s) if s.starts_with("snowshowers") => "Snøbyer",
            (Self::SME, s) if s.starts_with("snowshowers") => "Muohttabuolus",
            (Self::FR, s) if s.starts_with("snowshowers") => "Averses de neige",
            (Self::DE, s) if s.starts_with("snowshowers") => "Schneeschauer",
            (Self::ES, s) if s.starts_with("snowshowers") => "Chubascos de nieve",
            
            (Self::EN, s) if s.starts_with("heavysnowshowers") => "Heavy snow showers",
            (Self::NB, s) if s.starts_with("heavysnowshowers") => "Kraftige snøbyger",
            (Self::NN, s) if s.starts_with("heavysnowshowers") => "Kraftige snøbyer",
            (Self::SME, s) if s.starts_with("heavysnowshowers") => "Garrasat muohttabuolus",
            (Self::FR, s) if s.starts_with("heavysnowshowers") => "Fortes averses de neige",
            (Self::DE, s) if s.starts_with("heavysnowshowers") => "Starke Schneeschauer",
            (Self::ES, s) if s.starts_with("heavysnowshowers") => "Chubascos de nieve fuertes",
            
            (Self::EN, s) if s.starts_with("lightsnow") => "Light snow",
            (Self::NB, s) if s.starts_with("lightsnow") => "Lett snø",
            (Self::NN, s) if s.starts_with("lightsnow") => "Lett snø",
            (Self::SME, s) if s.starts_with("lightsnow") => "Geahpes muohta",
            (Self::FR, s) if s.starts_with("lightsnow") => "Neige légère",
            (Self::DE, s) if s.starts_with("lightsnow") => "Leichter Schnee",
            (Self::ES, s) if s.starts_with("lightsnow") => "Nieve ligera",
            
            (Self::EN, s) if s.starts_with("snow") => "Snow",
            (Self::NB, s) if s.starts_with("snow") => "Snø",
            (Self::NN, s) if s.starts_with("snow") => "Snø",
            (Self::SME, s) if s.starts_with("snow") => "Muohta",
            (Self::FR, s) if s.starts_with("snow") => "Neige",
            (Self::DE, s) if s.starts_with("snow") => "Schnee",
            (Self::ES, s) if s.starts_with("snow") => "Nieve",
            
            (Self::EN, s) if s.starts_with("heavysnow") => "Heavy snow",
            (Self::NB, s) if s.starts_with("heavysnow") => "Kraftig snø",
            (Self::NN, s) if s.starts_with("heavysnow") => "Kraftig snø",
            (Self::SME, s) if s.starts_with("heavysnow") => "Garrasat muohta",
            (Self::FR, s) if s.starts_with("heavysnow") => "Forte neige",
            (Self::DE, s) if s.starts_with("heavysnow") => "Starker Schnee",
            (Self::ES, s) if s.starts_with("heavysnow") => "Nieve fuerte",
            
            // Fog
            (Self::EN, s) if s.starts_with("fog") => "Fog",
            (Self::NB, s) if s.starts_with("fog") => "Tåke",
            (Self::NN, s) if s.starts_with("fog") => "Tåke",
            (Self::SME, s) if s.starts_with("fog") => "Heahka",
            (Self::FR, s) if s.starts_with("fog") => "Brouillard",
            (Self::DE, s) if s.starts_with("fog") => "Nebel",
            (Self::ES, s) if s.starts_with("fog") => "Niebla",
            
            // Unknown
            _ => match self {
                Self::EN => "Unknown",
                Self::NB => "Ukjent",
                Self::NN => "Ukjend",
                Self::SME => "Amas",
                Self::FR => "Inconnu",
                Self::DE => "Unbekannt",
                Self::ES => "Desconocido",
            }
        }
    }
}

use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use log::{info, warn, debug, error};
use chrono::{DateTime, Utc};
// Enum for possible room locations
#[derive(Debug, Clone)]
enum Location {
    LivingRoom,
    Bedroom,
    Kitchen,
    Unknown,
}

impl Location {
    fn from_str(s: &str) -> Location {
        match s.to_lowercase().as_str() {
            "living room" | "livingroom" | "living" | "living_room" | "living rum" => {
                debug!("Matched location string '{}' to LivingRoom", s);
                Location::LivingRoom
            }
            "bedroom" | "bed" | "bed_room" | "bedrum" => {
                debug!("Matched location string '{}' to Bedroom", s);
                Location::Bedroom
            }
            "kitchen" => {
                debug!("Matched location string '{}' to Kitchen", s);
                Location::Kitchen
            }
            _ => {
                warn!("Unknown location string '{}'", s);
                Location::Unknown
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Location::LivingRoom => "Living Room".to_string(),
            Location::Bedroom => "Bedroom".to_string(),
            Location::Kitchen => "Kitchen".to_string(),
            Location::Unknown => "Unknown".to_string(),
        }
    }

    fn default_sensor_id(&self) -> &'static str {
        match self {
            Location::LivingRoom => "1",
            Location::Bedroom => "2",
            Location::Kitchen => "3",
            Location::Unknown => "0",
        }
    }
}

// Temperature reading response structure
#[derive(Serialize, Debug)]
struct TemperatureReading {
    Value: f32,
    Unit: String,
    Timestamp: DateTime<Utc>,
    Location: String,
    Status: String,
    SensorID: String,
    SensorType: String,
    Description: String
}

// Handler for the temperature endpoint
#[get("/temperature")]
async fn get_temperature(info: web::Query<HashMap<String, String>>) -> impl Responder {
    info!("Received temperature request with query params: {:?}", info);

    // Get location and sensor_id from query parameters
    let location_str = info.get("location").map(|s| s.as_str()).unwrap_or("");
    let sensor_id = info.get("sensorId").map(|s| s.as_str()).unwrap_or("");

    debug!("Parsed location_str: '{}', sensor_id: '{}'", location_str, sensor_id);

    // Determine location and sensor_id
    let (location, sensor_id) = if location_str.is_empty() && sensor_id.is_empty() {
        warn!("No location or sensor_id provided, defaulting to Unknown");
        (Location::Unknown, "0".to_string())
    } else if location_str.is_empty() {
        // If no location provided, determine from sensor_id
        let location = match sensor_id {
            "1" => Location::LivingRoom,
            "2" => Location::Bedroom,
            "3" => Location::Kitchen,
            _ => Location::Unknown,
        };
        debug!("Determined location from sensor_id {}: {:?}", sensor_id, location);
        (location, sensor_id.to_string())
    } else if sensor_id.is_empty() {
        // If no sensor_id provided, determine from location
        let location = Location::from_str(location_str);
        let default_sensor_id = location.default_sensor_id().to_string();
        debug!("Determined sensor_id from location {}: {}", location_str, default_sensor_id);
        (location.clone(), default_sensor_id)
    } else {
        // Both provided
        let location = Location::from_str(location_str);
        debug!("Both location and sensor_id provided: {}, {}", location_str, sensor_id);
        (location, sensor_id.to_string())
    };

    // Generate random temperature between 15.0 and 30.0 Celsius
    let mut rng = rand::thread_rng();
    let temperature: f32 = rng.gen_range(15.0..30.0);
    debug!("Generated random temperature: {:.2}°C", temperature);

    // Prepare response
    let response = TemperatureReading {
        Value: temperature,
        Unit: "°C".to_string(),
        Timestamp: Utc::now(),
        Location: location.to_string(),
        Status: "active".to_string(),
        SensorID: sensor_id,
        SensorType: "temperature".to_string(),
        Description: "comment".to_string()
    };

    info!("Returning temperature response: {:?}", response);
    web::Json(response)
}

// Handler for the temperature by ID endpoint
#[get("/temperature/{sensor_id}")]
async fn get_temperature_by_id(path: web::Path<String>) -> impl Responder {
    let sensor_id = path.into_inner();
    info!("Received temperature request for sensor ID: {}", sensor_id);

    // Determine location from sensor_id
    let location = match sensor_id.as_str() {
        "1" => Location::LivingRoom,
        "2" => Location::Bedroom,
        "3" => Location::Kitchen,
        _ => Location::Unknown,
    };

    // Generate random temperature between 15.0 and 30.0 Celsius
    let mut rng = rand::thread_rng();
    let temperature: f32 = rng.gen_range(15.0..30.0);
    debug!("Generated random temperature: {:.2}°C", temperature);

    // Prepare response
    let response = TemperatureReading {
        Value: temperature,
        Unit: "°C".to_string(),
        Timestamp: Utc::now(),
        Location: location.to_string(),
        Status: "active".to_string(),
        SensorID: sensor_id,
        SensorType: "temperature".to_string(),
        Description: "comment".to_string()
    };

    info!("Returning temperature response: {:?}", response);
    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .format_level(true)
        .format_module_path(false)
        .init();

    info!("Starting temperature sensor API server at http://0.0.0.0:8181");

    HttpServer::new(|| {
        App::new()
            .service(get_temperature)
            .service(get_temperature_by_id)
    })
    .bind("0.0.0.0:8181")
    .map_err(|e| {
        error!("Failed to bind to 0.0.0.0:8181: {}", e);
        e
    })?
    .run()
    .await
    .map_err(|e| {
        error!("Server error: {}", e);
        e
    })
}
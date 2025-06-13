use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

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
            "living room" | "livingroom" | "living" | "living_room" | "living rum" => Location::LivingRoom,
            "bedroom" | "bed" | "bed_room" | "bedrum"=> Location::Bedroom,
            "kitchen" => Location::Kitchen,
            _ => Location::Unknown,
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
#[derive(Serialize)]
struct TemperatureReading {
    location: String,
    sensor_id: String,
    temperature_c: f32,
    status: String,
}

// Handler for the temperature endpoint
#[get("/temperature")]
async fn get_temperature(info: web::Query<HashMap<String, String>>) -> impl Responder {
    // Get location and sensor_id from query parameters
    let location_str = info.get("location").map(|s| s.as_str()).unwrap_or("");
    let sensor_id = info.get("sensorId").map(|s| s.as_str()).unwrap_or("");

    // Determine location and sensor_id
    let (location, sensor_id) = if location_str.is_empty() && sensor_id.is_empty() {
        (Location::Unknown, "0".to_string())
    } else if location_str.is_empty() {
        // If no location provided, determine from sensor_id
        let location = match sensor_id {
            "1" => Location::LivingRoom,
            "2" => Location::Bedroom,
            "3" => Location::Kitchen,
            _ => Location::Unknown,
        };
        (location, sensor_id.to_string())
    } else if sensor_id.is_empty() {
        // If no sensor_id provided, determine from location
        let location = Location::from_str(location_str);
        (location.clone(), location.default_sensor_id().to_string())
    } else {
        // Both provided
        (Location::from_str(location_str), sensor_id.to_string())
    };

    // Generate random temperature between 15.0 and 30.0 Celsius
    let mut rng = rand::thread_rng();
    let temperature: f32 = rng.gen_range(15.0..30.0);

    // Prepare response
    let response = TemperatureReading {
        location: location.to_string(),
        sensor_id,
        temperature_c: temperature,
        status: "OK".to_string(),
    };

    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting temperature sensor API server at http://localhost:8081");

    HttpServer::new(|| {
        App::new()
            .service(get_temperature)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
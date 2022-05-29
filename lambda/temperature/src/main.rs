use lambda_runtime::{service_fn, LambdaEvent, Error};
use log::LevelFilter;
use serde_json::{json, Value};
use simple_logger::SimpleLogger;

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let tfm: f64 = ( c * 9.0/5.0) + 32.0;
    let round_number = (tfm * 100.0).round() / 100.0;
    return round_number;
}

fn celsius_to_kelvin(c: f64) -> f64 {
    let tfm: f64 = c + 273.15;
    let round_number: f64 = (tfm * 100.0).round() / 100.0;
    return round_number;
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let tfm: f64 = (f - 32.0) * (5.0/9.0);
    let round_number = (tfm * 100.0).round() / 100.0;
    return round_number;
}

fn fahrenheit_to_kelvin(f: f64) -> f64 {
    let tfm: f64 = (f - 32.0) * (5.0/9.0) + 273.15;
    let round_number: f64 = (tfm * 100.0).round() / 100.0;
    return round_number;
}

fn kelvin_to_celsius(k: f64) -> f64 {
    let tfm: f64 = k - 273.15;
    let round_number = (tfm * 100.0).round() / 100.0;
    return round_number;
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    let tfm: f64 = (k - 273.15) * (9.0/5.0) + 32.0;
    let round_number: f64 = (tfm * 100.0).round() / 100.0;
    return round_number;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  lambda_runtime::run(service_fn(func)).await?;
  Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
  let (event, _context) = event.into_parts();

  let body: Value = serde_json::from_str(event["body"].as_str().unwrap())?;
  let scale: &str = body["scale"].as_str().unwrap_or("celsius");
  let temperature: f64 = body["temperature"].as_str().unwrap_or("0").parse().expect("Not a number!");
  let transform: f64 = match scale{
    "from_celsius_to_fahrenheit"=>celsius_to_fahrenheit(temperature),
    "from_celsius_to_kelvin"=>celsius_to_kelvin(temperature),
    "from_fahrenheit_to_celsius"=>fahrenheit_to_celsius(temperature),
    "from_fahrenheit_to_kelvin"=>fahrenheit_to_kelvin(temperature),
    "from_kelvin_to_celsius"=>kelvin_to_celsius(temperature),
    "from_kelvin_to_fahrenheit"=>kelvin_to_fahrenheit(temperature),
    _=>celsius_to_fahrenheit(0.0)
  };
    
  let response = format!("Scale: {}, your old temperature is {} and your new temperature is {}", scale, temperature, transform);

  log::info!("{}", response);
  Ok(json!({ "response": response }))
}
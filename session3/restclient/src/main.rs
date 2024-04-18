use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=38.9517&longitude=-92.3341&current_weather=true";
    let response = reqwest::get(URL).await?;
    // println!("{:?}", response.text().await); // NOTE: incase text

    let weather: serde_json::Value = response.json().await?;
    println!("{weather:#?}");

    Ok(())
}
